use markdown::{to_mdast, ParseOptions};

use generator::mdast::to_html;
use generator::file::reader;
use generator::file::writer;

fn main() -> std::io::Result<()> {
    let inputs = reader("../pages").unwrap();

    let outputs = inputs.into_iter().map(|(path, md)| {
        let mdast = to_mdast(&md, &ParseOptions::gfm()).unwrap();
        let main = to_html(&mdast).unwrap();

        (path, format!("
            <html>
            <head>
                <title>Nymr's Blog</title>
            </head>
            <body>
                {main}
            </body>
            </html>
        "))
    }).collect();

    writer("../docs", outputs)?;

    Ok(())
}
