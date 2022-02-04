use anyhow::{Error, Result};
use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs::File;

use crate::domain::ports::incoming::TemplateService;

#[derive(Clone)]
pub struct DefaultTemplateService<'a> {
  pub handlebars: Box<Handlebars<'a>>,
}

impl<'a> DefaultTemplateService<'a> {
  pub fn new() -> Result<Self> {
    // let mut handlebars = Box::new(Handlebars::new());
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(false);
    handlebars.register_templates_directory(".yaml", "templates")?;

    Ok(Self {
      handlebars: Box::new(handlebars),
    })
  }
}

impl<'a> TemplateService for DefaultTemplateService<'a> {
  fn render(&self, name: &str, values: &HashMap<&str, &str>) -> Result<String, Error> {
    self.handlebars.render(name, &values).map_err(anyhow::Error::msg)
  }

  fn write_to_file(&self, name: &str, values: &HashMap<&str, &str>, file_path: &str) -> Result<(), Error> {
    let output_file = File::create(file_path)?;
    self
      .handlebars
      .render_to_write(name, &values, output_file)
      .map_err(anyhow::Error::msg)
  }
}
