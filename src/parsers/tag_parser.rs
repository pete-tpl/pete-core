pub const TAG_START: &str = "{%";
pub const TAG_END: &str = "%}";

/// Returns:
/// - a keyword (if, elseif, etc)
/// - a remain AFTER keyword
/// - if the parsed tag has nolinebreak char
/// 
/// # Examples
/// 
/// ```
/// use pete_core::parsers::tag_parser;
/// assert_eq!(tag_parser::get_keyword(&String::from("{% if 1 + 1 %}")), ("if", String::from(" 1 + 1 %}"), false));
/// assert_eq!(tag_parser::get_keyword(&String::from("{%- if 1 + 1 %}")), ("if", String::from(" 1 + 1 %}"), true));
/// ```
pub fn get_keyword(string: &String) -> (&str, String, bool) {
    let s = match string.strip_prefix(TAG_START) {
        Some(m) => m,
        None => return ("", string.clone(), false),
    };
    let (s, has_nolinebreak_beginning) = match s.strip_prefix('-') {
        Some(m) => (m, true),
        None => (s, false),
    };
    let s =  s.trim_start_matches(' ');
    let endpos = match s.find(|c| !char::is_alphabetic(c)) {
        Some(p) => p,
        None => s.len() - 1,
    };
    (&s[..endpos], String::from(&s[endpos..]), has_nolinebreak_beginning)
}