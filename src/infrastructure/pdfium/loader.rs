use pdfium_render::prelude::*;

use crate::error::pdf_parser::PdfParserError;

#[derive(Debug)]
pub struct PdfLoader {
    pdfium: Pdfium
}

// use struct and impl to make pdfium live same as PdfLoader
// because in implementation implement these scripts:
// let loader = PdfLoader::new() <- inside PdfLoader there is pdfium, and PdfLoader alive as long as fn alive
// let document = loader.load("some/path/to/document") <- this document alive as long as loader alive
impl PdfLoader {
    pub fn new() -> Result<Self, PdfParserError> {
        let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name())
        .map_err(|e| return PdfParserError::UnknownError(e.to_string()))?;

        Ok(Self { pdfium: Pdfium::new(bindings) })
    }
    
    // 'a lifetime force pdfium alive as long as Self/PdfLoader alive
    pub fn load<'a>(
        &'a self,
        path: &str,
    ) -> Result<PdfDocument<'a>, PdfParserError> {
        self.pdfium.load_pdf_from_file(path, None)
        .map_err(|e| PdfParserError::FileNotFound(e.to_string()))
    }
}
