#[cfg(test)]
use crate::*;

#[test]
fn hello_pdf() {
    let html = "<h1>Hello from a PDF!</h1>";
    let height = 100;
    let width = 1080;
    let settings = Settings::PDF(Some(PDFOptions::default()));
    let path = "./target/samples/hello.pdf";
    print_file(html, width, height, settings, path).unwrap();
}

#[test]
fn hello_jpeg() {
    let html = "<h1>Hello from a JPEG!</h1>";
    let height = 100;
    let width = 1080;
    let settings = Settings::JPEG;
    let path = "./target/samples/hello.jpeg";
    print_file(html, width, height, settings, path).unwrap();
}

#[test]
fn hello_png() {
    let html = "<h1>Hello from a PNG!</h1>";
    let height = 100;
    let width = 1080;
    let settings = Settings::PNG;
    let path = "./target/samples/hello.png";
    print_file(html, width, height, settings, path).unwrap();
}

#[test]
fn hello_webp() {
    let html = "<h1>Hello from a WebP!</h1>";
    let height = 100;
    let width = 1080;
    let settings = Settings::WebP;
    let path = "./target/samples/hello.webp";
    print_file(html, width, height, settings, path).unwrap();
}
