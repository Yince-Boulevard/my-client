use anyhow::Result;
use futures::future::join_all;

use crate::{Comment, StoryData, StoryItem};

#[allow(unused)]
const MAX_STORIES: usize = 70;

pub async fn get_top_stories(limit: usize) -> Result<Vec<StoryItem>> {
    let n = limit.min(MAX_STORIES);
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let ids: Vec<i64> = reqwest::get(url).await?.json().await?;

    let story_futures = ids.into_iter().take(n).map(get_story);

    let stories = join_all(story_futures).await;
    let items = stories.into_iter().collect::<Result<Vec<_>>>()?;

    Ok(items)
}

#[allow(unused)]
pub async fn get_story(id: i64) -> Result<StoryItem> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let item: StoryItem = reqwest::get(url).await?.json().await?;

    Ok(item)
}

#[allow(unused)]
pub async fn get_story_comments(item: StoryItem) -> Result<StoryData> {
    let comment_futures = item.kids.iter().map(|id| get_comment_by_id(*id));
    let comments = join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|res| res.ok())
        .collect::<Vec<Comment>>();

    Ok(StoryData {
        items: vec![item],
        comments,
    })
}

pub async fn get_comment_by_id(id: i64) -> Result<Comment> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let item: Comment = reqwest::get(url).await?.json().await?;

    Ok(item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_top_stories() -> Result<()> {
        let stories = get_top_stories(3).await?;
        assert_eq!(stories.len(), 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_story() -> Result<()> {
        let item = get_story(1).await?;
        assert_eq!(item.id, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_comment_by_id() -> Result<()> {
        let story = get_top_stories(1).await.unwrap().pop().unwrap();
        let id = story.kids[0];
        let comment = get_comment_by_id(id).await?;
        assert_eq!(comment.id, id);
        Ok(())
    }
}
