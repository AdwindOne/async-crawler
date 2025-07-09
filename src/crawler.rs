use tokio::{sync::{mpsc, Semaphore}};
use std::sync::Arc;
use dashmap::DashSet;
use crate::{storage, filter, graph::SiteGraph, types::CrawlTask};
use crate::graph::write_dot;
use rand::{thread_rng, Rng};
use tokio::time::{sleep, Duration};

pub async fn start_crawling(
    mut rx: mpsc::Receiver<CrawlTask>,
    // tx: mpsc::Sender<CrawlTask>,
    tx: Arc<mpsc::Sender<CrawlTask>>,
    visited: Arc<DashSet<String>>,
    sem: Arc<Semaphore>,
    graph: Arc<SiteGraph>,
) {
    while let Some(task) = rx.recv().await {
        let permit = sem.clone().acquire_owned().await.unwrap();

        if visited.contains(&task.url) || task.depth == 0 {
            drop(permit);
            continue;
        }

        visited.insert(task.url.clone());
        // let mut rng = thread_rng();
        // let delay_ms = rng.gen_range(100..=1500); // ✅ 可调节范围
        // println!("delay: {}ms", delay_ms);
        // sleep(Duration::from_millis(delay_ms)).await;

        let page = storage::fetch_page(&task.url).await;
        println!("page: {:?}, url: {:?}, depth: {}", page.is_some(), task.url, task.depth);
        if let Some((title, body, links)) = page {
            if let Some(keyword) = filter::matches_keywords(&body) {
                println!("title: {:?}", title);
                println!("keyword: {:?}", keyword);
                println!("link: {:?}", links);
                if let Err(e) = storage::save_page(&task.url, &title, &body, &keyword) {
                    eprintln!("保存页面失败: {}", e);
                }
            }

            let _ = storage::save_links(&task.url, &links);
            for link in links {
                graph.add_link(&task.url, &link);

                if !visited.contains(&link) {
                    // let _ = tx.send(CrawlTask {
                    //     url: link,
                    //     depth: task.depth - 1,
                    // }).await;

                    let tx_clone = tx.clone(); // ✅ 克隆 sender
                    let new_task = CrawlTask {
                        url: link,
                        depth: task.depth - 1,
                    };

                    // ✅ 并发发送下一个爬取任务
                    tokio::spawn(async move {
                        let _ = tx_clone.send(new_task).await;
                    });
                }
            }
        }
        drop(permit);
    }
}