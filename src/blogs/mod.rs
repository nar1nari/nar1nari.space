use leptos::*;
use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Response};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub content_html: String,
}

#[derive(Debug, Deserialize, Default)]
struct FrontMatter {
    title: Option<String>,
    date: Option<String>,
    tags: Option<Vec<String>>,
}

impl BlogPost {
    pub fn from_md(slug: &str, content: &str) -> Self {
        let (fm_str, md_str) = if content.starts_with("---") {
            let mut parts = content.splitn(3, "---");
            parts.next();
            (
                parts.next().unwrap_or("").trim(),
                parts.next().unwrap_or("").trim(),
            )
        } else {
            ("", content)
        };

        let fm: FrontMatter = serde_yaml::from_str(fm_str).unwrap_or_default();

        let mut html_output = String::new();
        html::push_html(&mut html_output, Parser::new(md_str));

        Self {
            slug: slug.to_string(),
            title: fm.title.unwrap_or_else(|| slug.to_string()),
            date: fm.date.unwrap_or_default(),
            tags: fm.tags.unwrap_or_default(),
            content_html: html_output,
        }
    }
}

pub async fn fetch_blog(slug: &str) -> Option<BlogPost> {
    let url = format!("/assets/blogs/{}.md", slug);
    let window = window()?;
    let resp: Response = JsFuture::from(window.fetch_with_str(&url))
        .await
        .ok()?
        .into();

    if !resp.ok() || resp.status() == 404 {
        return None;
    }

    let text = JsFuture::from(resp.text().ok()?).await.ok()?.as_string()?;

    if text.trim_start().starts_with("<!DOCTYPE html>") || text.trim_start().starts_with("<html") {
        return None;
    }

    Some(BlogPost::from_md(slug, &text))
}

pub async fn fetch_all_blogs() -> Vec<BlogPost> {
    let mut posts = Vec::new();
    let mut i = 1;

    while let Some(post) = fetch_blog(&i.to_string()).await {
        posts.push(post);
        i += 1;
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}
