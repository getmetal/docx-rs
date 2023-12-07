use crate::documents::elements::paragraph::Paragraph;
use crate::documents::settings::Settings;

pub fn estimate_page_number(paragraph: &Paragraph, settings: &Settings) -> u32 {
    // Calculate the amount of space the Paragraph takes up on the page
    let paragraph_space = calculate_paragraph_space(paragraph);

    // Calculate the total amount of space available on a page
    let page_space = calculate_page_space(settings);

    // Estimate the page number by dividing the paragraph space by the page space
    let page_number = paragraph_space / page_space;

    page_number
}

fn calculate_paragraph_space(paragraph: &Paragraph) -> u32 {
    // This is a placeholder calculation. The actual calculation would be more complex and take into account
    // factors like the paragraph's font size, line spacing, etc.
    paragraph.raw_text().len() as u32
}

fn calculate_page_space(settings: &Settings) -> u32 {
    // This is a placeholder calculation. The actual calculation would be more complex and take into account
    // factors like the document's margins, font size, line spacing, etc.
    settings.default_tab_stop.value() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_page_number() {
        let paragraph = Paragraph::new().add_run(Run::new().add_text("Hello, world!"));
        let settings = Settings::new().default_tab_stop(10);

        let page_number = estimate_page_number(&paragraph, &settings);

        assert_eq!(page_number, 1);
    }

    // Additional tests would go here, covering different paragraph lengths, different document settings, and edge cases
}
