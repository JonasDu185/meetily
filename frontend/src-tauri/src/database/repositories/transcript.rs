use crate::api::{TranscriptSearchResult, TranscriptSegment};
use chrono::Utc;
use sqlx::{Connection, Error as SqlxError, SqlitePool};
use tracing::{error, info};
use uuid::Uuid;

pub struct TranscriptsRepository;

impl TranscriptsRepository {
    /// Saves a new meeting and its associated transcript segments.
    /// This function uses a transaction to ensure that either both the meeting
    /// and all its transcripts are saved, or none of them are.
    pub async fn save_transcript(
        pool: &SqlitePool,
        meeting_title: &str,
        transcripts: &[TranscriptSegment],
        folder_path: Option<String>,
    ) -> Result<String, SqlxError> {
        let meeting_id = format!("meeting-{}", Uuid::new_v4());

        let mut conn = pool.acquire().await?;
        let mut transaction = conn.begin().await?;

        let now = Utc::now();

        // 1. Create the new meeting
        let result = sqlx::query(
            "INSERT INTO meetings (id, title, created_at, updated_at, folder_path) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&meeting_id)
        .bind(meeting_title)
        .bind(now)
        .bind(now)
        .bind(&folder_path)
        .execute(&mut *transaction)
        .await;

        if let Err(e) = result {
            error!("Failed to create meeting '{}': {}", meeting_title, e);
            transaction.rollback().await?;
            return Err(e);
        }

        info!("Successfully created meeting with id: {}", meeting_id);

        // 2. Save each transcript segment with audio timing fields
        for segment in transcripts {
            let transcript_id = format!("transcript-{}", Uuid::new_v4());
            let result = sqlx::query(
                "INSERT INTO transcripts (id, meeting_id, transcript, timestamp, audio_start_time, audio_end_time, duration)
                 VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&transcript_id)
            .bind(&meeting_id)
            .bind(&segment.text)
            .bind(&segment.timestamp)
            .bind(segment.audio_start_time)
            .bind(segment.audio_end_time)
            .bind(segment.duration)
            .execute(&mut *transaction)
            .await;

            if let Err(e) = result {
                error!(
                    "Failed to save transcript segment for meeting {}: {}",
                    meeting_id, e
                );
                transaction.rollback().await?;
                return Err(e);
            }
        }

        info!(
            "Successfully saved {} transcript segments for meeting {}",
            transcripts.len(),
            meeting_id
        );

        // Commit the transaction
        transaction.commit().await?;

        Ok(meeting_id)
    }

    /// Searches for a query string within the transcripts.
    /// It returns a list of matching transcripts with context.
    pub async fn search_transcripts(
        pool: &SqlitePool,
        query: &str,
    ) -> Result<Vec<TranscriptSearchResult>, SqlxError> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let search_query = format!("%{}%", query.to_lowercase());

        let rows = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT m.id, m.title, t.transcript, t.timestamp
             FROM meetings m
             JOIN transcripts t ON m.id = t.meeting_id
             WHERE LOWER(t.transcript) LIKE ?",
        )
        .bind(&search_query)
        .fetch_all(pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|(id, title, transcript, timestamp)| {
                let match_context = Self::get_match_context(&transcript, query);
                TranscriptSearchResult {
                    id,
                    title,
                    match_context,
                    timestamp,
                }
            })
            .collect();

        Ok(results)
    }

    /// Helper function to extract a snippet of text around the first match of a query.
    fn get_match_context(transcript: &str, query: &str) -> String {
        if query.is_empty() {
            return transcript.chars().take(200).collect();
        }

        // SQLite 的 LOWER 默认只处理 ASCII。这里保持相同语义，同时确保索引
        // 与原字符串的 UTF-8 字节位置一致，避免中文或表情符号被从中间截断。
        let searchable_transcript = transcript.to_ascii_lowercase();
        let searchable_query = query.to_ascii_lowercase();

        match searchable_transcript.find(&searchable_query) {
            Some(match_index) => {
                let match_end = match_index + query.len();
                let start_index = transcript[..match_index]
                    .char_indices()
                    .rev()
                    .nth(99)
                    .map(|(index, _)| index)
                    .unwrap_or(0);
                let end_index = transcript[match_end..]
                    .char_indices()
                    .nth(100)
                    .map(|(index, _)| match_end + index)
                    .unwrap_or(transcript.len());

                let mut context = String::new();
                if start_index > 0 {
                    context.push_str("...");
                }
                context.push_str(&transcript[start_index..end_index]);
                if end_index < transcript.len() {
                    context.push_str("...");
                }
                context
            }
            None => transcript.chars().take(200).collect(), // Fallback to the start of the transcript
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TranscriptsRepository;

    #[test]
    fn 中文搜索不会截断_utf8() {
        let transcript = format!("{}重点结论{}", "前".repeat(150), "后".repeat(150));
        let context = TranscriptsRepository::get_match_context(&transcript, "重点结论");

        assert!(context.contains("重点结论"));
        assert!(context.starts_with("..."));
        assert!(context.ends_with("..."));
    }

    #[test]
    fn 中英混合搜索忽略英文大小写() {
        let context =
            TranscriptsRepository::get_match_context("今天讨论 Project Alpha 的下一步", "project alpha");

        assert!(context.contains("Project Alpha"));
    }

    #[test]
    fn 表情符号附近搜索不会崩溃() {
        let transcript = format!("{}😀行动项{}", "会".repeat(120), "议".repeat(120));
        let context = TranscriptsRepository::get_match_context(&transcript, "行动项");

        assert!(context.contains("😀行动项"));
    }

    #[test]
    fn 未匹配时最多返回二百个字符() {
        let transcript = "长".repeat(250);
        let context = TranscriptsRepository::get_match_context(&transcript, "不存在");

        assert_eq!(context.chars().count(), 200);
    }
}
