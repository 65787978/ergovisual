use dioxus::prelude::*;
use dioxus_logger::tracing;
use reqwest::Client;
use serde_json::Value;

const NODE_UNCONFIRMED_TRANSACTIONS: &str =
    "http://api.sigmamining.xyz/transactions/unconfirmed?limit=10&offset=0";
struct Asset {
    pub token_id: String,
    pub amount: u32,
}
struct Input {
    pub box_id: String,
}
struct Output {
    pub box_id: String,
    pub value: u32,
    pub assets: Vec<Asset>,
    pub creation_height: u32,
    pub tx_id: String,
}
struct Neshto {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl Neshto {
    pub async fn default() -> Neshto {
        Neshto {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub async fn get_data(&mut self, block_height: u32) -> Result<(), ServerFnError> {
        Ok(())
    }

    async fn api_get_header_id(&mut self) -> Result<(), ServerFnError> {
        let api_header_id: serde_json::Value = Client::new()
            .get(format!("{}/blocks/at/{}", NODE_API_URL, self.block_height))
            .send()
            .await?
            .json()
            .await?;

        self.header_id = api_header_id[0].as_str().unwrap().to_string();
        Ok(())
    }

    async fn api_get_block_info(&mut self) -> Result<(), ServerFnError> {
        let block_transactions: serde_json::Value = Client::new()
            .get(format! {"{}/blocks/{}", NODE_API_URL, self.header_id})
            .send()
            .await?
            .json()
            .await?;

        Ok(())
    }
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    let header_ids: serde_json::Value = Client::new()
        .get("http://api.sigmamining.xyz/blocks/at/1330050")
        .send()
        .await?
        .json()
        .await?;

    tracing::info!("Server received: {:?}", header_ids);
    let header_string = header_ids[0].as_str().unwrap().to_string();
    Ok(header_string)
}
