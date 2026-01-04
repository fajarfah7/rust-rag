use rag::infrastructure::pdfium::loader::PdfLoader;

#[tokio::main]
async fn main() {
    let pdf_loader = match PdfLoader::new() {
        Ok(pl) => pl,
        Err(e) => {
            tracing::error!(error = ?e, "failed init pdf loader");
            return;
        }
    };

    let source_pdf = "./tmp/test.pdf";
    let doc = match pdf_loader.load(source_pdf) {
        Ok(d) => d,
        Err(e) => {
            tracing::error!(error = ?e, "failed load file");
            return;
        }
    };

    for (idx, page) in doc.pages().iter().enumerate() {
        if idx > 8 {
            println!("Page size: {} x {}", page.width(), page.height());

            let text = match page.text() {
                Ok(t) => t.all(),
                Err(e) => {
                    tracing::error!(error = ?e, "found error in a page");
                    return;
                }
            };

            // for text_rect in text_page.text_rects() {
            //     let bounds = text_rect.bounds();
            //     let text = text_page.text_in_rect(&bounds)?;

            //     if !text.is_empty() {
            //         // You can perform further analysis here to group related text_rects into paragraphs
            //         println!("Bounds: {:?}", bounds);
            //         println!("Text: \'{}\'", text);
            //     }
            // }
        }
        if idx > 9 {
            return;
        }
    }
}
