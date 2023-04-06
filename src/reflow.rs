use textwrap::wrap;
use unicode_width::UnicodeWidthStr;

/// Determines if a line is a natural paragraph break.
///
/// # Arguments
///
/// * `line` - The current line of text as a string slice.
/// * `avg_line_length` - The average line length in the text as a floating-point number.
/// * `threshold_ratio` - The ratio (as a floating-point number) of the line width to the average line length
///                       that determines if the line is considered a natural paragraph break.
///
/// # Returns
///
/// * `bool` - Returns `true` if the line is considered a natural paragraph break, `false` otherwise.
fn is_natural_paragraph_break(line: &str, avg_line_length: f32, threshold_ratio: f32) -> bool {
    // Calculate the width of the line (without leading/trailing spaces).
    let line_width = UnicodeWidthStr::width(line.trim()) as f32;

    // Check if the line width is less than the threshold ratio of the average line length.
    let is_short_line = line_width / avg_line_length < threshold_ratio;

    // A comprehensive list of punctuation marks that can end a paragraph.
    let sentence_ending_punctuations = [
        '.', '܂', '。',// periods
        '։', ':', '፡', '：', // colons
        '!', '！',  // exclamation marks
        '?', '？', // question marks
        '”', '᠉',// end quotes
        '᠃', '᠁', '…',// ellipsis
        ';', '；', '؟', '।', '۔', '܀', '܁', '።', '፧', '፨', '᙮', 'ክ', 'ዼ',
    ];

    // Check if the line ends with any of the sentence-ending punctuation marks.
    let ends_with_punctuation = sentence_ending_punctuations.iter().any(|&p| line.trim().ends_with(p));

    // Return true if the line is short and ends with a sentence-ending punctuation mark.
    is_short_line && ends_with_punctuation
}

fn calculate_average_line_length(text: &str) -> f32 {
    let lines: Vec<&str> = text.lines().collect();
    let total_length: usize = lines.iter().map(|line| UnicodeWidthStr::width(line.trim())).sum();
    total_length as f32 / lines.len() as f32
}

pub struct ReflowOptions {
    pub threshold_ratio: f32,
    pub para_chars_limit: usize,
}

impl Default for ReflowOptions {
    fn default() -> Self {
        ReflowOptions {
            threshold_ratio: 0.9,
            para_chars_limit: usize::MAX,
        }
    }
}

/// Reflow the given text to minimize artificial line breaks and break paragraphs based on word limits.
///
/// # Arguments
/// * `text` - The input text containing lines that may have artificial line breaks.
/// * `options` - The reflow options containing threshold_ratio and word_limit.
///
/// # Returns
/// * A `String` containing the reflowed text with artificial line breaks minimized and paragraphs split according to word limit.
pub fn reflow_text(text: &str, options: Option<ReflowOptions>) -> String {
    // If the user provides options, use them; otherwise, use default values.
    let options = options.unwrap_or(ReflowOptions::default());

    // Split the input text into lines.
    let lines: Vec<&str> = text.lines().collect();

    // Calculate the average line length.
    let avg_line_length = lines.iter().map(|line| line.len()).sum::<usize>().checked_div(lines.len()).unwrap_or(0);

    // Initialize a buffer to store the modified lines.
    let mut buffer = String::new();

    // Iterate through the input lines.
    for (index, line) in lines.iter().enumerate() {
        // Check if the current line is a natural paragraph break (shorter than the threshold).
        if is_natural_paragraph_break(line, avg_line_length as f32, options.threshold_ratio) {
            // If the buffer is not empty, add it to the result with a newline.
            if !buffer.is_empty() {
                buffer.push_str(line);
                buffer.push('\n');
            } else {
                // If the buffer is empty, just add the current line to the result with a newline.
                buffer.push_str(line);
                buffer.push('\n');
            }
        } else {
            // If the line is not a natural paragraph break, add it to the buffer.
            buffer.push_str(line);

            // If it's not the last line, add a space to the buffer.
            if index != lines.len() - 1 {
                buffer.push(' ');
            }
        }
    }

    // break the buffers into lines and wrap paragraphs that are longer than the word limit.
    let mut result = String::new();
    for paragraph in buffer.lines() {
        let char_count = paragraph.chars().count();

        // if the number of chars is less than the word limit, just add the paragraph to the result
        if char_count <= options.para_chars_limit {
            result.push_str(paragraph);
            result.push('\n');
            continue;
        } else {

            // otherwise, wrap the paragraph and add it to the result
            let wrapped_paragraph = wrap(paragraph, options.para_chars_limit);
            for line in wrapped_paragraph {
                result.push_str(&line.to_string());
                result.push('\n');
            }
        }
    };

    result.pop();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflow_text() {
        let input = "This is a test of the reflow text function. This text should be \
        broken into multiple lines if the word limit is set to a small \
        value. This line is intentionally short.";

        let expected_output_no_options = "This is a test of the reflow text function. This text should be broken into multiple lines if the word limit is set to a small value. \
        This line is intentionally short.";

        let expected_output_with_options = ["This is", "a test of", "the reflow", "text", "function.", "This text", "should", "be broken", "into", "multiple", "lines if", "the word", "limit is", "set to", "a small", "value.", "This", "line is", "intentiona", "lly short."];

        // Test with no options
        let output = reflow_text(input, None);
        assert_eq!(output, expected_output_no_options);

        // Test with options
        let options = ReflowOptions {
            threshold_ratio: 0.9,
            para_chars_limit: 10,
        };
        let output = reflow_text(input, Some(options));
        let output_lines: Vec<&str> = output.lines().collect();
        assert_eq!(output_lines, expected_output_with_options)
    }
}