mod html;
mod page;

use crate::html::{BASE, HEAD, RESIZE, THEME, TOC};
use crate::page::QueryBody;
use lol_html::html_content::ContentType;
use lol_html::{element, HtmlRewriter, Settings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use worker::wasm_bindgen::JsValue;
use worker::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PublicPageData {
    r#type: String,
    name: String,
    block_id: String,
    space_domain: Option<String>,
    show_move_to: bool,
    save_parent: bool,
    should_duplicate: bool,
    project_management_launch: bool,
    requested_on_public_domain: bool,
    configure_open_in_desktop_app: bool,
    mobile_data: MobileData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MobileData {
    is_push: bool,
}

pub struct BlogEnv {
    pub page_map: HashMap<String, String>,
    pub comment_map: HashMap<String, String>,
    pub my_domain: String,
    pub notion_domain: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub query_body: String,
}

fn var_to_map(env: &Env, name: &str) -> HashMap<String, String> {
    let page_id_map = env
        .var(name)
        .unwrap_or(worker::Var::from(JsValue::from_str("{}")))
        .to_string();
    serde_json::from_str::<HashMap<String, String>>(&page_id_map).unwrap_or_default()
}

impl BlogEnv {
    pub fn new(env: Env) -> Self {
        let my_domain = env.var("MY_DOMAIN").unwrap().to_string();
        let notion_domain = env.var("NOTION_DOMAIN").unwrap().to_string();
        let title = env.var("PAGE_TITLE").unwrap().to_string();
        let description = env.var("PAGE_DESCRIPTION").unwrap().to_string();
        let icon = env.var("ICON_URL").unwrap().to_string();
        let query_body = env.var("QUERY_BODY").unwrap().to_string();
        let page_map = var_to_map(&env, "PAGE_MAP");
        let comment_map = var_to_map(&env, "COMMENT_MAP");
        BlogEnv {
            page_map,
            comment_map,
            my_domain,
            notion_domain,
            title,
            description,
            icon,
            query_body,
        }
    }
}

async fn cors_options() -> Result<Response> {
    let response = Response::empty()?;
    let mut header = Headers::new();
    header.set("Access-Control-Allow-Origin", "*")?;
    header.set(
        "Access-Control-Allow-Methods",
        "GET,POST,PUT,PATCH,TRACE,DELETE,HEAD,OPTIONS",
    )?;
    header.set("Access-Control-Allow-Headers", "Content-Type")?;
    header.set("Access-Control-Allow-Credentials", "True")?;
    header.set("Access-Control-Max-Age", "1728000")?;
    Ok(response.with_headers(header))
}

// async fn rewriter_js(req: Request, full_url: Url, blog_env: BlogEnv) -> Result<Response> {
//     let request = Request::new_with_init(
//         full_url.as_str(),
//         RequestInit::new().with_method(req.method()),
//     )?;
//     return if let Ok(mut o) = Fetch::Request(request).send().await {
//         let body = o.bytes().await.unwrap_or_default();
//         let body = String::from_utf8_lossy(&body).to_string();
//         let new_body = body.replace(&blog_env.my_domain, &blog_env.notion_domain);
//         let response = Response::from_bytes(new_body.as_bytes().to_vec())?;
//         let mut response_headers = Headers::new();
//         response_headers.set("Content-Type", "application/x-javascript")?;
//         Ok(response.with_headers(response_headers))
//     } else {
//         Response::redirect(full_url)
//     };
// }
async fn get_pages(
    path: &str,
    notion_domain: &String,
    query_body: &str,
) -> Result<page::QueryCollection> {
    let api_url = format!("https://{}/api/v3/{}", notion_domain, path);
    let mut header = Headers::new();
    header.set("Content-Type", "application/json")?;
    header.set(
        "Accept-Language",
        "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2",
    )?;
    header.set(
        "User-Agent",
        " Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0",
    )?;
    let request = Request::new_with_init(
        api_url.as_str(),
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(header)
            .with_body(Some(JsValue::from_str(query_body))),
    )?;
    let mut res = Fetch::Request(request).send().await?;
    let body = res.text().await?;
    match serde_json::from_str(body.as_str()) {
        Ok(page) => Ok(page),
        Err(err) => Err(Error::JsError(err.to_string())),
    }
}

async fn proxy_js(req: Request, full_url: Url) -> Result<Response> {
    let request = Request::new_with_init(
        full_url.as_str(),
        RequestInit::new().with_method(req.method()),
    )?;
    Fetch::Request(request).send().await
}

async fn rewriter_api(mut req: Request, full_url: Url, blog_env: BlogEnv) -> Result<Response> {
    let mut headers = req.headers().clone();
    headers.set("Content-Type", "application/json;charset=UTF-8")?;
    headers.set("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    let body = if req.path() == "/api/v3/getPublicPageData" {
        let mut public_page_data: PublicPageData =
            serde_json::from_slice(&req.bytes().await.unwrap_or_default()).unwrap();
        public_page_data.requested_on_public_domain = true;
        let space_domain = blog_env.notion_domain.replace(".notion.site", "");
        public_page_data.space_domain = Some(space_domain);
        Some(JsValue::from_str(
            &serde_json::to_string(&public_page_data).unwrap_or_default(),
        ))
    } else {
        Some(JsValue::from_str(
            String::from_utf8_lossy(&req.bytes().await.unwrap_or_default()).as_ref(),
        ))
    };
    let request = Request::new_with_init(
        full_url.as_str(),
        RequestInit::new()
            .with_headers(headers)
            .with_body(body)
            .with_method(Method::Post),
    )?;
    if let Ok(response) = Fetch::Request(request).send().await {
        let mut header = Headers::new();
        // response_headers.delete("Content-Security-Policy")?;
        // response_headers.delete("X-content-Security-Policy")?;
        // response_headers.delete("Content-Security-Policy")?;
        // response_headers.delete("X-Content-Security-Policy")?;
        // response_headers.delete("Referrer-Policy")?;
        // response_headers.delete("X-Permitted-Cross-Domain-Policies")?;
        header.set("Access-Control-Allow-Origin", "*")?;
        header.set("Timing-Allow-Origin", "*")?;
        Ok(response.with_headers(header))
    } else {
        Response::redirect(full_url)
    }
}

async fn get_page_title_icon(
    id: &uuid::Uuid,
    blog_env: &BlogEnv,
) -> (Option<String>, Option<String>) {
    let body = serde_json::to_string(&QueryBody::new(id.to_string())).unwrap_or_default();
    match get_pages("syncRecordValues", &blog_env.notion_domain, &body).await {
        Ok(page) => {
            return (page.get_title(id), page.get_icon(id));
        }
        Err(err) => {
            console_log!("{}", err)
        }
    }
    (None, None)
}

fn update_history(page_map: &HashMap<String, String>) -> String {
    let page_map_str = serde_json::to_string(&page_map).unwrap_or("{}".to_string());
    let mut slug_map = HashMap::new();
    for (k, v) in page_map {
        slug_map.insert(v, k);
    }
    let slug_map_str = serde_json::to_string(&slug_map).unwrap_or("{}".to_string());
    r#"
      <script>
      const SLUG_TO_PAGE = slug_map_str;
      const PAGE_TO_SLUG = page_map_str;
      function update_history() {
        if (PAGE_TO_SLUG[location.pathname]){
          return;
        }
        if (SLUG_TO_PAGE[location.pathname.slice(-32)]){
          history.replaceState(history.state, '', SLUG_TO_PAGE[location.pathname.slice(-32)]);
        } else{
          let path_name = location.pathname.split("/");
          history.replaceState(history.state, '', "/"+ path_name[path_name.length-1]);
        }
      };
      update_history();
    </script>"#
        .replace("slug_map_str", &slug_map_str)
        .replace("page_map_str", &page_map_str)
}

async fn rewriter_html(req: Request, full_url: Url, blog_env: BlogEnv) -> Result<Response> {
    let headers = req.headers().clone();
    let request = Request::new_with_init(
        full_url.as_str(),
        RequestInit::new()
            .with_headers(headers)
            .with_method(req.method()),
    )?;
    let mut response = Fetch::Request(request).send().await?;
    let body = response.bytes().await.unwrap_or_default();
    let mut response_header = response.headers().clone();
    if let Ok(Some(mut csp)) = response_header.get("Content-Security-Policy") {
        csp = csp.replace(
      "https://gist.github.com",
      "https://gist.github.com https://giscus.app/client.js https://static.cloudflareinsights.com",
    );
        if csp.contains("style-src") {
            csp = csp.replace("style-src", "style-src https://giscus.app/default.css");
        } else {
            csp.push_str(";style-src https://giscus.app/default.css");
        }
        response_header.set("Content-Security-Policy", &csp)?;
    }
    let page_id = if full_url.path().len() > 32 {
        let page_id = &full_url.path()[full_url.path().len() - 32..];
        page_id.to_string()
    } else {
        blog_env.page_map.get("/").unwrap_or(&String::new()).clone()
    };
    let mut title = None;
    let mut icon_url = None;
    if let Ok(page_uuid) = uuid::Uuid::parse_str(&page_id) {
        (title, icon_url) = get_page_title_icon(&page_uuid, &blog_env).await;
    }
    let new_response = Response::from_bytes(rewriter(body, blog_env, title, icon_url))
        .unwrap()
        .with_headers(response_header)
        .with_status(response.status_code());
    Ok(new_response)
}

fn get_comment(comment_map: &HashMap<String, String>) -> String {
    let mut script = r#"
    <script>
      let page_location = new URL(window.location.toString());
      let giscus_session = page_location.searchParams.get("giscus");
      if (giscus_session!==null){
        giscus_session = '"'+giscus_session+'"'
        localStorage.setItem("giscus-session",giscus_session);
      }
      function addComment() {
          let my_giscus = document.getElementById('giscus');
          waitForElementToExist('.notion-table_of_contents-block').then((el)=>{
          addTOC();
          if (my_giscus!==null)return;
          let comment = document.createElement('script');
              comment.id = "giscus";
              comment.setAttribute("src","https://giscus.app/client.js");
              comment.setAttribute("data-repo","DATA_REPO");
              comment.setAttribute("data-repo-id","DATA_REPO_ID");
              comment.setAttribute("data-category","Announcements");
              comment.setAttribute("data-category-id","DATA_CATEGORY_ID");
              comment.setAttribute("data-mapping","DATA_MAPPING");
              comment.setAttribute("data-strict","0");
              comment.setAttribute("data-reactions-enabled","1");
              comment.setAttribute("data-emit-metadata","0");
              comment.setAttribute("data-input-position","DATA_INPUT_POSITION");
              comment.setAttribute("data-theme","DATA_THEME");
              comment.setAttribute("data-lang","DATA_LANG");
              comment.setAttribute("data-loading","lazy");
              comment.setAttribute("crossorigin","anonymous");
              const content = document.querySelector('.notion-page-content');
              content.append(comment);
          });
      }
      waitForElementToExist('.shadow-cursor-breadcrumb').then((el)=>{
          const breadcrumb = new MutationObserver(function(mutationsList, observer) {
            console.log(mutationsList)
            addComment();
          });
          breadcrumb.observe(document.querySelector('.shadow-cursor-breadcrumb'), {
            childList: true,
            subtree: true,
          });
      });
      waitForElementToExist('.notion-page-content').then((el)=>{
        addComment();
      });
    </script>"#
        .to_string();
    script = script
        .replace(
            "DATA_REPO_ID",
            comment_map
                .get("data-repo-id")
                .unwrap_or(&String::from("R_kgDOI1wUgQ")),
        )
        .replace(
            "DATA_REPO",
            comment_map
                .get("data-repo")
                .unwrap_or(&String::from("cn-kali-team/notion-blog")),
        )
        .replace(
            "DATA_CATEGORY_ID",
            comment_map
                .get("data-category-id")
                .unwrap_or(&String::from("DIC_kwDOI1wUgc4CZK9O")),
        )
        .replace(
            "DATA_MAPPING",
            comment_map
                .get("data-mapping")
                .unwrap_or(&String::from("title")),
        )
        .replace(
            "DATA_INPUT_POSITION",
            comment_map
                .get("data-input-position")
                .unwrap_or(&String::from("top")),
        )
        .replace(
            "DATA_THEME",
            comment_map
                .get("data-theme")
                .unwrap_or(&String::from("preferred_color_scheme")),
        )
        .replace(
            "DATA_LANG",
            comment_map
                .get("data-lang")
                .unwrap_or(&String::from("zh-CN")),
        );
    script
}

fn rewriter(
    html: Vec<u8>,
    blog_env: BlogEnv,
    title: Option<String>,
    icon_url: Option<String>,
) -> Vec<u8> {
    let mut output = vec![];
    let title = title.unwrap_or(blog_env.description);
    let mut icon_url = icon_url.unwrap_or(blog_env.icon);
    if icon_url.starts_with("/images/") {
        icon_url = format!("https://{}{}", blog_env.my_domain, icon_url);
    }
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("title", |el| {
                    el.set_inner_content(&blog_env.title, ContentType::Text);
                    Ok(())
                }),
                element!("meta", |el| {
                    match el.get_attribute("name").unwrap_or_default().as_str() {
                        "description"
                        | "twitter:title"
                        | "twitter:site"
                        | "twitter:description" => {
                            el.set_attribute("content", &title).unwrap();
                        }
                        "twitter:url" => {
                            el.set_attribute("content", &format!("https://{}", blog_env.my_domain))
                                .unwrap();
                        }
                        "twitter:image" => {
                            el.set_attribute("content", &icon_url).unwrap();
                        }
                        "apple-itunes-app" => {
                            el.remove();
                        }
                        _ => {}
                    }
                    match el.get_attribute("property").unwrap_or_default().as_str() {
                        "og:site_name" | "og:title" | "og:description" => {
                            el.set_attribute("content", &title).unwrap();
                        }
                        "og:url" => {
                            el.set_attribute("content", &format!("https://{}", blog_env.my_domain))
                                .unwrap();
                        }
                        "og:image" => {
                            el.set_attribute("content", &icon_url).unwrap();
                        }
                        _ => {}
                    }
                    Ok(())
                }),
                element!("head", |el| {
                    el.append(HEAD, ContentType::Html);
                    Ok(())
                }),
                element!("body", |el| {
                    // el.append(rewriter_http, ContentType::Html);
                    el.append(BASE, ContentType::Html);
                    el.append(&update_history(&blog_env.page_map), ContentType::Html);
                    el.append(RESIZE, ContentType::Html);
                    el.append(THEME, ContentType::Html);
                    el.append(TOC, ContentType::Html);
                    if !blog_env.comment_map.is_empty() {
                        el.append(&get_comment(&blog_env.comment_map), ContentType::Html);
                    }
                    Ok(())
                }),
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );
    rewriter.write(&html).unwrap();
    rewriter.end().unwrap();
    output
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let blog_env = BlogEnv::new(env);
    let mut full_url = req.url()?;
    full_url.set_host(Some(&blog_env.notion_domain))?;
    let path = req.path();
    if let Some(page_id) = blog_env.page_map.get(&path) {
        return Response::redirect(format!("https://{}/{}", &blog_env.my_domain, page_id).parse()?);
    }
    match path.as_str() {
        "/BingSiteAuth.xml" => {
            return Response::ok("<?xml version=\"1.0\"?><users><user>6743F9D57B1260BC5F59A888815408B4</user></users>");
        }
        "/sitemap.xml" => {
            let page = get_pages(
                "queryCollection?src=initial_load",
                &blog_env.notion_domain,
                &blog_env.query_body,
            )
            .await?;
            let header = Headers::from_iter(vec![("Content-Type", "text/xml")]);
            let sitemap = page.get_sitemap().replace("MY_DOMAIN", &blog_env.my_domain);
            return Ok(Response::ok(sitemap)?.with_headers(header));
        }
        "/index.xml" => {
            let page = get_pages(
                "queryCollection?src=initial_load",
                &blog_env.notion_domain,
                &blog_env.query_body,
            )
            .await?;
            let header = Headers::from_iter(vec![("Content-Type", "text/xml")]);
            let atom = page
                .get_atom(&blog_env)
                .replace("MY_DOMAIN", &blog_env.my_domain);
            return Ok(Response::ok(atom)?.with_headers(header));
        }
        "/api/v3/teV1" => {
            return Response::ok("success");
        }
        "/robots.txt" => {
            return Response::ok(format!(
                "User-agent: *\nAllow: /\nSitemap: https://{}/sitemap.xml",
                blog_env.my_domain
            ));
        }
        "/images/favicon.ico" => {
            return Response::redirect(blog_env.icon.parse()?);
        }
        _ => {}
    }
    if matches!(req.method(), Method::Options) {
        return cors_options().await;
    }
    if path.ends_with(".js") {
        proxy_js(req, full_url).await
    } else if path.starts_with("/api") {
        rewriter_api(req, full_url, blog_env).await
    } else {
        rewriter_html(req, full_url, blog_env).await
    }
}

#[cfg(test)]
mod tests {
    use crate::PublicPageData;
    use lol_html::{element, HtmlRewriter, Settings};

    #[test]
    fn test_json() {
        let j = r#"{"type":"block-space","name":"page","blockId":"edb6a939-baab-4424-a25f-d295b3c51312","showMoveTo":false,"saveParent":false,"shouldDuplicate":false,"projectManagementLaunch":false,"requestedOnPublicDomain":false,"configureOpenInDesktopApp":false,"mobileData":{"isPush":false}}"#;
        let p: PublicPageData = serde_json::from_str(j).unwrap();
        println!("{:#?}", p);
    }

    #[test]
    fn it_works() {
        let html = r#"<meta name="description" content="A new tool that blends your everyday work apps into one. It's the all-in-one workspace for you and your team">
        <meta name="twitter:site" content="@NotionHQ">
        <meta name="twitter:url" content="https://www.notion.so">
        "#.to_string();
        let mut output = vec![];
        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!("meta", |el| {
                    match el.get_attribute("name").unwrap_or_default().as_str() {
                        "description"
                        | "twitter:title"
                        | "twitter:site"
                        | "twitter:description" => {
                            el.set_attribute("content", "Kali-Team").unwrap();
                        }
                        "twitter:url" => {
                            el.set_attribute("content", "https://blog.kali-team.cn")
                                .unwrap();
                        }
                        _ => {}
                    }
                    match el.get_attribute("property").unwrap_or_default().as_str() {
                        "og:site_name" | "og:title" | "og:description" => {
                            el.set_attribute("content", "Kali-Team").unwrap();
                        }
                        "og:url" => {
                            el.set_attribute("content", "https://blog.kali-team.cn")
                                .unwrap();
                        }
                        "og:image" => {}
                        _ => {}
                    }

                    Ok(())
                })],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );
        rewriter.write(html.as_bytes()).unwrap();
        rewriter.end().unwrap();
        println!("{}", String::from_utf8_lossy(&output));
    }
}
