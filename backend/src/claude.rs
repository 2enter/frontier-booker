use serde::Deserialize;
use serde_json::{json, Value};

use reqwest::header::{HeaderMap, HeaderValue};

const PROMPT: &str  = "
 	<description>
  	這是一個寄往想像中未來外太空貿易站的貨物，收件人與寄件人可能是包含地球在內的任何外星生命，生成一段 120 字以內、不分段、用字通俗易懂的內容物說明，並為該物資取名。該說明將會收錄進一本太空物資圖鑑中。
    請注意：你的回應必須完全符合格式要求，並使用繁體中文、避免使用中國用語，只包含名稱和說明，中間用%%%分隔，不要有任何其他文字。
	</description>

	<output>
		{{名稱}}%%%{{說明}}
	</output>
";

#[derive(Deserialize, Debug)]
struct ContentBlock {
    text: String,
}

#[derive(Deserialize, Debug)]
struct ClaudeResponse {
    content: Vec<ContentBlock>,
}

fn _gen_message_params(base64_image: &str) -> Value {
    json!({
        "model": "claude-3-5-sonnet-latest",
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": [
                        {
                            "type": "text",
                            "text": PROMPT
                        },
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": "image/jpeg",
                            "data": base64_image
                        }
                    }
                ]
            }
        ]
    })
}

pub async fn gen_cargo_text_info(
    api_key: &str,
    image_base64_url: &str,
) -> Result<(String, String), reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", HeaderValue::from_str(api_key).unwrap());
    headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
    headers.insert("content-type", HeaderValue::from_static("application/json"));

    // Construct the request body with multimodal content
    let request_body = json!({
        "model": "claude-3-5-sonnet-latest",
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": [
                        {
                            "type": "text",
                            "text": PROMPT
                        },
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": "image/jpeg",
                            "data": image_base64_url
                        }
                    }
                ]
            }
        ]
    });

    // Create client and send request
    let client = reqwest::Client::new();

    let response: ClaudeResponse = client
        .post("https://api.anthropic.com/v1/messages")
        .headers(headers)
        .json(&request_body)
        .send()
        .await?
        .json()
        .await?;

    // println!("{response:#?}");

    let text = response.content[0].text.clone();
    let splits: Vec<&str> = text.split("%%%").collect();
    Ok((splits[0].to_owned(), splits[1].to_owned()))
}
