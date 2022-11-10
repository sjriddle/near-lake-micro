use std::collections::hash_map::RandomState;
use std::str::FromStr;

use std::collections::{HashMap, HashSet};

use clap::Parser;
use near_lake_framework::near_indexer_primitives::views::BlockView;
use serde_json::value::Index;
use tokio::sync::mpsc;
use tracing::info;

use near_lake_framework::near_indexer_primitives::{self, IndexerShard};
use near_lake_framework::LakeConfig;

use configs::{init_logging, Opts};

mod configs;

pub struct StreamResult {
    pub block: BlockView,
    pub index_shards: Vec<IndexerShard>,
}

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    init_logging();

    let opts: Opts = Opts::parse();

    let config: LakeConfig = opts.clone().into();

    let (_, stream) = near_lake_framework::streamer(config);

    listen_blocks(stream).await;

    Ok(())
}

/// The main listener function the will be reading the stream of blocks `StreamerMessage`
/// and perform necessary checks
async fn listen_blocks(
    mut stream: mpsc::Receiver<near_indexer_primitives::StreamerMessage>,
) {
    eprintln!("listen_blocks");
    // This will be a map of correspondence between transactions and receipts
    // let mut stream_result  = StreamResult::new();
    // This will be a list of receipt ids we're following
    let mut block_shards: Vec<IndexerShard> = Vec::new();
    // let mut output: HashSet<StreamResult> = HashSet::new();


    // Boilerplate code to listen the stream
    while let Some(streamer_message) = stream.recv().await {
        eprintln!("Streamer Message: {:#?}", streamer_message);
        
        for shard in streamer_message.shards {
            block_shards.push(shard);
        }

        eprintln!(" Block: {:#?}", streamer_message.block);
        eprintln!(" Shards: {:#?}", block_shards);
    }
}
