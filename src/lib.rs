use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser, LaunchOptions};

mod tests;

pub type PDFOptions = headless_chrome::types::PrintToPdfOptions;

pub enum Settings {
    PDF(Option<PDFOptions>),
    JPEG,
    PNG,
    WebP,
}

pub fn print(html: &str, width: u32, height: u32, settings: Settings) -> Result<Vec<u8>, String> {
    let launch_options = LaunchOptions {
        headless: true,
        sandbox: true,
        window_size: Some((width, height)),
        port: None,
        ignore_certificate_errors: false,
        path: None,
        user_data_dir: None,
        extensions: Vec::new(),
        args: Vec::new(),
        disable_default_args: false,
        idle_browser_timeout: std::time::Duration::from_secs(60),
        process_envs: None,
        proxy_server: None,
    };

    let browser = match Browser::new(launch_options) {
        Ok(browser) => browser,
        Err(err) => return Result::Err(err.to_string()),
    };

    let tab = match browser.wait_for_initial_tab() {
        Ok(tab) => tab,
        Err(err) => return Result::Err(err.to_string()),
    };

    let url = &format!("data:text/html,{}", html);

    if let Err(err) = tab.navigate_to(&url) {
        return Result::Err(err.to_string());
    }

    if let Err(err) = tab.wait_until_navigated() {
        return Result::Err(err.to_string());
    }

    let raw = match settings {
        Settings::PDF(options) => match tab.print_to_pdf(options) {
            Ok(pdf) => pdf,
            Err(err) => return Result::Err(err.to_string()),
        },
        Settings::JPEG => {
            match tab.capture_screenshot(CaptureScreenshotFormatOption::Jpeg, None, None, true) {
                Ok(jpg) => jpg,
                Err(err) => return Result::Err(err.to_string()),
            }
        }
        Settings::PNG => {
            match tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true) {
                Ok(png) => png,
                Err(err) => return Result::Err(err.to_string()),
            }
        }
        Settings::WebP => {
            match tab.capture_screenshot(CaptureScreenshotFormatOption::Webp, None, None, true) {
                Ok(web) => web,
                Err(err) => return Result::Err(err.to_string()),
            }
        }
    };

    Result::Ok(raw)
}

pub fn print_file<P>(
    html: &str,
    width: u32,
    height: u32,
    settings: Settings,
    path: P,
) -> Result<(), String>
where
    P: AsRef<std::path::Path>,
{
    let buf = print(html, width, height, settings)?;

    let mut file = match std::fs::File::create(path) {
        Ok(file) => file,
        Err(err) => return Result::Err(err.to_string()),
    };

    if let Err(err) = std::io::Write::write_all(&mut file, &buf) {
        return Result::Err(err.to_string());
    };

    Result::Ok(())
}
