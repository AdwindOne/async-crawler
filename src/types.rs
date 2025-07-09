#[derive(Debug, Clone)]
pub struct CrawlTask {
    pub url: String,
    pub depth: usize,
}