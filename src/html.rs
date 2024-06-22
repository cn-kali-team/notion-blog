pub const _REWRITER_HTTP: &str = r#"
    <script>
        const HTTP_BLACK_LIST = {
          "https://exp.notion.so/": "",
          "https://http-inputs-notion.splunkcloud.com/": "",
          "https://msgstore.www.notion.so/": "",
          "https://o324374.ingest.sentry.io/": "",
          "/api/v3/trackSegmentEvent": "{}",
          "/api/v3/ping": "{}",
          "/f/refresh":"",
          "/api/v3/getAssetsJsonV2":"{}",
          "https://statsigapi.net/v1/sdk_exception":"",
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
        // console.log(resource, config);
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
pub const BASE: &str = r#"
    <div>Powered by <a href="https://github.com/cn-kali-team/notion-blog">Kali-Team</a></div>
      <script>
      localStorage.__console = true;
      window.CONFIG.domainBaseUrl = location.origin;
      function waitForElementToExist(selector) {
        return new Promise(resolve => {
          if (document.querySelector(selector)) {
            return resolve(document.querySelector(selector));
          }
          const observer = new MutationObserver(function(mutationsList, observer) {
            if (document.querySelector(selector)) {
              resolve(document.querySelector(selector));
              observer.disconnect();
            }
          });
          observer.observe(document.body, {
            subtree: true,
            childList: true,
          });
        });
      };
    </script>"#;
pub const RESIZE: &str = r#"
    <script>
      function remove_notion_page_content(){
        let scroll_bar = document.querySelector(".notion-page-content");
        if (scroll_bar){
          scroll_bar.style.paddingBottom = "0vh";
        }
        let iterable = [
          "div.pseudoSelection div",
          "div.pseudoSelection div div div",
          "div.pseudoSelection div div div div img",
        ];
        for (const entry of iterable) {
          let pseudo_selection = document.querySelector(entry);
          if (pseudo_selection !== null){
            pseudo_selection.style.height = "8vh";
          }
        }
        let layout = document.querySelector(".layout");
        if (layout != null){
          layout.style.paddingBottom="2vh";
        }
        let notion_page_controls = document.querySelector("div.pseudoSelection div.notion-page-controls");
        if (notion_page_controls !== null){
          notion_page_controls.remove()
        }
        let notranslate = document.querySelector("div.pseudoSelection div.notion-record-icon.notranslate");
        let imgNode = document.querySelector(".vertical > div:nth-child(2) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > img:nth-child(1)");

        if (notranslate !== null && imgNode !== null){
            notranslate.style.marginTop="-36px";
        }
      }
      const page_observe = new MutationObserver(function(mutationsList, observer) {
        remove_notion_page_content();
        update_history();
      });
      page_observe.observe(document.querySelector('#notion-app'), {
        childList: true,
        subtree: true,
      });
    </script>"#;
pub const THEME: &str = r#"
    <script>
      const el = document.createElement('div');
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
      waitForElementToExist('.shadow-cursor-breadcrumb').then((el)=>{
        const nav = document.querySelector('.notion-topbar');
        const mobileNav = document.querySelector('.notion-topbar-mobile');
        if ((nav && nav.firstChild && nav.firstChild.firstChild) || (mobileNav && mobileNav.firstChild)) {
          addDarkModeButton(nav ? 'web' : 'mobile');
        }
      });
    </script>"#;
pub const HEAD: &str = r#"
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
