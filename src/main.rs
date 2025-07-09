mod crawler;
mod storage;
mod graph;
mod filter;
mod types;

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use dashmap::DashSet;
use types::CrawlTask;
use graph::{SiteGraph, write_dot};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);
    let visited = Arc::new(DashSet::new());
    let graph = Arc::new(SiteGraph::new());
    let sem = Arc::new(Semaphore::new(10));

    tx.send(CrawlTask {
        url: "https://www.rust-lang.org".into(),
        depth: 2,
    }).await.unwrap();

    crawler::start_crawling(rx, tx.clone().into(), visited, sem, graph.clone()).await;

    write_dot(&graph, "site.dot").expect("write dot file failed");
    println!("✅ crawler finished，site graph generated！");
}