#[cfg(test)]
use crate::print_file;
#[cfg(test)]
use crate::Settings;
#[cfg(test)]
use headless_chrome::types::PrintToPdfOptions;

#[test]
fn hello_pdf() {
    let html = "<h1>Hello from a PDF!</h1>";
    let height = 20;
    let width = 50;
    let settings = Settings::PDF(Some(PrintToPdfOptions::default()));
    let path = "./target/samples/hello.pdf";
    print_file(html, width, height, settings, path).unwrap();
}
