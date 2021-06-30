use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Answer {
	pub tags: Vec<String>,
	pub question_id: usize,
	pub comment_count: usize,
	pub up_vote_count: usize,
	pub down_vote_count: usize,
	pub is_accepted: bool,
	pub score: usize,
	// pub last_activity_date: usize,
	// pub creation_date: usize,
	pub answer_id: usize,
	// pub content_license: String,
	pub title: String,
	pub body_markdown: String,
}

#[derive(Deserialize, Debug)]
pub struct Question {
	pub tags: Vec<String>,
	pub is_answered: bool,
	pub view_count: usize,
	pub answer_count: usize,
	pub accepted_answer_id: Option<usize>,
	pub score: usize,
	// pub last_activity_date: usize,
	// pub creation_date: usize,
	pub question_id: usize,
	pub body: String,
	pub body_markdown: String,
	pub title: String,
	// pub link_url: String,
	// pub last_edit_date: usize,
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
	pub has_more: bool,
	pub items: Vec<T>
}
