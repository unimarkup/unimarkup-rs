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

function getMode() {
    return citeproc.opt.xclass;
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
        if (getMode() === "in-text") {
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
                footnoteResults.push(`<div id=\"footnote-${counter}/">[${counter}] ${lastResult[1]}</div>`);
                for (let i = 0; i < newStrings.length - 1; ++i) {
                    let index = newStrings[i][0];
                    footnoteResults[index] = `<div id=\"footnote-${index + 1}/">[${index + 1}] ${newStrings[i][1]}</div>`;
                }
            }
        }
        counter += 1;
    }
    for (let result of footnoteResults) {
        console.log(result);
    }
    return citationResults;
}
