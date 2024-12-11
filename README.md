## Deploy

[![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/cn-kali-team/notion-blog)

- wrangler.toml

```toml
name = "blog" #your cloudflare worker's name
main = "build/worker/shim.mjs"
compatibility_date = "2023-03-22"
routes = [{ pattern = "blog.kali-team.cn/*", zone_id = "28eeca2e15ad32083050e97611262653" }] # your domain and zone_id
usage_model = "bundled"
[build]
command = "cargo install -q worker-build && worker-build --release"

[vars]
MY_DOMAIN = "blog.kali-team.cn" # change me
NOTION_DOMAIN = "kali-team.notion.site" # change me
PAGE_MAP = '{"/":"edb6a939baab4424a25fd295b3c51312","/links":"9c74faba0b14441a93c2f94a40da3f79","/donate":"eb55bc48f7fb42bfaef8473d2b7b63aa"}'
COMMENT_MAP = '{"data-repo":"cn-kali-team/notion-blog","data-repo-id":"R_kgDOI1wUgQ","data-category-id":"DIC_kwDOI1wUgc4CZK9O","data-mapping":"title","data-input-position":"top","data-theme":"preferred_color_scheme","data-lang":"zh-CN"}'
PAGE_TITLE = "Kali-Team"
PAGE_DESCRIPTION = "三米前有蕉皮"
ICON_URL = "https://avatars.githubusercontent.com/u/30642514?v=4"
QUERY_BODY = '{"collectionView":{"id":"a5b688dd-2876-4f80-a47d-d84e713ac56e","spaceId":"d4aa424b-d5f8-4dc3-a0fb-e5270f17203e"},"source":{"type":"collection","id":"52de4e5e-ba6e-46a2-9dc5-5581637cf339","spaceId":"d4aa424b-d5f8-4dc3-a0fb-e5270f17203e"},"loader":{"type":"reducer","reducers":{"collection_group_results":{"type":"results","limit":10},"table:uncategorized:title:count":{"type":"aggregation","aggregation":{"property":"title","aggregator":"count"}},"table:uncategorized:[[=B:unique":{"type":"aggregation","aggregation":{"property":"[[=B","aggregator":"unique"}},"table:uncategorized:enzw:date_range":{"type":"aggregation","aggregation":{"property":"enzw","aggregator":"date_range"}}},"sort":[{"property":"enzw","direction":"descending"}],"searchQuery":"","userTimeZone":"Asia/Shanghai"}}'

[placement]
mode = "smart"
```
## wrangler.toml

### name

- This is the worker name you created for yourself
- My cloudflare worker's name is `blog`,you can open your workers link `https://dash.cloudflare.com/Account_ID/Domain/workers`

### routes

- This subdomain routing trigger bound by your worker
- open `https://dash.cloudflare.com/Account_ID/workers/services/view/WORKER_NAME/production/triggers`
- My `routes` pattern is `blog.kali-team.cn/*`
![routes](doc/routes.png)
- My `zone_id` is `28eeca2e15ad32083050e97611262653`
![zone_id](doc/zone_id.png)

### var

- How to modify a configuration file.
- `PAGE_MAP`: routing and page id,like: `/` => `https://kali-team.notion.site/edb6a939baab4424a25fd295b3c51312`
- `COMMENT_MAP`:comment system, open [giscus](https://giscus.app/),After completing the guided tutorial,You will
  receive:

```html

<script src="https://giscus.app/client.js"
        data-repo="cn-kali-team/notion-blog"
        data-repo-id="R_kgDOI1wUgQ"
        data-category="Announcements"
        data-category-id="DIC_kwDOI1wUgc4CZK9O"
        data-mapping="title"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="zh-CN"
        crossorigin="anonymous"
        async>
</script>
```

- Convert JavaScript attributes to JSON

```json
{
  "data-repo": "cn-kali-team/notion-blog",
  "data-repo-id": "R_kgDOI1wUgQ",
  "data-category-id": "DIC_kwDOI1wUgc4CZK9O",
  "data-mapping": "title",
  "data-input-position": "top",
  "data-theme": "preferred_color_scheme",
  "data-lang": "zh-CN"
}
```

- One line string:

```
'{"data-repo":"cn-kali-team/notion-blog","data-repo-id":"R_kgDOI1wUgQ","data-category-id":"DIC_kwDOI1wUgc4CZK9O","data-mapping":"title","data-input-position":"top","data-theme":"preferred_color_scheme","data-lang":"zh-CN"}'
```

### GitHub Action

- Fork repository to your accounts
- Edit your own `wrangler.toml` file.
- [wrangler-action](https://github.com/cloudflare/wrangler-action/)
- Open [Create Token](https://dash.cloudflare.com/profile/api-tokens) on cloudflare
- Use template `Edit Cloudflare Workers` -> Edit Option -> Continue to summary
- Copy Token and remember it
- Open `https://github.com/xxx/notion-blog/settings/secrets/actions/new` and create a Name `CF_API_TOKEN`
- Secret:`Your Cloudflare Token`

### Manual

- install cloudflare [wrangler](https://github.com/cloudflare/workers-rs)
- [Rust WebAssembly guide](https://developers.cloudflare.com/workers/runtime-apis/webassembly/rust/)
- wrangler login

```bash
npx wrangler login
```

- deploy to cloudflare

```bash
npx wrangler deploy
```

## Notion Blog

- 原来的[notion](https://kali-team.notion.site/edb6a939baab4424a25fd295b3c51312)页面。
- 必须满足的条件：有一个notion的免费帐号，有一个域名，有一个cloudflare免费帐号。
- 详细步骤请看：[https://blog.kali-team.cn/95ab328cb39041709231abf7b56ac8fc](https://blog.kali-team.cn/95ab328cb39041709231abf7b56ac8fc)

## Notion Blog

- Original [notion](https://kali-team.notion.site/edb6a939baab4424a25fd295b3c51312) page.
- Must be met: Have a notion free account, have a domain name, and have a [cloudflare](https://workers.cloudflare.com/)
  free account.
- For detailed steps, please see: [https://blog.kali-team.cn/95ab328cb39041709231abf7b56ac8fc](https://blog.kali-team.cn/95ab328cb39041709231abf7b56ac8fc)
