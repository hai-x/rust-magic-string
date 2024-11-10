use fast_magic_string::{
  fast_magic_string_sourcemap::SourceMap, GenerateMapOptions, IndentOptions, MagicStringOptions,
  OverwriteOptions,
};

#[napi(object)]
pub struct JsIndentOptions {
  pub exclude: Option<Vec<Vec<u32>>>,
  pub indent_start: Option<bool>,
}

impl From<JsIndentOptions> for IndentOptions {
  fn from(js_indent_options: JsIndentOptions) -> Self {
    IndentOptions {
      exclude: js_indent_options.exclude,
      indent_start: js_indent_options.indent_start,
    }
  }
}

#[napi(object)]
pub struct JsMagicStringOptions {
  pub filename: Option<String>,
  pub indent_exclusion_ranges: Option<Vec<u32>>,
  pub ignore_list: Option<bool>,
}

impl From<JsMagicStringOptions> for MagicStringOptions {
  fn from(js_magic_string_options: JsMagicStringOptions) -> Self {
    MagicStringOptions {
      filename: js_magic_string_options.filename,
      indent_exclusion_ranges: js_magic_string_options.indent_exclusion_ranges,
      ignore_list: js_magic_string_options.ignore_list,
    }
  }
}

#[napi(object)]
pub struct JsGenerateMapOptions {
  pub file: Option<String>,
  pub source: Option<String>,
  pub source_root: Option<String>,
  pub include_content: Option<bool>,
  pub hires: Option<bool>,
}

impl From<JsGenerateMapOptions> for GenerateMapOptions {
  fn from(js_generate_map_options: JsGenerateMapOptions) -> Self {
    GenerateMapOptions {
      file: js_generate_map_options.file,
      source: js_generate_map_options.source,
      source_root: js_generate_map_options.source_root,
      include_content: js_generate_map_options.include_content,
      hires: js_generate_map_options.hires,
    }
  }
}

#[derive(Clone)]
#[napi(object)]
pub struct JsOverwriteOptions {
  pub content_only: Option<bool>,
  pub store_name: Option<bool>,
  pub overwrite: Option<bool>,
}

impl From<JsOverwriteOptions> for OverwriteOptions {
  fn from(js_overwrite_options: JsOverwriteOptions) -> Self {
    OverwriteOptions {
      content_only: js_overwrite_options.content_only,
      store_name: js_overwrite_options.store_name,
      overwrite: js_overwrite_options.overwrite,
    }
  }
}

#[napi(object)]
pub struct JsRegExp {
  pub global: Option<bool>,
  pub rule: String,
}
