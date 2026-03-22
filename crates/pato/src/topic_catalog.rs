pub struct TopicEntry {
    pub name: &'static str,
    pub content: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/topic_catalog.rs"));

pub fn find_topic(name: &str) -> Option<&'static TopicEntry> {
    TOPICS.iter().find(|topic| topic.name == name)
}

pub fn topic_names() -> impl Iterator<Item = &'static str> {
    TOPICS.iter().map(|topic| topic.name)
}
