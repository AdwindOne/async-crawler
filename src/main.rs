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

    drop(tx.clone());
    crawler::start_crawling(rx, tx.clone().into(), visited, sem, graph.clone()).await;

    drop(tx);
    write_dot(&graph, "site.dot").expect("写入 dot 文件失败");
    println!("✅ 爬虫结束，结构图已生成！");
}