## Deploy

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
INDEX_PAGE_ID = "edb6a939baab4424a25fd295b3c51312" # your original notion page's id
LINK_PAGE_ID = "9c74faba0b14441a93c2f94a40da3f79" # some page
DONATE_PAGE_ID = "eb55bc48f7fb42bfaef8473d2b7b63aa" # some page 
PAGE_TITLE = "Kali-Team"
PAGE_DESCRIPTION = "三米前有蕉皮"

[placement]
mode = "smart"
```

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
- 详细步骤请看：https://blog.kali-team.cn/95ab328cb39041709231abf7b56ac8fc

## Notion Blog

- Original [notion](https://kali-team.notion.site/edb6a939baab4424a25fd295b3c51312) page.
- Must be met: Have a notion free account, have a domain name, and have a [cloudflare](https://workers.cloudflare.com/)
  free account.
- For detailed steps, please see: https://blog.kali-team.cn/95ab328cb39041709231abf7b56ac8fc