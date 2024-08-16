use dioxus::prelude::*;
use dioxus_logger::tracing;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const NODE_UNCONFIRMED_TRANSACTIONS: &str =
    "http://api.sigmamining.xyz/transactions/unconfirmed?limit=10&offset=0";

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub token_id: String,
    pub amount: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub box_id: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub box_id: String,
    pub value: u32,
    pub assets: Vec<Asset>,
    pub creation_height: u32,
    pub tx_id: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnconfirmedTxs {
    pub id: String,
    pub inputs: Vec<Input>,
    pub data_inputs: Vec<String>,
    pub outputs: Vec<Output>,
    pub size: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorUnconfirmedTxs {
    node_array: Value,
    pub unconfirmed_txs: Vec<UnconfirmedTxs>,
}

impl VectorUnconfirmedTxs {
    pub async fn default() -> VectorUnconfirmedTxs {
        VectorUnconfirmedTxs {
            node_array: Value::default(),
            unconfirmed_txs: Vec::default(),
        }
    }

    pub async fn get_data(&mut self) -> Result<(), ServerFnError> {
        self.api_fetch_unconfirmed_transactions().await?;
        self.process_api_data().await?;
        Ok(())
    }

    async fn api_fetch_unconfirmed_transactions(&mut self) -> Result<(), ServerFnError> {
        self.node_array = Client::new()
            .get(format!("{}", NODE_UNCONFIRMED_TRANSACTIONS))
            .send()
            .await?
            .json()
            .await?;
        Ok(())
    }
    async fn process_api_data(&mut self) -> Result<(), ServerFnError> {
        if let serde_json::Value::Array(unconfirmed_array) = self.node_array.clone() {
            for transaction in unconfirmed_array {
                /* Store Transaction ID in self.id */
                let id = transaction["id"].as_str().unwrap().to_string();

                /* Store transaction inputs in self.inputs */
                let mut inputs_vec = vec![];

                match transaction["inputs"].as_array() {
                    Some(inputs) => {
                        for input in inputs {
                            inputs_vec.push(Input {
                                box_id: input["boxId"].as_str().unwrap().to_string(),
                            })
                        }
                    }
                    None => (),
                }

                /* Store dataInputs in self.data_inputs */
                let mut data_input_vec = vec![];
                match transaction["dataInputs"].as_array() {
                    Some(data_inputs) => {
                        for data_input in data_inputs {
                            data_input_vec.push(data_input["boxId"].as_str().unwrap().to_string());
                        }
                    }
                    None => (),
                }

                /* Store transaction outputs in self.outputs */
                let mut outputs_vec = vec![];
                for outputs in transaction["outputs"].as_array().unwrap() {
                    outputs_vec.push(Output {
                        box_id: {
                            match outputs["boxId"].as_str() {
                                Some(box_id) => box_id.to_string(),
                                None => String::new(),
                            }
                        },
                        value: outputs["value"].as_u64().unwrap() as u32,

                        assets: {
                            let mut assets_vec = Vec::new();
                            for assets in outputs["assets"].as_array().unwrap() {
                                assets_vec.push(Asset {
                                    token_id: {
                                        match assets["tokenId"].as_str() {
                                            Some(token_id) => token_id.to_string(),
                                            None => String::new(),
                                        }
                                    },
                                    amount: {
                                        match assets["amount"].as_u64() {
                                            Some(amount) => amount as u32,
                                            None => 0,
                                        }
                                    },
                                });
                            }
                            /* Return Vec<Assets> */
                            assets_vec
                        },

                        creation_height: {
                            match outputs["creationHeight"].as_u64() {
                                Some(creation_height) => creation_height as u32,
                                None => 0,
                            }
                        },
                        tx_id: {
                            match outputs["transactionId"].as_str() {
                                Some(tx_id) => tx_id.to_string(),
                                None => String::new(),
                            }
                        },
                    })
                }

                /* Store transaction size in self.size */
                let size = match transaction["size"].as_u64() {
                    Some(size) => size as u32,
                    None => 0,
                };

                self.unconfirmed_txs.push(UnconfirmedTxs {
                    id: id,
                    inputs: inputs_vec,
                    data_inputs: data_input_vec,
                    outputs: outputs_vec,
                    size: size,
                })
            }
        }
        Ok(())
    }
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<VectorUnconfirmedTxs, ServerFnError> {
    let mut data = VectorUnconfirmedTxs::default().await;
    data.get_data().await?;

    // tracing::info!("Data processed: {:?}", data.node_array);
    Ok(data)
}
