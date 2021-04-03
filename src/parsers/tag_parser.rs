pub const TAG_START: &str = "{%";
pub const TAG_END: &str = "%}";

/// Result of get_keyword function invocation
pub struct GetKeywordResult<'a> {
    /// TRUE if the parsed tag has nolinebreak char in beginning
    pub has_nolinebreak_beginning: bool,
    /// a keyword (if, elseif, etc)
    pub keyword: &'a str,
    /// a remain AFTER keyword
    pub remain: String,
}

/// Returns:
/// - a keyword (if, elseif, etc)
/// - a remain AFTER keyword
/// - if the parsed tag has nolinebreak char
/// 
/// # Examples
/// 
/// ```
/// use pete_core::parsers::tag_parser;
/// 
/// assert_eq!(tag_parser::get_keyword(&String::from("Hello")).is_none(), true);
/// 
/// let sample = String::from("{% if 1 + 1 %}");
/// let result = tag_parser::get_keyword(&sample).unwrap();
/// assert_eq!(result.has_nolinebreak_beginning, false);
/// assert_eq!(result.keyword, "if");
/// assert_eq!(result.remain, String::from(" 1 + 1 %}"));
/// 
/// let sample = String::from("{%- if 1 + 1 %}");
/// let result = tag_parser::get_keyword(&sample).unwrap();
/// assert_eq!(result.has_nolinebreak_beginning, true);
/// assert_eq!(result.keyword, "if");
/// assert_eq!(result.remain, String::from(" 1 + 1 %}"));
/// ```
pub fn get_keyword(string: &String) -> Option<GetKeywordResult> {
    let s = match string.strip_prefix(TAG_START) {
        Some(m) => m,
        None => return None,
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
    Some(GetKeywordResult{
        has_nolinebreak_beginning,
        keyword: &s[..endpos],
        remain: String::from(&s[endpos..]),
    })
}