/* CONFIGURATION STARTS HERE */

const SLUG_TO_PAGE = {
  "": INDEX_PAGE_ID,
  links: LINK_PAGE_ID,
  donate: DONATE_PAGE_ID,
};

const GOOGLE_FONT = "";

/* Step 5: enter any custom scripts you'd like */
const CUSTOM_SCRIPT = ``;

/* CONFIGURATION ENDS HERE */

const PAGE_TO_SLUG = {};
const slugs = [];
const pages = [];
Object.keys(SLUG_TO_PAGE).forEach((slug) => {
  const page = SLUG_TO_PAGE[slug];
  slugs.push(slug);
  pages.push(page);
  PAGE_TO_SLUG[page] = slug;
});

addEventListener("fetch", (event) => {
  console.log(event);
  event.respondWith(fetchAndApply(event.request));
});
// 生成网站地图
function makeLoc(path, time) {
  return (
    "<url>\n\t<loc>https://" +
    MY_DOMAIN +
    "/" +
    path +
    "</loc>\n\t<lastmod>" +
    time +
    "</lastmod>\n\t<changefreq>weekly</changefreq>\n\t<priority>0.6</priority>\n</url>\n"
  );
}
// 生成订阅信息
function makeEntry(path, title, published_time, updated_time) {
  return (
    "<entry>\n\t<title>" +
    title +
    '</title>\n\t<link href="https://' +
    MY_DOMAIN +
    "/" +
    path +
    '" rel="alternate"></link>\n\t' +
    "<published>" +
    published_time +
    "</published>\n\t" +
    "<updated>" +
    updated_time +
    "</updated>\n\t" +
    "<id>" +
    "https://" +
    MY_DOMAIN +
    "/" +
    path +
    "</id>\n\t" +
    '<summary type="html">' +
    title +
    "</summary>\n</entry>\n"
  );
}

async function generateAtom() {
  let atom_xml = '<?xml version="1.0" encoding="utf-8"?>\n';
  atom_xml += '<feed xmlns="http://www.w3.org/2005/Atom" xml:lang="zh-hans">\n';
  atom_xml += "<title>" + PAGE_DESCRIPTION + "</title>\n";
  atom_xml +=
    '<link rel="alternate" href="https://' + MY_DOMAIN + '/"></link>\n';
  atom_xml +=
    '<link rel="self" href="https://' + MY_DOMAIN + '/index.xml"></link>\n';
  atom_xml += "<id>https://" + MY_DOMAIN + "/</id>\n";
  atom_xml += "<updated>" + new Date().toISOString() + "</updated>\n";
  response = await fetch(
    "https://" + NOTION_DOMAIN + "/api/v3/queryCollection?src=reset",
    {
      headers: {
        "User-Agent":
          "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/109.0",
        Accept: "application/x-ndjson",
        "Accept-Language":
          "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2",
        "notion-client-version": "23.11.0.40",
        "notion-audit-log-platform": "web",
        "x-notion-active-user-header": "",
        "Content-Type": "application/json",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-origin",
        Pragma: "no-cache",
        "Cache-Control": "no-cache",
      },
      body: SITEMAP_BODY,
      method: "POST",
    }
  );
  let body = await response.json();
  let blocks = body.recordMap.block;
  for (const [k, block] of Object.entries(blocks)) {
    let properties = block.value.properties;
    let page_id = k.replaceAll("-", "");
    if (
      properties !== undefined &&
      properties.title.length === 1 &&
      page_id !== INDEX_PAGE_ID
    ) {
      let page_title = properties.title[0][0];
      let original_page_title = page_title;
      page_title = page_title.replace(/^[^-\w.]{1,}/gmu, ""); //删除前面的
      page_title = page_title.replace(/[^-\w.]{1,}$/gmu, ""); //删除后面的
      page_title = page_title.replace(/[^-\w.]{1,}/gmu, "-"); //替换中间的
      page_title = page_title.replace("--", ""); //替换双重横杠
      if (page_title.length > 0) {
        atom_xml += makeEntry(
          page_title + "-" + page_id,
          original_page_title,
          new Date(block.value.created_time).toISOString(),
          new Date(block.value.last_edited_time).toISOString()
        );
      } else {
        atom_xml += makeEntry(
          page_id,
          original_page_title,
          new Date(block.value.created_time).toISOString(),
          new Date(block.value.last_edited_time).toISOString()
        );
      }
    }
  }
  atom_xml += "</feed>";
  return atom_xml;
}
async function generateSitemap() {
  let sitemap =
    '<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n';
  response = await fetch(
    "https://" + NOTION_DOMAIN + "/api/v3/queryCollection?src=reset",
    {
      headers: {
        "User-Agent":
          "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/109.0",
        Accept: "application/x-ndjson",
        "Accept-Language":
          "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2",
        "notion-client-version": "23.11.0.40",
        "notion-audit-log-platform": "web",
        "x-notion-active-user-header": "",
        "Content-Type": "application/json",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-origin",
        Pragma: "no-cache",
        "Cache-Control": "no-cache",
      },
      body: SITEMAP_BODY,
      method: "POST",
    }
  );
  let body = await response.json();
  let blocks = body.recordMap.block;
  for (const [k, block] of Object.entries(blocks)) {
    let properties = block.value.properties;
    let page_id = k.replaceAll("-", "");
    if (
      properties !== undefined &&
      properties.title.length === 1 &&
      page_id !== INDEX_PAGE_ID
    ) {
      let page_title = properties.title[0][0];
      page_title = page_title.replace(/^[^-\w.]{1,}/gmu, ""); //删除前面的
      page_title = page_title.replace(/[^-\w.]{1,}$/gmu, ""); //删除后面的
      page_title = page_title.replace(/[^-\w.]{1,}/gmu, "-"); //替换中间的
      page_title = page_title.replace("--", ""); //替换双重横杠
      if (page_title.length > 0) {
        sitemap += makeLoc(
          page_title + "-" + page_id,
          new Date(block.value.last_edited_time).toJSON().slice(0, 10)
        );
      } else {
        sitemap += makeLoc(
          page_id,
          new Date(block.value.last_edited_time).toJSON().slice(0, 10)
        );
      }
    }
  }
  sitemap += "</urlset>";
  return sitemap;
}
const HTTP_BLACK_LIST = {
  "https://exp.notion.so/": "",
  "https://http-inputs-notion.splunkcloud.com/": "",
  "https://msgstore.www.notion.so/": "",
  "https://o324374.ingest.sentry.io/": "",
  "/api/v3/trackSegmentEvent": "{}",
  // "/api/v3/getPublicPageData":"{\"publicAccessRole\":\"none\"}",
  "/api/v3/getUserAnalyticsSettings":"{\"isIntercomEnabled\":true,\"isZendeskEnabled\":true,\"isAmplitudeEnabled\":true,\"isSegmentEnabled\":true,\"intercomAppId\":\"gpfdrxfd\",\"noIntercomUserId\":false,\"isSprigEnabled\":true,\"isLoaded\":true}"
};

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods":
    "GET,POST,PUT,PATCH,TRACE,DELETE,HEAD,OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type",
  "Access-Control-Allow-Credentials": "True",
  "Access-Control-Max-Age": "1728000",
};
// 当前端浏览器询问option返回可以支持各种请求，可以跨域
function handleOptions(request) {
  if (
    request.headers.get("Origin") !== null &&
    request.headers.get("Access-Control-Request-Method") !== null &&
    request.headers.get("Access-Control-Request-Headers") !== null
  ) {
    // Handle CORS pre-flight request.
    for (const [k, v] of corsHeaders.entries()) {
      request.headers.set(k, v);
    }
    return new Response(null, {
      headers: request.headers,
    });
  } else {
    // Handle standard OPTIONS request.
    return new Response(null, {
      headers: {
        Allow: "GET,POST,PUT,PATCH,TRACE,DELETE,HEAD,OPTIONS",
      },
    });
  }
}
// 路由入口
async function fetchAndApply(request) {
  console.log(request.toString());
  if (request.method === "OPTIONS") {
    return handleOptions(request);
  }
  let url = new URL(request.url);
  console.log(url.toString());
  url.hostname = NOTION_DOMAIN;
  if (url.pathname === "/robots.txt") {
    return new Response("Sitemap: https://" + MY_DOMAIN + "/sitemap.xml");
  }
  if (url.pathname === "/sitemap.xml") {
    let response = new Response(await generateSitemap());
    response.headers.set("content-type", "application/xml");
    return response;
  }
  if (url.pathname === "/index.xml") {
    let response = new Response(await generateAtom());
    response.headers.set("content-type", "application/xml");
    return response;
  }
  let response;
  if (
    (url.pathname.startsWith("/app") || url.pathname.startsWith("/mermaid")) &&
    url.pathname.endsWith("js")
  ) {
    response = await fetch(url.toString());
    let body = await response.text();
    response = new Response(
      body
        .replace(/'${MY_DOMAIN}'/g, NOTION_DOMAIN)
        .replace(/'${MY_DOMAIN}'/g, NOTION_DOMAIN),
      response
    );
    response.headers.set("Content-Type", "application/x-javascript");
    return response;
  } else if (url.pathname.startsWith("/api")) {
    // Forward API
    response = await fetch(url.toString(), {
      body: url.pathname.startsWith("/api/v3/getPublicPageData")
        ? null
        : request.body,
      headers: {
        "content-type": "application/json;charset=UTF-8",
        "user-agent":
          "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36",
      },
      method: "POST",
    });
    response = new Response(response.body, response);
    response.headers.set("Access-Control-Allow-Origin", "*");
    return response;
  } else if (slugs.indexOf(url.pathname.slice(1)) > -1) {
    const pageId = SLUG_TO_PAGE[url.pathname.slice(1)];
    return Response.redirect("https://" + MY_DOMAIN + "/" + pageId, 301);
  } else if (url.pathname.startsWith("/image/")) {
    request.headers["Referer"] = "https://blog.kali-team.cn/";
    response = await fetch(url.toString(), {
      headers: request.headers,
      method: request.method,
    });
    response = new Response(response.body, response);
    response.headers.delete("Content-Security-Policy");
    response.headers.delete("X-Content-Security-Policy");
    response.headers.delete("Referrer-Policy");
    response.headers.delete("X-Permitted-Cross-Domain-Policies");
    response.headers.set("Access-Control-Allow-Origin", "*");
    response.headers.set("Timing-Allow-Origin", "*");
    return response;
  } else {
    response = await fetch(url.toString(), {
      body: request.body,
      headers: request.headers,
      method: request.method,
    });
    response = new Response(response.body, response);
    response.headers.delete("Content-Security-Policy");
    response.headers.delete("X-Content-Security-Policy");
  }

  return appendJavascript(response, SLUG_TO_PAGE);
}
// 重写META
class MetaRewriter {
  element(element) {
    if (PAGE_TITLE !== "") {
      if (
        element.getAttribute("property") === "og:title" ||
        element.getAttribute("name") === "twitter:title"
      ) {
        element.setAttribute("content", PAGE_TITLE);
      }
      if (element.tagName === "title") {
        element.setInnerContent(PAGE_TITLE);
      }
    }
    if (PAGE_DESCRIPTION !== "") {
      if (
        element.getAttribute("name") === "description" ||
        element.getAttribute("property") === "og:description" ||
        element.getAttribute("name") === "twitter:description"
      ) {
        element.setAttribute("content", PAGE_DESCRIPTION);
      }
    }
    if (
      element.getAttribute("property") === "og:url" ||
      element.getAttribute("name") === "twitter:url"
    ) {
      element.setAttribute("content", MY_DOMAIN);
    }
    if (element.getAttribute("name") === "apple-itunes-app") {
      element.remove();
    }
  }
}
// 重写请求头
class HeadRewriter {
  element(element) {
    if (GOOGLE_FONT !== "") {
      element.append(
        `<link href="https://fonts.googleapis.com/css?family=${GOOGLE_FONT.replace(
          " ",
          "+"
        )}:Regular,Bold,Italic&display=swap" rel="stylesheet">
        <style>* { font-family: "${GOOGLE_FONT}" !important; }</style>`,
        {
          html: true,
        }
      );
    }
    // 隐藏工具栏
    element.append(
      `<link rel=alternate type=application/rss+xml href=https://${MY_DOMAIN}/index.xml title=${PAGE_DESCRIPTION}>
      <link rel=alternate type=application/rss+xml href=https://${MY_DOMAIN}/index.xml title=${PAGE_DESCRIPTION}>
      <style>
      div.notion-topbar > div > div:nth-child(3) { display: none !important; }
      // div.notion-topbar > div > div:nth-child(4) { display: none !important; }
      div.notion-topbar > div > div:nth-child(5) { display: none !important; }
      div.notion-topbar > div > div:nth-child(6) { display: none !important; }
      div.notion-topbar > div > div:nth-child(7) { display: none !important; }
      div.notion-topbar > div > div:nth-child(8) { display: none !important; }
      div.notion-topbar-mobile > div:nth-child(3) { display: none !important; }
      div.notion-topbar-mobile > div:nth-child(4) { display: none !important; }
      div.notion-topbar > div > div:nth-child(1n).toggle-mode { display: block !important; }
      div.notion-topbar-mobile > div:nth-child(1n).toggle-mode { display: block !important; }
      </style>`,
      {
        html: true,
      }
    );
  }
}
// 重写正文
class BodyRewriter {
  constructor(SLUG_TO_PAGE) {
    this.SLUG_TO_PAGE = SLUG_TO_PAGE;
    this.HTTP_BLACK_LIST = HTTP_BLACK_LIST;
  }
  element(element) {
    element.append(
      `<div>Powered by <a href="https://blog.kali-team.cn">Kali-Team</a></div>
      <script>
      const HTTP_BLACK_LIST = ${JSON.stringify(this.HTTP_BLACK_LIST)};
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
      window.CONFIG.domainBaseUrl = 'https://${MY_DOMAIN}';
      const SLUG_TO_PAGE = ${JSON.stringify(this.SLUG_TO_PAGE)};
      const PAGE_TO_SLUG = {};
      const slugs = [];
      const pages = [];
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
      let redirected = false;
      Object.keys(SLUG_TO_PAGE).forEach(slug => {
        const page = SLUG_TO_PAGE[slug];
        slugs.push(slug);
        pages.push(page);
        PAGE_TO_SLUG[page] = slug;
      });
      function getPage() {
        return location.pathname.slice(-32);
      }
      function remove_notion_page_content(){
        let scroll_bar = document.getElementsByClassName("notion-page-content");
        if (scroll_bar.length > 0){
          scroll_bar[0].style.paddingBottom = "0vh";
        }
        let iterable = [
          "html.notion-html body.notion-body div#notion-app div.notion-app-inner.notion-light-theme div.notion-cursor-listener div div.notion-frame div.notion-scroller.vertical div.pseudoSelection div",
          "html.notion-html body.notion-body div#notion-app div.notion-app-inner.notion-light-theme div.notion-cursor-listener div div.notion-frame div.notion-scroller.vertical div.pseudoSelection div div div",
          "html.notion-html body.notion-body div#notion-app div.notion-app-inner.notion-light-theme div.notion-cursor-listener div div.notion-frame div.notion-scroller.vertical div.pseudoSelection div div div div img",
          "html.notion-html body.notion-body div#notion-app div.notion-app-inner.notion-light-theme div.notion-cursor-listener div div.notion-frame div.notion-scroller.vertical div.pseudoSelection div div div div img"];
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
      function getSlug() {
        return location.pathname.slice(1);
      }
      function updateSlug() {
        const slug = PAGE_TO_SLUG[getPage()];
        if (slug != null) {
          history.replaceState(history.state, '', '/' + slug);
        }
        remove_notion_page_content();
      }
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
              toc_p.style.maxHeight = '50vh';
          }
      });
      }
      const observer = new MutationObserver(function() {
        remove_notion_page_content();
        TOC();
        if (redirected) return;
        const nav = document.querySelector('.notion-topbar');
        const mobileNav = document.querySelector('.notion-topbar-mobile');
        if (nav && nav.firstChild && nav.firstChild.firstChild
          || mobileNav && mobileNav.firstChild) {
          redirected = true;
          updateSlug();
          addDarkModeButton(nav ? 'web' : 'mobile');
          const onpopstate = window.onpopstate;
          window.onpopstate = function() {
            if (slugs.includes(getSlug())) {
              const page = SLUG_TO_PAGE[getSlug()];
              if (page) {
                history.replaceState(history.state, 'bypass', '/' + page);
              }
            }
            onpopstate.apply(this, [].slice.call(arguments));
            updateSlug();
          };
        }
      });
      observer.observe(document.querySelector('#notion-app'), {
        childList: true,
        subtree: true,
      });
      const replaceState = window.history.replaceState;
      window.history.replaceState = function(state) {
        if (arguments[1] !== 'bypass' && slugs.includes(getSlug())) return;
        return replaceState.apply(window.history, arguments);
      };
      const pushState = window.history.pushState;
      window.history.pushState = function(state) {
        const dest = new URL(location.protocol + location.host + arguments[2]);
        const id = dest.pathname.slice(-32);
        if (pages.includes(id)) {
          arguments[2] = '/' + PAGE_TO_SLUG[id];
        }
        return pushState.apply(window.history, arguments);
      };
      const open = window.XMLHttpRequest.prototype.open;
      window.XMLHttpRequest.prototype.open = function() {
        arguments[1] = arguments[1].replace('${MY_DOMAIN}', '${NOTION_DOMAIN}');
        return open.apply(this, [].slice.call(arguments));
      };
      remove_notion_page_content();
    </script>${CUSTOM_SCRIPT}`,
      {
        html: true,
      }
    );
  }
}

async function appendJavascript(res, SLUG_TO_PAGE) {
  return new HTMLRewriter()
    .on("title", new MetaRewriter())
    .on("meta", new MetaRewriter())
    .on("head", new HeadRewriter())
    .on("body", new BodyRewriter(SLUG_TO_PAGE))
    .transform(res);
}
