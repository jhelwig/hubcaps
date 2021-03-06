//! Labels interface

extern crate serde_json;

use futures::future;
use hyper::client::Connect;

use {Github, Future};

pub struct Labels<C>
where
    C: Connect + Clone,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Connect + Clone> Labels<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Labels {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/labels{}", self.owner, self.repo, more)
    }

    pub fn create(&self, lab: &LabelOptions) -> Future<Label> {
        self.github.post(&self.path(""), json!(lab))
    }

    pub fn update(&self, prevname: &str, lab: &LabelOptions) -> Future<Label> {
        self.github.patch(
            &self.path(&format!("/{}", prevname)),
            json!(lab),
        )
    }

    pub fn delete(&self, name: &str) -> Future<()> {
        self.github.delete(&self.path(&format!("/{}", name)))
    }

    pub fn list(&self) -> Future<Vec<Label>> {
        self.github.get(&self.path(""))
    }
}

// representations

#[derive(Debug, Serialize)]
pub struct LabelOptions {
    pub name: String,
    pub color: String,
}

impl LabelOptions {
    pub fn new<N, C>(name: N, color: C) -> LabelOptions
    where
        N: Into<String>,
        C: Into<String>,
    {
        LabelOptions {
            name: name.into(),
            color: color.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub url: String,
    pub name: String,
    pub color: String,
}