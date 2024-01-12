use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryBody {
    page: PageId,
    limit: u32,
    chunk_number: u32,
    vertical_columns: bool,
}

impl QueryBody {
    pub fn new(id: String) -> QueryBody {
        QueryBody {
            page: PageId { id },
            limit: 30,
            chunk_number: 0,
            vertical_columns: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageId {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryCollection {
    record_map: RecordMap,
}

impl QueryCollection {
    pub fn get_sitemap(&self) -> String {
        let mut sitemap = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
        for (_key, block) in self.record_map.block.iter() {
            sitemap.push_str(&block.value.get_page().unwrap_or_default());
        }
        sitemap.push_str("</urlset>");
        sitemap
    }
    pub fn get_title(&self, id: &uuid::Uuid) -> Option<String> {
        if let Some(block) = self.record_map.block.get(&id.to_string()) {
            return block.value.get_title();
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordMap {
    block: HashMap<String, Block>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    value: BlockEnum,
}

impl BlockEnum {
    fn get_page(&self) -> Option<String> {
        match self {
            BlockEnum::Page(p) => Some(p.to_loc()),
            _ => None,
        }
    }
    fn get_title(&self) -> Option<String> {
        match self {
            BlockEnum::Page(p) => Some(p.properties.title.get_title()),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockEnum {
    Page(Page),
    ColumnList(ColumnList),
    Column(Column),
    CollectionView(CollectionView),
    #[serde(other)]
    Divider,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionView {
    id: String,
    version: i32,
    view_ids: Vec<String>,
    #[serde(with = "date_format")]
    created_time: DateTime<FixedOffset>,
    #[serde(with = "date_format")]
    last_edited_time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Column {
    id: String,
    version: i32,
    content: Vec<String>,
    #[serde(with = "date_format")]
    created_time: DateTime<FixedOffset>,
    #[serde(with = "date_format")]
    last_edited_time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnList {
    id: String,
    version: i32,
    content: Vec<String>,
    #[serde(with = "date_format")]
    created_time: DateTime<FixedOffset>,
    #[serde(with = "date_format")]
    last_edited_time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    id: String,
    version: i32,
    properties: Properties,
    #[serde(with = "date_format")]
    created_time: DateTime<FixedOffset>,
    #[serde(with = "date_format")]
    last_edited_time: DateTime<FixedOffset>,
}

impl Page {
    fn to_loc(&self) -> String {
        format!("<url>\n<loc>https://MY_DOMAIN/{}</loc>\n\t<lastmod>{}</lastmod>\n\t<changefreq>daily</changefreq>\n\t<priority>0.9</priority>\n</url>\n",
                self.id.replace('-', ""),
                self.created_time.format("%Y-%m-%d"),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    title: Title,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Title {
    String(String),
    Array(Vec<Title>),
}

impl Title {
    fn get_title(&self) -> String {
        match self {
            Title::String(t) => t.to_string(),
            Title::Array(ts) => {
                return ts
                    .iter()
                    .map(|t| t.get_title())
                    .collect::<Vec<String>>()
                    .join("");
            }
        }
    }
}

mod date_format {
    use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i64::deserialize(deserializer)?;
        let tz_offset = FixedOffset::east_opt(8 * 60 * 60).unwrap();
        match NaiveDateTime::from_timestamp_millis(s) {
            Some(t) => Ok(DateTime::from_utc(t, tz_offset)),
            None => Ok(Utc::now().with_timezone(&tz_offset)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::page::QueryCollection;

    #[test]
    fn test_notion_json() {
        let j = r#"
        {
	"recordMap": {
		"__version__": 3,
		"block": {
			"0b976b99-3c87-4015-98be-4caab8cbe0cd": {
				"spaceId": "d4aa424b-d5f8-4dc3-a0fb-e5270f17203e",
				"value": {
					"value": {
						"id": "0b976b99-3c87-4015-98be-4caab8cbe0cd",
						"version": 187,
						"type": "page",
						"properties": {
							":H`m": [
								["Yes"]
							],
							"L:TS": [
								["Done"]
							],
							"[[=B": [
								["Metaploit"]
							],
							"title": [
								["Metasploit模块获取"],
								["MobaXterm", [
									["b"]
								]],
								["密码"]
							],
							"04eb09ec-4c31-4735-818c-8e602680d327": [
								["Post"]
							],
							"0fe81fdb-1b1d-479c-8039-3c4c116aca43": [
								["Metasploit,安全开发"]
							],
							"6545e944-c1af-4173-a142-1731e21b5432": [
								["Published"]
							]
						},
						"content": ["3fb0904e-25b2-44b1-8ddd-f760e5f8e4e4", "07f2ca80-4814-4da3-a1c7-bb8c5458d716"],
						"format": {
							"page_icon": "🩸",
							"page_cover": "/images/page-cover/webb3.jpg",
							"page_full_width": true,
							"page_cover_position": 0.5
						},
						"permissions": [{
							"role": "editor",
							"type": "user_permission",
							"user_id": "69dbb335-a0bf-4008-b7db-011bb29d1b5a"
						}],
						"created_time": 1662392717059,
						"last_edited_time": 1694400266071,
						"parent_id": "52de4e5e-ba6e-46a2-9dc5-5581637cf339",
						"parent_table": "collection",
						"alive": true,
						"ignore_block_count": true,
						"created_by_table": "notion_user",
						"created_by_id": "69dbb335-a0bf-4008-b7db-011bb29d1b5a",
						"last_edited_by_table": "notion_user",
						"last_edited_by_id": "69dbb335-a0bf-4008-b7db-011bb29d1b5a",
						"space_id": "d4aa424b-d5f8-4dc3-a0fb-e5270f17203e"
					},
					"role": "reader"
				}
			},
			"edb6a939-baab-4424-a25f-d295b3c51312": {
				"spaceId": "d4aa424b-d5f8-4dc3-a0fb-e5270f17203e",
				"value": {
					"value": {
						"id": "edb6a939-baab-4424-a25f-d295b3c51312",
						"version": 1193,
						"type": "page",
						"properties": {
							"title": [
								["三米前有蕉皮的博客"]
							]
						},
						"content": ["4ca8f99f-273c-4eba-89e2-8fd214b5b854", "4d2af9ce-0e2c-494c-a244-583612e708c8"],
						"format": {
							"page_icon": "🎯",
							"page_cover": "/images/page-cover/met_william_morris_1877_willow.jpg",
							"block_locked": false,
							"block_locked_by": "69dbb335-a0bf-4008-b7db-011bb29d1b5a",
							"page_full_width": true,
							"page_small_text": false,
							"copied_from_pointer": {
								"id": "99616497-0e5a-46ee-bd1e-7e4f37b7f999",
								"table": "block",
								"spaceId": "c1725102-1d6c-4016-bc15-909a98f67c7e"
							},
							"page_cover_position": 0.1,
							"page_section_visibility": {
								"comments": "section_show",
								"backlinks": "section_collapsed",
								"margin_comments": "inline"
							}
						},
						"permissions": [{
							"role": "editor",
							"type": "user_permission",
							"user_id": "69dbb335-a0bf-4008-b7db-011bb29d1b5a"
						}, {
							"role": "reader",
							"type": "public_permission",
							"added_timestamp": 1674656171258,
							"allow_duplicate": false
						}],
						"created_time": 1656257999436,
						"last_edited_time": 1694401675037,
						"parent_id": "80b3d42d-5991-4bb8-b4ab-9b4a2a5f428d",
						"parent_table": "team",
						"alive": true,
						"copied_from": "99616497-0e5a-46ee-bd1e-7e4f37b7f999",
						"file_ids": ["23f4ac2b-70b7-40dd-8317-f82f05533be0", "f4041e2a-f393-48d1-9558-77010c40bece", "a91c02bc-f962-4539-bf43-48176973123d", "ec7ef1c2-a996-47ee-acda-2dec5cecfccf", "4a8040cd-4b55-4d93-b5d2-e9f083473a9a", "7325ad45-3d2e-4f92-877e-596a36fee721", "7b93bd19-88f0-44e4-a676-bf917f540620", "70e51722-2eb2-49f4-a95d-476a7feaf0ba", "d890bae5-2af6-4f0c-9edc-ebe85a86dfa7", "b8e31bfb-082e-4300-929f-6176bf671186", "1705211d-b001-4eb3-b4be-bd85656c676b", "b5f9fa60-722c-44c7-aaab-bd288a70725b", "39f31e97-1040-4450-891c-fd53a5906560", "a4e3a2fa-c662-4ae7-bc59-b590f004bce6", "09628f5f-0a40-45e7-9092-a3beac1d894a", "cf4daf0e-2746-4363-93e2-20b20057a8bb", "a90cbff0-aa90-4767-bd2a-dcce842cc298", "ad2fd2f0-28b5-458b-bda9-02d0df79becb", "ed5300ca-ac5a-40cd-b51a-8a54d9827cfe"],
						"created_by_table": "notion_user",
						"created_by_id": "69dbb335-a0bf-4008-b7db-011bb29d1b5a",
						"last_edited_by_table": "notion_user",
						"last_edited_by_id": "69dbb335-a0bf-4008-b7db-011bb29d1b5a",
						"space_id": "d4aa424b-d5f8-4dc3-a0fb-e5270f17203e"
					},
					"role": "reader"
				}
			}
		}
	}
}"#;
        let p: QueryCollection = serde_json::from_str(j).unwrap();
        for (_key, block) in p.record_map.block {
            println!("{}", block.value.get_page().unwrap_or_default());
        }
    }
}
