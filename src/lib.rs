use lol_html::html_content::ContentType;
use lol_html::{element, HtmlRewriter, Settings};
use serde::{Deserialize, Serialize};
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

struct BlogEnv {
    my_domain: String,
    notion_domain: String,
    index: String,
    links: String,
    sponsor: String,
    title: String,
    description: String,
    icon: String,
    allow_comment: bool,
}

impl BlogEnv {
    pub fn new(env: Env) -> Self {
        let my_domain = env.var("MY_DOMAIN").unwrap().to_string();
        let notion_domain = env.var("NOTION_DOMAIN").unwrap().to_string();
        let index = env.var("INDEX_PAGE_ID").unwrap().to_string();
        let links = env.var("LINK_PAGE_ID").unwrap().to_string();
        let sponsor = env.var("SPONSOR_PAGE_ID").unwrap().to_string();
        let title = env.var("PAGE_TITLE").unwrap().to_string();
        let description = env.var("PAGE_DESCRIPTION").unwrap().to_string();
        let icon = env.var("ICON_URL").unwrap().to_string();
        BlogEnv {
            my_domain,
            notion_domain,
            index,
            links,
            sponsor,
            title,
            description,
            icon,
            allow_comment: false,
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

async fn rewriter_js(req: Request, full_url: Url, blog_env: BlogEnv) -> Result<Response> {
    let request = Request::new_with_init(
        full_url.as_str(),
        RequestInit::new().with_method(req.method()),
    )?;
    return if let Ok(mut o) = Fetch::Request(request).send().await {
        let body = o.bytes().await.unwrap_or_default();
        let body = String::from_utf8_lossy(&body).to_string();
        let new_body = body.replace(&blog_env.my_domain, &blog_env.notion_domain);
        let response = Response::from_bytes(new_body.as_bytes().to_vec())?;
        let mut response_headers = Headers::new();
        response_headers.set("Content-Type", "application/x-javascript")?;
        Ok(response.with_headers(response_headers))
    } else {
        Response::redirect(full_url)
    };
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
    let new_response = Response::from_bytes(rewriter(body, blog_env))
        .unwrap()
        .with_headers(response_header)
        .with_status(response.status_code());
    Ok(new_response)
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let mut blog_env = BlogEnv::new(env);
    let mut full_url = req.url()?;
    full_url.set_host(Some(&blog_env.notion_domain))?;
    let path = req.path();
    match path.as_str() {
        "/" => {
            return Response::redirect(
                format!("https://{}/{}", &blog_env.my_domain, &blog_env.index).parse()?,
            );
        }
        // "/images/favicon.ico" | "/images/logo-ios.png" => {
        //     return Response::redirect(blog_env.icon.parse()?);
        // }
        "/links" => {
            return Response::redirect(
                format!("https://{}/{}", &blog_env.my_domain, &blog_env.links).parse()?,
            );
        }
        "/donate" | "/sponsor" => {
            return Response::redirect(
                format!("https://{}/{}", &blog_env.my_domain, &blog_env.sponsor).parse()?,
            );
        }
        "/api/v3/teV1" => {
            return Response::ok("success");
        }
        "/robots.txt" => {
            return Response::ok(format!(
                "Sitemap: https://{}/sitemap.xml",
                blog_env.my_domain
            ));
        }
        "/sitemap.xml" => {}
        _ => {}
    }
    if !path.ends_with(&blog_env.index) {
        blog_env.allow_comment = true;
    }
    if matches!(req.method(), Method::Options) {
        return cors_options().await;
    }
    if path.starts_with("/app") && path.ends_with(".js") {
        rewriter_js(req, full_url, blog_env).await
    } else if path.ends_with(".js") {
        proxy_js(req, full_url).await
    } else if path.starts_with("/api") {
        rewriter_api(req, full_url, blog_env).await
    } else {
        rewriter_html(req, full_url, blog_env).await
    }
}

fn rewriter(html: Vec<u8>, blog_env: BlogEnv) -> Vec<u8> {
    let mut output = vec![];
    let rewriter_http = r#"
    <script>
        const HTTP_BLACK_LIST = {
          "https://exp.notion.so/": "",
          "https://http-inputs-notion.splunkcloud.com/": "",
          "https://msgstore.www.notion.so/": "",
          "https://o324374.ingest.sentry.io/": "",
          "/api/v3/trackSegmentEvent": "{}",
          "/api/v3/ping": "{}",
          // "/api/v3/getPublicPageData":"{\"publicAccessRole\":\"none\"}",
          "/api/v3/getUserAnalyticsSettings":
            '{"isIntercomEnabled":true,"isZendeskEnabled":true,"isAmplitudeEnabled":true,"isSegmentEnabled":true,"intercomAppId":"gpfdrxfd","noIntercomUserId":false,"isSprigEnabled":true,"isLoaded":true}',
        };
      async function HttpRewriter(resource, config){
        for (const [k,v] of Object.entries(HTTP_BLACK_LIST)) {
          if (resource.startsWith(k)){
            var init = { "status" : 200 , "statusText" : "OK" };
            var defaultResponse = new Response(null, init);
            if (v !=""){
              defaultResponse = new Response.json(JSON.stringify(v));
            }
            return defaultResponse;
          }
        }
        // console.log(resource,config)
        const response = await originalFetch(resource, config);
        return response;
      }
      const { fetch: originalFetch } = window;
      window.fetch = async (...args) => {
          let [resource, config ] = args;
          const response = await HttpRewriter(resource, config);
          return response;
      };
    </script>"#;
    let h = r#"
    <div>Powered by <a href="https://github.com/cn-kali-team/notion-blog">Kali-Team</a></div>
      <script>
      localStorage.__console = true;
      window.CONFIG.domainBaseUrl = location.origin;
      let redirected = false;
      const el = document.createElement('div');
      const waitFor = (...selectors) => new Promise(resolve => {
        const delay = 500;
        const f = () => {
            const elements = selectors.map(selector => document.querySelector(selector));
            if (elements.every(element => element != null)) {
                resolve(elements);
            } else {
                setTimeout(f, delay);
            }
        }
        f();
      });
      function remove_notion_page_content(){
        let scroll_bar = document.getElementsByClassName("notion-page-content");
        if (scroll_bar.length > 0){
          scroll_bar[0].style.paddingBottom = "0vh";
        }
        let iterable = [
          "html.notion-html body.notion-body div#notion-app div.notion-app-inner.notion-light-theme div div.notion-cursor-listener div main.notion-frame div.notion-scroller.vertical div.pseudoSelection div",
          "html.notion-html body.notion-body div#notion-app div.notion-app-inner.notion-light-theme div div.notion-cursor-listener div main.notion-frame div.notion-scroller.vertical div.pseudoSelection div div div",
          "html.notion-html body.notion-body div#notion-app div.notion-app-inner.notion-light-theme div div.notion-cursor-listener div main.notion-frame div.notion-scroller.vertical div.pseudoSelection div div div div img",
          "html.notion-html body.notion-body div#notion-app div.notion-app-inner.notion-light-theme div div.notion-cursor-listener div main.notion-frame div.notion-scroller.vertical div.pseudoSelection div div div div img"];
        for (const entry of iterable) {
          let pseudo_selection = document.querySelector(entry);
          if (pseudo_selection !== null){
            pseudo_selection.style.height = "8vh";
          }
        }
        let notion_page_controls = document.querySelector("html.notion-html body.notion-body div#notion-app div.notion-app-inner.notion-light-theme div.notion-cursor-listener div div.notion-frame div.notion-scroller.vertical div div div div div.pseudoSelection div.notion-page-controls");
        if (notion_page_controls !== null){
          notion_page_controls.remove()
        }
      }
      remove_notion_page_content();
      function onDark() {
        el.innerHTML = '<div title="Change to Light Mode" style="margin-top: 8px; padding-left: 8px; padding-right: 8px; margin-left: 8px; margin-right: 8px; min-width: 0px;"><svg id="moon" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z"></path></svg></div></div>';
        document.body.classList.add('dark');
        __console.environment.ThemeStore.setState({ mode: 'dark' });
      };
      function onLight() {
        el.innerHTML = '<div title="Change to Dark Mode" style="margin-top: 8px; padding-left: 8px; padding-right: 8px; margin-left: 8px; margin-right: 8px; min-width: 0px;"><svg id="sun" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg></div></div>';
        document.body.classList.remove('dark');
        __console.environment.ThemeStore.setState({ mode: 'light' });
      }
      function toggle() {
        if (document.body.classList.contains('dark')) {
          onLight();
        } else {
          onDark();
        }
      }
      function addDarkModeButton(device) {
        const nav = device === 'web' ? document.querySelector('.notion-topbar').firstChild : document.querySelector('.notion-topbar-mobile');
        el.className = 'toggle-mode';
        el.addEventListener('click', toggle);
        nav.appendChild(el);
        onLight();
      }
      function addComment() {
          let comment = document.querySelector(".giscus");
          waitFor('.notion-page-content').then(([el]) => {
            let notion_page_content = document.querySelector(".notion-page-content");
            if (notion_page_content !== null && comment !== null) {
                notion_page_content.appendChild(comment);
            }
          });
      }
      // Notion 浮动 TOC
      function TOC() {
        waitFor('.notion-table_of_contents-block').then(([el]) => {
          const toc = document.querySelector('.notion-table_of_contents-block');
          if (toc) {
              const toc_p = toc.parentElement;
              if (!toc_p.classList.contains('notion-column-block')) {
                  return;
              }
              toc_p.style.position = 'sticky';
              toc_p.style.top = '0';
              toc_p.style.overflowY = 'scroll';
              toc_p.style.maxHeight = '100vh';
          }
      });
      }
      const observer = new MutationObserver(function() {
        remove_notion_page_content();
        TOC();
        addComment();
        if (redirected) return;
        const nav = document.querySelector('.notion-topbar');
        const mobileNav = document.querySelector('.notion-topbar-mobile');
        if (nav && nav.firstChild && nav.firstChild.firstChild
          || mobileNav && mobileNav.firstChild) {
          redirected = true;
          addDarkModeButton(nav ? 'web' : 'mobile');
        }
      });
      observer.observe(document.querySelector('#notion-app'), {
        childList: true,
        subtree: true,
      });
      remove_notion_page_content();
    </script>"#;
    let head = r#"
      <style>
      div.notion-topbar > div > div:nth-last-child(-n+4) { display: none !important; }
      div.notion-topbar-mobile > div:nth-child(1) { padding: 0px 10px !important; }
      div.notion-topbar-mobile > div:nth-child(3) { display: none !important; }
      div.notion-topbar-mobile > div:nth-child(4) { display: none !important; }
      div.notion-topbar-mobile > div:nth-child(5) { display: none !important; }
      div.notion-topbar > div > div:nth-child(1n).toggle-mode { display: block !important; }
      div.notion-topbar-mobile > div:nth-child(1n).toggle-mode { display: block !important; }
      </style>
    "#;
    let comment = r#"
    <script
        src="https://giscus.app/client.js"
        data-repo="cn-kali-team/notion-blog"
        data-repo-id="R_kgDOI1wUgQ"
        data-category="Announcements"
        data-category-id="DIC_kwDOI1wUgc4CZK9O"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="top"
        data-theme="preferred_color_scheme"
        data-lang="zh-CN"
        data-loading="lazy"
        crossorigin="anonymous"
        async>
    </script>
    "#;
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
                            el.set_attribute("content", &blog_env.description).unwrap();
                        }
                        "twitter:url" => {
                            el.set_attribute("content", &format!("https://{}", blog_env.my_domain))
                                .unwrap();
                        }
                        "twitter:image" => {
                            el.set_attribute("content", &blog_env.icon).unwrap();
                        }
                        "apple-itunes-app" => {
                            el.remove();
                        }
                        _ => {}
                    }
                    match el.get_attribute("property").unwrap_or_default().as_str() {
                        "og:site_name" | "og:title" | "og:description" => {
                            el.set_attribute("content", &blog_env.description).unwrap();
                        }
                        "og:url" => {
                            el.set_attribute("content", &format!("https://{}", blog_env.my_domain))
                                .unwrap();
                        }
                        "og:image" => {
                            el.set_attribute("content", &blog_env.icon).unwrap();
                        }
                        _ => {}
                    }
                    Ok(())
                }),
                element!("head", |el| {
                    el.append(head, ContentType::Html);
                    Ok(())
                }),
                element!("body", |el| {
                    el.append(rewriter_http, ContentType::Html);
                    el.append(h, ContentType::Html);
                    if blog_env.allow_comment {
                        el.append(comment, ContentType::Html);
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
