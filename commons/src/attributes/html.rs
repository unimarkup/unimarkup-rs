use crate::lexer::position::Position;

use super::{AttributeValue, NestedAttribute};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HtmlAttribute {
    Property(HtmlProperty),
    Nested(NestedAttribute<HtmlAttribute>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HtmlProperty {
    ident: HtmlPropertyId,
    value: AttributeValue,
    start: Position,
    end: Position,
}

macro_rules! html_attributes {
    ($($name:literal -> $id:ident $(: $info:literal)?);*) => {

        /// HTML attributes that are supported as Unimarkup attributes.
        /// Taken from: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes?retiredLocale=de
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum HtmlPropertyId {
            $(
                #[doc=concat!("The `", $name, "` attribute.")]
                $(
                    #[doc=concat!("See: ", $info)]
                )?
                $id,

            )*
            /// Attribute starting with `data-`.
            Custom(Box<super::AttributeIdent>),
        }

        impl TryFrom<(&str, Position, Position)> for HtmlPropertyId {
            type Error = super::log_id::AttributeError;

            /// Tries to convert a given `str` to a [`HtmlPropertyId`].
            /// The positions are the `start` and `end` of the given `str`.
            ///
            /// Usage: `HtmlPropertyId::try_from("autoplay", <start pos>, <end pos>)`
            fn try_from(value: (&str, Position, Position)) -> Result<Self, Self::Error> {
                match value.0.to_lowercase().as_str() {
                    $(
                        $name => Ok(HtmlPropertyId::$id),
                    )*
                    s if s.starts_with("data-") => Ok(HtmlPropertyId::Custom(Box::new(super::AttributeIdent::from(value)))),
                    _ => Err(super::log_id::AttributeError::InvalidHtmlIdent),
                }
            }
        }

        impl HtmlPropertyId {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        HtmlPropertyId::$id => $name,
                    )*
                    HtmlPropertyId::Custom(c) => &c.ident,
                }
            }

            #[allow(clippy::len_without_is_empty)]
            pub fn len(&self) -> usize {
                match self {
                    $(
                        HtmlPropertyId::$id => $name.len(),
                    )*
                    HtmlPropertyId::Custom(c) => c.ident.len(),
                }
            }
        }
    }
}

html_attributes!(
    "accept" -> Accept: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/accept";
    "accept-charset" -> AcceptCharset: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#accept-charset";
    "accesskey" -> AccessKey: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/accesskey";
    "action" -> Action: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#action";
    "allow" -> Allow: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#allow";
    "async" -> AsyncAttrb: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script#async";
    "autocapitalize" -> AutoCapitalize: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/autocapitalize";
    "autocomplete" -> AutoComplete: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete";
    "autofocus" -> AutoFocus: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/autofocus";
    "autoplay" -> AutoPlay: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video#autoplay";
    "capture" -> Capture: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/capture";
    "checked" -> Checked: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#checked";
    "cite" -> Cite: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote#cite";
    "class" -> Class: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class";
    "cols" -> Cols: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea#cols";
    // "colspan" excluded, because it is only for tables
    // "content" excluded, because it is only for `meta` HTML elements
    "contenteditable" -> ContentEditable: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable";
    "controls" -> Controls: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video#controls";
    "coords" -> Coords: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#coords";
    "crossorigin" -> CrossOrigin: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/crossorigin";
    "datetime" -> DateTime: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del#datetime";
    "decoding" -> Decoding: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#decoding";
    "default" -> Default: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track#default";
    "defer" -> Defer: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script#defer";
    "dir" -> Dir: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/dir";
    "dirname" -> Dirname: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/dirname";
    "disabled" -> Disabled: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/disabled";
    "download" -> Download: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#download";
    "draggable" -> Draggable: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/draggable";
    "elementtiming" -> ElementTiming: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/elementtiming";
    "enctype" -> Enctype: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#enctype";
    "enterkeyhint" -> EnterKeyHint: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/enterkeyhint";
    "exportparts" -> ExportParts: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/exportparts";
    "for" -> ForAttrb: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/for";
    "form" -> Form: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#form";
    "formaction" -> FormAction: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#formaction";
    "formenctype" -> FormEncType: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#formenctype";
    "formmethod" -> FormMethod: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#formmethod";
    "formnovalidate" -> FormNoValidate: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#formnovalidate";
    "formtarget" -> FormTarget: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#formtarget";
    // "headers" excluded, because it is only for tables
    "hidden" -> Hidden: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/hidden";
    "high" -> High: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter#high";
    // "href" excluded, because it is implicit in Unimarkup elements
    "hreflang" -> HrefLang: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#hreflang";
    // "http-equiv" excluded, because it is only for `meta` HTML elements
    "id" -> Id: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id";
    "inert" -> Inert: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/inert";
    "inputmode" -> InputMode: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/inputmode";
    "integrity" -> Integrity: "https://developer.mozilla.org/en-US/docs/Web/Security/Subresource_Integrity";
    "is" -> Is: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/is";
    "ismap" -> IsMap: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#ismap";
    "itemid" -> ItemId: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemid";
    "itemprop" -> ItemProp: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemprop";
    "itemref" -> ItemRef: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemref";
    "itemscope" -> ItemScope: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemscope";
    "itemtype" -> ItemType: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemtype";
    "kind" -> Kind: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track#kind";
    "label" -> Label: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option#label";
    "lang" -> Lang: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/lang";
    "list" -> List: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#list";
    "loading" -> Loading: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#loading";
    "loop" -> Loop: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video#loop";
    "low" -> Low: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter#low";
    "max" -> Max: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/max";
    "maxlength" -> MaxLength: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/maxlength";
    "minlength" -> MinLength: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/minlength";
    "media" -> Media: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source#media";
    "method" -> Method: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#method";
    "min" -> Min: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/min";
    "multiple" -> Multiple: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/multiple";
    "muted" -> Muted: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video#muted";
    "name" -> Name: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#name";
    "nonce" -> Nonce: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/nonce";
    "novalidate" -> NoValidate: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#novalidate";
    "open" -> Open: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details#open";
    "optimum" -> Optimum: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter#optimum";
    "part" -> Part: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/part";
    "pattern" -> Pattern: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/pattern";
    "ping" -> Ping: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#ping";
    // "placeholder" excluded, because it is implicit in Unimarkup elements
    "playsinline" -> PlaysInline: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video#playsinline";
    "popover" -> PopOver: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/popover";
    "popovertarget" -> PopOverTarget: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#popovertarget";
    "popovertargetaction" -> PopOverTargetAction: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#popovertargetaction";
    "poster" -> Poster: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video#poster";
    "preload" -> Preload: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video#preload";
    "readonly" -> Readonly: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/readonly";
    "referrerpolicy" -> ReferrerPolicy: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#referrerpolicy";
    "rel" -> Rel: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rel";
    "required" -> Required: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/required";
    "reversed" -> Reversed: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol#reversed";
    "role" -> Role: "https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles";
    "rows" -> Rows: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea#rows";
    // "rowspan" excluded, because it is only for tables
    "sandbox" -> Sandbox: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#sandbox";
    // "scope" excluded, because it is only for tables
    "selected" -> Selected: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option#selected";
    "shape" -> Shape: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#shape";
    "size" -> Size: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/size";
    "sizes" -> Sizes: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#sizes";
    "slot" -> Slot: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/slot";
    // "span" excluded, because it is only for tables
    "spellcheck" -> SpellCheck: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/spellcheck";
    // "src" excluded, because it is implicit in Unimarkup elements
    // "srcdoc" excluded, because it is implicit in Unimarkup elements
    "srclang" -> SrcLang: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track#srclang";
    // "srcset" excluded, because it is implicit in Unimarkup elements
    // "start" excluded, because it is implicit in Unimarkup elements
    "step" -> Step: "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/step";
    "tabindex" -> TabIndex: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/tabindex";
    "target" -> Target: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#target";
    "title" -> Title: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/title";
    "translate" -> Translate: "https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/translate";
    "type" -> TypeAttrb: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#input_types";
    "usemap" -> UseMap: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#usemap";
    "value" -> Value: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#value";
    "wrap" -> Wrap: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea#wrap"
);
