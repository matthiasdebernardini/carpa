use serde::{Deserialize, Serialize};

/// OpenRouter's `openrouter:web_fetch` server tool, for requests sent through OpenRouter.
///
/// OpenRouter fetches the page, so Claude's reply carries `server_tool_use` and
/// `web_fetch_tool_result` blocks and no tool call for the app to answer. Every parameter is
/// optional; OpenRouter's defaults apply to the ones left unset. OpenRouter marks server tools
/// as beta.
///
/// ```
/// use carpa::{OpenRouterFetchEngine, OpenRouterWebFetchTool, ToolUnionParam};
///
/// let tool = ToolUnionParam::OpenRouterWebFetch(
///     OpenRouterWebFetchTool::new()
///         .with_engine(OpenRouterFetchEngine::Firecrawl)
///         .with_max_uses(5),
/// );
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct OpenRouterWebFetchTool {
    /// The fetch settings, omitted when none is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<OpenRouterWebFetchParameters>,
}

/// The `parameters` of an [`OpenRouterWebFetchTool`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct OpenRouterWebFetchParameters {
    /// The fetch engine; OpenRouter's default is `auto`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<OpenRouterFetchEngine>,

    /// Fetches the model may run in one request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u32>,

    /// Approximate tokens of content kept per page; longer content is truncated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_content_tokens: Option<u64>,

    /// Only fetch from these domains.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    /// Never fetch from these domains.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_domains: Option<Vec<String>>,
}

/// The engine behind [`OpenRouterWebFetchTool`].
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OpenRouterFetchEngine {
    /// The provider's own fetch when it has one, otherwise Exa.
    Auto,
    /// The provider's own fetch, falling back to the `openrouter` engine.
    Native,
    /// Exa's Contents API.
    Exa,
    /// OpenRouter's own HTTP fetch with content extraction.
    #[serde(rename = "openrouter")]
    OpenRouter,
    /// Firecrawl's scrape, with OpenRouter's key, billed to OpenRouter credits.
    Firecrawl,
    /// Parallel's extract API.
    Parallel,
}

impl OpenRouterWebFetchTool {
    /// A web fetch tool with OpenRouter's defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn parameters_mut(&mut self) -> &mut OpenRouterWebFetchParameters {
        self.parameters.get_or_insert_with(Default::default)
    }

    /// Sets the fetch engine.
    #[must_use]
    pub fn with_engine(mut self, engine: OpenRouterFetchEngine) -> Self {
        self.parameters_mut().engine = Some(engine);
        self
    }

    /// Sets the fetches allowed in one request.
    #[must_use]
    pub fn with_max_uses(mut self, max_uses: u32) -> Self {
        self.parameters_mut().max_uses = Some(max_uses);
        self
    }

    /// Sets the approximate tokens of content kept per page.
    #[must_use]
    pub fn with_max_content_tokens(mut self, max_content_tokens: u64) -> Self {
        self.parameters_mut().max_content_tokens = Some(max_content_tokens);
        self
    }

    /// Only fetch from these domains.
    #[must_use]
    pub fn with_allowed_domains(mut self, domains: Vec<String>) -> Self {
        self.parameters_mut().allowed_domains = Some(domains);
        self
    }

    /// Never fetch from these domains.
    #[must_use]
    pub fn with_blocked_domains(mut self, domains: Vec<String>) -> Self {
        self.parameters_mut().blocked_domains = Some(domains);
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::ToolUnionParam;

    #[test]
    fn bare_tool_has_no_parameters() {
        let tool = ToolUnionParam::OpenRouterWebFetch(OpenRouterWebFetchTool::new());
        let expected = json!({"type": "openrouter:web_fetch"});
        assert_eq!(to_value(&tool).unwrap(), expected);
        assert_eq!(
            serde_json::from_value::<ToolUnionParam>(expected).unwrap(),
            tool
        );
    }

    #[test]
    fn every_parameter_serializes_and_round_trips() {
        let tool = ToolUnionParam::OpenRouterWebFetch(
            OpenRouterWebFetchTool::new()
                .with_engine(OpenRouterFetchEngine::Firecrawl)
                .with_max_uses(10)
                .with_max_content_tokens(100_000)
                .with_allowed_domains(vec!["docs.example.com".to_string()])
                .with_blocked_domains(vec!["private.example.com".to_string()]),
        );
        let expected = json!({
            "type": "openrouter:web_fetch",
            "parameters": {
                "engine": "firecrawl",
                "max_uses": 10,
                "max_content_tokens": 100_000,
                "allowed_domains": ["docs.example.com"],
                "blocked_domains": ["private.example.com"]
            }
        });
        assert_eq!(to_value(&tool).unwrap(), expected);
        assert_eq!(
            serde_json::from_value::<ToolUnionParam>(expected).unwrap(),
            tool
        );
        // Both domain lists may be sent together, unlike Anthropic's own web fetch tool.
        assert!(tool.validate().is_ok());
    }

    #[test]
    fn engines_use_the_documented_names() {
        let engines = [
            (OpenRouterFetchEngine::Auto, "auto"),
            (OpenRouterFetchEngine::Native, "native"),
            (OpenRouterFetchEngine::Exa, "exa"),
            (OpenRouterFetchEngine::OpenRouter, "openrouter"),
            (OpenRouterFetchEngine::Firecrawl, "firecrawl"),
            (OpenRouterFetchEngine::Parallel, "parallel"),
        ];
        for (engine, name) in engines {
            assert_eq!(to_value(engine).unwrap(), json!(name));
        }
    }
}
