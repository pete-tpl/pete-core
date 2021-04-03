use crate::expressions;

/// Result of parse_expression_string function
pub struct ParseExpressionStringResult {
    /// A parser expression string
    pub expression_string: String,
    /// end offset of the tag
    pub end_offset: usize,
    /// true if expression has a no linebreak character at the end
    pub has_nolinebreak_end: bool,
}

/// Parses an expression from non-parsed template
/// 
/// Returns:
/// - an Expression string
/// - end offset of the tag
/// - if expression has a no linebreak character at the end
/// 
/// # Examples
/// 
/// ```
/// use pete_core::parsers::expression_parser::parse_expression_string;
/// 
/// let input = String::from(" 2+3 %}A{%endif%}");
/// let result = parse_expression_string(&input, 5, "%}").unwrap();
/// assert_eq!(String::from(" 2+3"), result.expression_string);
/// assert_eq!(11, result.end_offset);
/// assert_eq!(false, result.has_nolinebreak_end);
/// 
/// let input = String::from(" 2+3 -%}A{%endif%}");
/// let result = parse_expression_string(&input, 5, "%}").unwrap();
/// assert_eq!(String::from(" 2+3"), result.expression_string);
/// assert_eq!(12, result.end_offset);
/// assert_eq!(true, result.has_nolinebreak_end);
/// ```
pub fn parse_expression_string(template_string: &String, offset_shift: usize, stop_sequence: &str) -> Result<ParseExpressionStringResult, String> {
    let tag_end_pos_rel = match expressions::get_end_offset(template_string, stop_sequence) {
        Some(end_pos) => end_pos,
        None => {
            return Err(String::from("Cannot find closing tag."));
        }
    };
    let tag_end_pos_abs = tag_end_pos_rel + offset_shift;
    let (expr_end_pos, has_nolinebreak_end) = {
        let end_pos = tag_end_pos_rel - stop_sequence.len();
        let str_before_end_tag = template_string[..tag_end_pos_rel - stop_sequence.len() + 1].to_string();
        match str_before_end_tag.ends_with('-') {
            true => (end_pos - 1, true),
            false => (end_pos, false),
        }
    };
    let expression_string = template_string[..expr_end_pos].to_string();
    Ok(ParseExpressionStringResult{
        expression_string,
        end_offset: tag_end_pos_abs,
        has_nolinebreak_end,
    })
}