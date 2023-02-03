/* CONFIGURATION STARTS HERE */

/* Step 1: enter your domain name like fruitionsite.com */
const MY_DOMAIN = "blog.kali-team.cn";

/*
 * Step 2: enter your URL slug to page ID mapping
 * The key on the left is the slug (without the slash)
 * The value on the right is the Notion page ID
 */
const SLUG_TO_PAGE = {
  "": "edb6a939baab4424a25fd295b3c51312",
  links: "9c74faba0b14441a93c2f94a40da3f79",
};

/* Step 3: enter your page title and description for SEO purposes */
const PAGE_TITLE = "Kali-Team";
const PAGE_DESCRIPTION = "三米前有蕉皮";

/* Step 4: enter a Google Font name, you can choose from https://fonts.google.com */
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
async function generateSitemap() {
  let sitemap =
    '<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n';
  slugs.forEach(
    (slug) => (sitemap += makeLoc(slug, new Date().toJSON().slice(0, 10)))
  );
  response = await fetch(
    "https://kali-team.notion.site/api/v3/queryCollection?src=reset",
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
      body: '{"source":{"type":"collection","id":"52de4e5e-ba6e-46a2-9dc5-5581637cf339","spaceId":"d4aa424b-d5f8-4dc3-a0fb-e5270f17203e"},"collectionView":{"id":"a5b688dd-2876-4f80-a47d-d84e713ac56e","spaceId":"d4aa424b-d5f8-4dc3-a0fb-e5270f17203e"},"loader":{"type":"reducer","reducers":{"collection_group_results":{"type":"results","limit":50},"table:uncategorized:title:unique":{"type":"aggregation","aggregation":{"property":"title","aggregator":"unique"}},"table:uncategorized:|oXv:latest_date":{"type":"aggregation","aggregation":{"property":"|oXv","aggregator":"latest_date"}},"table:uncategorized:L:TS:[object Object]":{"type":"aggregation","aggregation":{"property":"L:TS","aggregator":{"operator":"percent_per_group","groupName":"Complete"}}}},"sort":[{"property":"|oXv","direction":"descending"}],"searchQuery":"","userTimeZone":"Asia/Shanghai"}}',
      method: "POST",
    }
  );
  let body = await response.json();
  let blocks = body.recordMap.block;
  for (const [k, block] of Object.entries(blocks)) {
    let properties = block.value.properties;
    let page_id = k.replaceAll("-", "");
    if (properties !== undefined && properties.title.length === 1) {
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

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods":
    "GET,POST,PUT,PATCH,TRACE,DELETE,HEAD,OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type",
  "Access-Control-Allow-Credentials": "True",
  "Access-Control-Max-Age": "1728000",
};

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

async function fetchAndApply(request) {
  console.log(request.toString());
  if (request.method === "OPTIONS") {
    return handleOptions(request);
  }
  let url = new URL(request.url);
  console.log(url.toString());
  url.hostname = "kali-team.notion.site";
  if (url.pathname === "/robots.txt") {
    return new Response("Sitemap: https://" + MY_DOMAIN + "/sitemap.xml");
  }
  if (url.pathname === "/baidu_verify_codeva-MTR3VJrJMr.html") {
    return new Response("ef43aea0ad35bab36475b00311de3662");
  }
  if (url.pathname === "/sitemap.xml") {
    let response = new Response(await generateSitemap());
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
        .replace(/kali-team.notion.site/g, MY_DOMAIN)
        .replace(/kali-team.notion.site/g, MY_DOMAIN)
        .replace(/exp.blog.kali-team.cn/g, "exp.notion.so"),
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
      `<style>
      div.notion-topbar > div > div:nth-child(3) { display: none !important; }
      // div.notion-topbar > div > div:nth-child(4) { display: none !important; }
      div.notion-topbar > div > div:nth-child(5) { display: none !important; }
      div.notion-topbar > div > div:nth-child(6) { display: none !important; }
      div.notion-topbar > div > div:nth-child(7) { display: none !important; }
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

class BodyRewriter {
  constructor(SLUG_TO_PAGE) {
    this.SLUG_TO_PAGE = SLUG_TO_PAGE;
  }
  element(element) {
    element.append(
      `<div>Powered by <a href="https://blog.kali-team.cn">Kali-Team</a></div>
      <script src="https://zz.bdstatic.com/linksubmit/push.js"></script>
      <script>
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
        arguments[1] = arguments[1].replace('${MY_DOMAIN}', 'kali-team.notion.site');
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
