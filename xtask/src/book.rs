use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use chromiumoxide::handler::viewport::Viewport;
use chromiumoxide::page::ScreenshotParams;
use devx_cmd::cmd;
use devx_pre_commit::locate_project_root;
use futures::StreamExt;
use mdbook::{
    BookItem,
    book::{Book, Chapter, SectionNumber},
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
};
use std::time::Duration;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Stdio,
};
use tokio::runtime::Runtime;

// https://nick.groenen.me/notes/recursively-copy-files-in-rust/
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub struct ExamplesPreprocessor {
    pub examples_dir: PathBuf,
}

impl Preprocessor for ExamplesPreprocessor {
    fn name(&self) -> &str {
        "examples-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // Get examples chapter
        let examples: &mut Chapter = book
            .sections
            .iter_mut()
            .find_map(|section| match section {
                BookItem::Chapter(chapter) => {
                    if chapter.name == "Examples" {
                        Some(chapter)
                    } else {
                        None
                    }
                }
                BookItem::Separator => None,
                BookItem::PartTitle(_) => None,
            })
            .ok_or(Error::msg("Missing examples chapter"))?;

        // List all the examples
        let mut example_folders: Vec<PathBuf> = std::fs::read_dir(&self.examples_dir)
            .expect("Directory should be valid")
            .filter_map(|e| {
                let e = e.ok()?;
                let meta = e.metadata().ok()?;
                if meta.is_dir() { Some(e.path()) } else { None }
            })
            .collect();
        example_folders.sort();

        // Build examples
        example_folders
            .into_iter()
            .enumerate()
            .for_each(|(index, path)| make_append_example(examples, index as u32 + 1, path));

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

fn make_append_example(parent: &mut Chapter, index: u32, path: PathBuf) {
    let (item, snippet) = make_example(&parent, index, path);
    parent.sub_items.push(item);
    parent.content += &snippet;
}

async fn execute(input: &Path, output: &Path) {
    let url = format!("file://{}", input.display());

    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .arg("--disable-web-security")
            .viewport(Viewport {
                height: 300,
                width: 750,
                ..Default::default()
            })
            .build()
            .expect("Build config should work"),
    )
    .await
    .expect("Launch browser should work");

    let handle = tokio::task::spawn(async move {
        loop {
            match handler.next().await {
                Some(_) => {}
                None => break,
            };
        }
    });

    let page = browser.new_page(url).await.expect("Page should load");
    tokio::time::sleep(Duration::from_millis(5000)).await;

    page.save_screenshot(
        ScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .full_page(true)
            .omit_background(true)
            .build(),
        output,
    )
    .await
    .expect("Saving page should work");

    browser.close().await.expect("Closing browser should work");
    handle.await.expect("Handle should not fail");
}

fn make_example(parent: &Chapter, index: u32, path: PathBuf) -> (BookItem, String) {
    let root = locate_project_root().expect("Root should be findable");

    // Determine common variables
    let readme_file = "README.md";
    let index_html_file = "index.html";
    let main_rs_file = "src/main.rs";
    let scss_file = "scss/style.scss";
    let repo = "https://github.com/RabadanDotDev/maplibre-gl-js-rs/tree/main";
    let base_dir = std::env::var("BASE_PAGE_DIR").unwrap_or("http://localhost:3000".into());

    let readme = std::fs::read_to_string(path.join(readme_file)).expect("Read should work");
    let index_html = std::fs::read_to_string(path.join(index_html_file)).expect("Read should work");
    let main_rs = std::fs::read_to_string(path.join(main_rs_file)).expect("Read should work");
    let scss = std::fs::read_to_string(path.join(scss_file)).expect("Read should work");

    let id = path
        .file_name()
        .expect("There should be a name")
        .to_string_lossy()
        .into_owned();
    let url = format!("examples/{}-overview", id);
    let repo_url = format!("{repo}/examples/{id}");
    let title = readme
        .lines()
        .next()
        .expect("There should be a line")
        .get(2..)
        .expect("There should be a title");
    let description = readme
        .get((2 + title.len() + 1)..)
        .expect("There should be a description");

    let trunk_result = root.join(format!("target/dist/{id}"));
    let trunk_result_index = trunk_result.clone().join("index.html");
    let book_folder = root.join(format!("doc/src/examples/{id}"));

    // Generate example
    cmd!(
        "trunk",
        "build",
        "--release",
        "-M",
        "--public-url",
        format!("{base_dir}/examples/{id}"),
        "--dist",
        &trunk_result,
    )
    .current_dir(dbg!(&path))
    .spawn_with(io::stderr().into(), Stdio::piped())
    .expect("Spawn should work")
    .wait()
    .expect("Build should work");

    let _ = fs::remove_dir_all(&book_folder);
    copy_recursively(&trunk_result, &book_folder).expect("Copy should work");

    // Generate preview
    cmd!("trunk", "build", "--release", "-M", "--dist", &trunk_result)
        .current_dir(&path)
        .spawn_with(io::stderr().into(), Stdio::piped())
        .expect("Spawn should work")
        .wait()
        .expect("Build should work");
    Runtime::new().unwrap().block_on(async {
        execute(&trunk_result_index, &book_folder.join("preview.png")).await;
    });

    let example_embedded = format!(
        "<iframe src='{base_dir}/examples/{id}/index.html', width=100%, height=300px></iframe>\n"
    );
    let index_embedded = format!("```html\n<!-- {index_html_file} -->\n\n{index_html}```");
    let main_embedded = format!("```rust\n// {main_rs_file} \n\n{main_rs}```");
    let scss_embedded = format!("```css\n/* {scss_file} */\n\n{scss}```");
    let preview =
        format!("<a href={url}.html><img style='opacity: 0.5' src=examples/{id}/preview.png></a>");

    let page_contents = format!(
        "# {title} [🔗]({repo_url})\n{description}\n{example_embedded}\n{main_embedded}\n{index_embedded}\n{scss_embedded}"
    );
    let snippet = format!("## {title} [🌐]({url}.html) [🔗]({repo_url})\n{preview}\n{description}");

    let number = {
        let mut number = parent
            .number
            .as_ref()
            .map(|v| v.0.clone())
            .unwrap_or_default();
        number.push(index);
        Some(SectionNumber(number))
    };
    let sub_items = Vec::new();
    let source_path = Some(path);
    let path = Some(url.into());
    let parent_names = {
        let mut parent_names = parent.parent_names.clone();
        parent_names.push(parent.name.clone());
        parent_names
    };

    let item = BookItem::Chapter(Chapter {
        name: title.to_string(),
        content: page_contents,
        number,
        sub_items,
        source_path,
        path,
        parent_names,
    });

    (item, snippet)
}
