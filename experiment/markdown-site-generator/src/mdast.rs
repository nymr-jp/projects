use markdown::message::Message;
use markdown::mdast::Node;

pub fn to_html(node: &Node) -> Result<String, Message> {
    let mut html = String::from("");

    if let Some(children) = node.children() {
        let mut content = String::from("");

        for child in children {
            content.push_str(&to_html(child)?);
        };

        match node {
            Node::Heading(heading) => {
                let depth = heading.depth;
                html = format!("<h{depth}>{content}</h{depth}>")
            },
            Node::Emphasis(_) => {
                html = format!("<em>{content}</em>")
            },
            Node::Delete(_) => {
                html = format!("<s>{content}</s>")
            },
            Node::Link(link) => {
                let url = &link.url;
                html = format!("<a href={url}>{content}</a>")
            },
            _ => {
                html = content;
            }
        }
    } else {
        if let Node::Text(text) = node {
            html = text.value.clone();
        }
    }

    Ok(html)
}
