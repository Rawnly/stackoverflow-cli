use url::Url;
use std::{result::Result, collections::HashMap};
use isahc::{prelude::*};
use serde_json::{from_str};
use crate::models::{ Answer, Response, Question };

type IsahcResult<T> = Result<T, isahc::Error>;

pub struct StackExchangeAPI {
	base_url: Url,
	site: String
}

pub struct AnswerParams {
	page: usize,
	pagesize: usize,
	order: String,
	sort: String,
	site: String,
	filter: String
}

pub enum Order {
	ASC,
	DESC,
}

impl AnswerParams {
	pub fn new(site: &str) -> AnswerParams {
		AnswerParams {
			page: 1,
			pagesize: 10,
			site: site.into(),
			order: "desc".into(),
			sort: "votes".into(),
			filter: "!T1gn2_ezIc6))k8COA".into()
		}
	}

	pub fn site<'a>(&'a mut self, site: &str) {
		self.site = site.into();
	}

	pub fn order<'a>(&'a mut self, order: Order) {
		self.order = match order {
			Order::ASC => "asc".into(),
			Order::DESC => "desc".into()
		};
	}

	pub fn page<'a>(&'a mut self, page: usize) {
		self.page = page;
	}

	pub fn pagesize<'a>(&'a mut self, pagesize: usize) {
		self.pagesize = pagesize;
	}
}

impl StackExchangeAPI {
	pub fn new(base_url: &str, site: &str) -> StackExchangeAPI {
		StackExchangeAPI {
			site: String::from(site),
			base_url: Url::parse(base_url)
				.unwrap()
		}
	}

	pub fn get_answers(&self, question_id: usize) -> IsahcResult<Vec<Answer>> {
		let mut options: HashMap<&str, &str> = HashMap::new();

		options.insert("page", "1");
		options.insert("order", "desc");
		options.insert("sort", "votes");
		options.insert("pagesize", "10");
		options.insert("site", &self.site);
		options.insert("filter", "!T1gn2_ezIc6))k8COA");

		let path = format!("/questions/{}/answer_id", question_id);
		let response = self.get(path, options)?;
		let answers: Response<Answer> = from_str(&response)
			.expect("An error has occurred while decoding JSON");

		return Ok(answers.items);
	}

	pub fn search(&self, query: &str, tagged: &str) -> IsahcResult<Vec<Question>> {
		let mut options: HashMap<&str, &str> = HashMap::new();

		options.insert("page", "1");
		options.insert("order", "desc");
		options.insert("sort", "votes");
		options.insert("pagesize", "15");
		options.insert("answers", "1");
		options.insert("q", query);
		options.insert("tagged", tagged);
		options.insert("site", &self.site);
		options.insert("filter", "7W_5I-TD_");

		let response = self.get("/search/advanced".into(), options)?;

		let answers: Response<Question> = from_str(&response)
			.expect("An error has occurred while decoding JSON");

		return Ok(answers.items);
	}


	//=> GET Request
	fn get(&self, path: String, params: HashMap<&str, &str>) -> IsahcResult<String> {
		let mut url = self.base_url.clone();

		url.set_path(&format!("/2.3{}", path));

		// Setup query parameters
		for key in params.keys() {
			url
				.query_pairs_mut()
				.append_pair(key, params.get(key).unwrap());
		}

		// Prepare
		let request = isahc::Request::builder()
			.method("GET")
			// Build the url
			.uri(url.as_str())
			.body(())?;

		// Send the request
		let mut response = isahc::send(request)?;

		return Ok(response.text()?.into());
	}
}
