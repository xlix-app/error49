use std::io::Read;
use once_cell::sync::OnceCell;


/// Position of the server address argument.
pub static ARG_POS_ADDRESS: usize = 1;

/// Argument flag used to pass the path to the file containing page text.
static ARG_FLAG_TEXT: &'static str = "--text-";

/// Argument flag used to pass the path to the file containing page icon.
static ARG_FLAG_ICON: &'static str = "--icon-";

/// Contents of the HTML page after inserting other components.
pub static HTML_PAGE: OnceCell<String> = OnceCell::new();

/// Contents of the HTML page.
static HTML_PAGE_BASE: &'static str = include_str!("web/index.html");

/// Text that will be displayed on the page.
static HTML_TEXT: &'static str = include_str!("web/text.txt");

/// Icon that will be displayed on the page.
static HTML_ICON: &'static str = include_str!("web/icon.html");

/// Loads the `HTML_PAGE`.
pub fn init_html_page<'a>(args: &Vec<String>) {
    if HTML_PAGE.get().is_some() {
        return;
    }

    let mut storage = (None, None);  // (text, icon)

    let load_component_content = |arg: &String, flag: &str, storage: &mut Option<String>| -> Option<()> {
        if !arg.starts_with(flag) {
            return None;
        }

        let path = &arg[flag.len()..];

        let mut content = String::new();
        std::fs::OpenOptions::new()
            .read(true)
            .open(path)
            .ok()?
            .read_to_string(&mut content)
            .ok()?;

        *storage = Some(content);

        Some(())
    };

    for arg in args {
        match &mut storage {
            (Some(_), Some(_)) => break,
            (text, icon) => {
                if text.is_none() {
                    load_component_content(arg, ARG_FLAG_TEXT, text);
                }

                if icon.is_none() {
                    load_component_content(arg, ARG_FLAG_ICON, icon);
                }
            }
        }
    }

    let html_text = if let Some(text) = &storage.0 {
        text.as_str()
    } else {
        HTML_TEXT
    };

    let html_icon = if let Some(icon) = &storage.1 {
        icon.as_str()
    } else {
        HTML_ICON
    };

    let full_page = HTML_PAGE_BASE
        .replace("{{text}}", html_text)
        .replace("{{icon}}", html_icon);

    let _ = HTML_PAGE.set(full_page);
}
