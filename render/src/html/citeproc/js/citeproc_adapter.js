import CSL from "./citeproc-js/citeproc_commonjs.js";

let citations;
let citeproc;
let citationsPre = [];
const citationsPost = [];
let citationResults = [];
let footnoteResults = [];

function getCitations(citationText) {
    const citationData = JSON.parse(citationText);

    let citations = {};
    let itemIDs = [];
    for (let i = 0, ilen = citationData.items.length; i < ilen; i++) {
        let item = citationData.items[i];
        if (!item.issued) continue;
        if (item.URL) delete item.URL;
        let id = item.id;
        citations[id] = item;
        itemIDs.push(id);
    }
    return citations;
}

function getCiteprocSys(localeText) {
    return {
        retrieveLocale: function(lang) {
            return localeText;
        },
        retrieveItem: function(id){
            return citations[id];
        }
    };
}

function getProcessor(citeprocSys, styleAsText) {
    return new CSL.Engine(citeprocSys, styleAsText);
}

function appendCitation(noteIndex, ids) {
    let citationID = "";
    let citationItems = [];
    for (let item of ids) {
        citationID += item;
        citationID += ";";
        citationItems.push({id: item});
    }
    citationID += noteIndex;
    const citation = {
        properties: {
            noteIndex: noteIndex
        },
        citationItems: citationItems,
        citationID: citationID
    };
    const citationString = citeproc.processCitationCluster(citation, citationsPre, citationsPost);

    citationsPre.push([citationString[1][citationString[1].length - 1][2], citationsPre.length + 1]);
    const result = citationString[1];
    for (const ar of result) {
        ar[1] = splitCitationEntry(ar[1], ar[2]);
    }
    return citationString[1];
}

function splitCitationEntry(inputString, ids) {
    let splitStrings = inputString.split(String.fromCharCode(31));
    let splitIds = ids.split(";");
    let resultString = []
    for (let i = 0; i < splitStrings.length; ++i) {
        resultString.push(`<a href=\"#${splitIds[i]}\" style=\"color: inherit; text-decoration: none\">`);
        resultString.push(splitStrings[i]);
        resultString.push("</a>");
    }
    return resultString.join("");
}

function getBibliographyStyles(bibStyles) {
    let stylesString = [];
    stylesString.push("<style scoped>");
    stylesString.push(".csl-entry { ");
    if (bibStyles["entryspacing"] === 0) {
        stylesString.push("padding-bottom: 0.1em;");
    } else {
        stylesString.push(`padding-bottom: ${bibStyles["entryspacing"]}em;`);
    }
    stylesString.push(`line-height: ${bibStyles["linespacing"]}em;`);
    if (bibStyles["hangingindent"] !== undefined) {
        stylesString.push("padding-left: 1.3em;");
        stylesString.push("text-indent: -1.3em;");
    }
    stylesString.push("}");
    console.log(bibStyles["second-field-align"]);
    if (bibStyles["second-field-align"]) {
        if (bibStyles["second-field-align"] === "flush") {
            stylesString.push(".csl-left-margin { ");
            stylesString.push("position: absolute;");
            stylesString.push("}");
            stylesString.push(".csl-right-inline { ");
            stylesString.push(`margin-left: ${bibStyles["maxoffset"]}ch`);
            stylesString.push("}");
        } else {
            stylesString.push(".csl-left-margin { ");
            stylesString.push("text-align: right;");
            stylesString.push("position: absolute;");
            stylesString.push("transform-origin: top right;");
            stylesString.push("transform: translate(-100%);");
            stylesString.push("margin-left: -1ch;");
            stylesString.push("}");
        }
    }
    stylesString.push("</style>");
    return stylesString.join("\n");
}

function getFootnoteStyles() {
    let stylesString = [];
    stylesString.push("<style scoped>");
    stylesString.push(".footnote-left-margin { ");
    stylesString.push("text-align: right;");
    stylesString.push("position: absolute;");
    stylesString.push("transform-origin: top right;");
    stylesString.push("transform: translate(-100%);");
    stylesString.push("margin-left: -1ch;");
    stylesString.push("}");
    stylesString.push("</style>");
    return stylesString.join("\n");
}

export function initProcessor(citationText, localeText, styleText) {
    citations = getCitations(citationText);
    let citeprocSys = getCiteprocSys(localeText);
    citeproc = getProcessor(citeprocSys, styleText);
}

export function getCitationStrings(citationIds, for_pagedjs) {
    let counter = 1;
    for (let citationId of JSON.parse(citationIds)) {
        let newStrings = appendCitation(counter, citationId);
        let lastResult = newStrings[newStrings.length - 1];
        if (citeproc.opt.xclass === "in-text") {
            citationResults.push(lastResult[1]);
            for (let i = 0; i < newStrings.length - 1; ++i) {
                citationResults[newStrings[i][0]] = newStrings[i][1];
            }
        } else {
            if (for_pagedjs) {
                citationResults.push(`<span className=\"footnote\">${lastResult[1]}</span>`);
                for (let i = 0; i < newStrings.length - 1; ++i) {
                    citationResults[newStrings[i][0]] = `<span className=\"footnote\">${newStrings[i][1]}</span>`;
                }
            } else {
                citationResults.push(`<a href=\"#footnote-${counter}/" style=\"color: inherit; text-decoration: none\">[${counter}]</a>`);
                footnoteResults.push(`<div id=\"footnote-${counter}/"><div class="footnote-left-margin">[${counter}]</div> <div class="footnote-right-inline">${lastResult[1]}</div></div>`)
                for (let i = 0; i < newStrings.length - 1; ++i) {
                    let index = newStrings[i][0];
                    footnoteResults[index] = `<div id=\"footnote-${index + 1}/"><div class="footnote-left-margin">[${index + 1}]</div> <div class="footnote-right-inline">${newStrings[i][1]}</div></div>`;
                }
            }
        }
        counter += 1;
    }
    return citationResults;
}

export function hasFootnotes() {
    return footnoteResults.length > 0;
}
export function getFootnoteString() {
    let joinedResults = footnoteResults.join("");
    let footnoteString = [];
    footnoteString.push(
        "<div style='margin-left: 4ch'>",
        getFootnoteStyles());
    footnoteString.push(joinedResults);
    footnoteString.push("</div>");
    return footnoteString.join("");
}

export function getBibliography() {
    let bib = citeproc.makeBibliography();
    let resultString = [];
    let ids = bib[0]["entry_ids"];
    resultString.push(bib[0]["bibstart"]);
    resultString.push(getBibliographyStyles(bib[0]));
    for (let i = 0; i < bib[1].length; ++i) {
        resultString.push(
            `<div id="${ids[i]}">`,
            bib[1][i],
            "</div>"
        );
    }
    resultString.push(bib[0]["bibend"]);
    return resultString.join("");
}
