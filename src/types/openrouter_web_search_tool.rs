use serde::{Deserialize, Serialize};

/// OpenRouter's `openrouter:web_search` server tool, for requests sent through OpenRouter.
///
/// OpenRouter runs the search, so Claude's reply carries `server_tool_use` and
/// `web_search_tool_result` blocks and no tool call for the app to answer. Every parameter is
/// optional; OpenRouter's defaults apply to the ones left unset. OpenRouter marks server tools
/// as beta.
///
/// ```
/// use carpa::{OpenRouterSearchEngine, OpenRouterWebSearchTool, ToolUnionParam};
///
/// let tool = ToolUnionParam::OpenRouterWebSearch(
///     OpenRouterWebSearchTool::new()
///         .with_engine(OpenRouterSearchEngine::Firecrawl)
///         .with_max_results(5),
/// );
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct OpenRouterWebSearchTool {
    /// The search settings, omitted when none is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<OpenRouterWebSearchParameters>,
}

/// The `parameters` of an [`OpenRouterWebSearchTool`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct OpenRouterWebSearchParameters {
    /// The search engine; OpenRouter's default is `auto`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<OpenRouterSearchEngine>,

    /// An engine-specific search mode; engines that lack it ignore it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<OpenRouterSearchMode>,

    /// Results per search, 1 to 25 (1 to 20 for Perplexity); ignored by native search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,

    /// Searches the model may run in one request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u32>,

    /// Results across every search in one request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_total_results: Option<u32>,

    /// How much page content to retrieve; ignored by native search and Firecrawl.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_context_size: Option<OpenRouterSearchContextSize>,

    /// Content characters per result, 1 to 100,000; overrides `search_context_size`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_characters: Option<u32>,

    /// The user's approximate location; only native search uses it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_location: Option<OpenRouterUserLocation>,

    /// Only return results from these domains.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    /// Never return results from these domains.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded_domains: Option<Vec<String>>,
}

/// The engine behind [`OpenRouterWebSearchTool`].
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OpenRouterSearchEngine {
    /// The provider's own search when it has one, otherwise Exa.
    Auto,
    /// The provider's own search, falling back to Exa.
    Native,
    /// Exa.
    Exa,
    /// Firecrawl, with OpenRouter's key, billed to OpenRouter credits.
    Firecrawl,
    /// Parallel.
    Parallel,
    /// Perplexity's Search API.
    Perplexity,
}

/// An engine-specific search mode for [`OpenRouterWebSearchParameters::mode`].
///
/// Exa takes `instant`, `fast`, `auto`, `deep-lite`, `deep` and `deep-reasoning`; Parallel
/// takes `turbo`, `fast`, `basic` and `advanced`.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum OpenRouterSearchMode {
    /// Exa `instant`.
    Instant,
    /// Exa or Parallel `fast`.
    Fast,
    /// Exa `auto`, its default.
    Auto,
    /// Exa `deep-lite`.
    DeepLite,
    /// Exa `deep`.
    Deep,
    /// Exa `deep-reasoning`.
    DeepReasoning,
    /// Parallel `turbo`.
    Turbo,
    /// Parallel `basic`, its default.
    Basic,
    /// Parallel `advanced`.
    Advanced,
}

/// How much content [`OpenRouterWebSearchTool`] retrieves per search.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OpenRouterSearchContextSize {
    /// Least content.
    Low,
    /// Medium content.
    Medium,
    /// Most content.
    High,
}

/// The user's approximate location, sent as `{"type": "approximate", ...}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename = "approximate")]
#[non_exhaustive]
pub struct OpenRouterUserLocation {
    /// City, such as `San Francisco`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Region, such as `California`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Country code, such as `US`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// IANA time zone, such as `America/Los_Angeles`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl OpenRouterUserLocation {
    /// A location with no field set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the city.
    #[must_use]
    pub fn with_city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    /// Sets the region.
    #[must_use]
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Sets the country code.
    #[must_use]
    pub fn with_country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Sets the IANA time zone.
    #[must_use]
    pub fn with_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }
}

impl OpenRouterWebSearchTool {
    /// A web search tool with OpenRouter's defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn parameters_mut(&mut self) -> &mut OpenRouterWebSearchParameters {
        self.parameters.get_or_insert_with(Default::default)
    }

    /// Sets the search engine.
    #[must_use]
    pub fn with_engine(mut self, engine: OpenRouterSearchEngine) -> Self {
        self.parameters_mut().engine = Some(engine);
        self
    }

    /// Sets the engine-specific search mode.
    #[must_use]
    pub fn with_mode(mut self, mode: OpenRouterSearchMode) -> Self {
        self.parameters_mut().mode = Some(mode);
        self
    }

    /// Sets the results per search.
    #[must_use]
    pub fn with_max_results(mut self, max_results: u32) -> Self {
        self.parameters_mut().max_results = Some(max_results);
        self
    }

    /// Sets the searches allowed in one request.
    #[must_use]
    pub fn with_max_uses(mut self, max_uses: u32) -> Self {
        self.parameters_mut().max_uses = Some(max_uses);
        self
    }

    /// Sets the results allowed across every search in one request.
    #[must_use]
    pub fn with_max_total_results(mut self, max_total_results: u32) -> Self {
        self.parameters_mut().max_total_results = Some(max_total_results);
        self
    }

    /// Sets how much content to retrieve.
    #[must_use]
    pub fn with_search_context_size(mut self, size: OpenRouterSearchContextSize) -> Self {
        self.parameters_mut().search_context_size = Some(size);
        self
    }

    /// Sets the content characters per result.
    #[must_use]
    pub fn with_max_characters(mut self, max_characters: u32) -> Self {
        self.parameters_mut().max_characters = Some(max_characters);
        self
    }

    /// Sets the user's approximate location.
    #[must_use]
    pub fn with_user_location(mut self, location: OpenRouterUserLocation) -> Self {
        self.parameters_mut().user_location = Some(location);
        self
    }

    /// Only return results from these domains.
    #[must_use]
    pub fn with_allowed_domains(mut self, domains: Vec<String>) -> Self {
        self.parameters_mut().allowed_domains = Some(domains);
        self
    }

    /// Never return results from these domains.
    #[must_use]
    pub fn with_excluded_domains(mut self, domains: Vec<String>) -> Self {
        self.parameters_mut().excluded_domains = Some(domains);
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
        let tool = ToolUnionParam::OpenRouterWebSearch(OpenRouterWebSearchTool::new());
        let expected = json!({"type": "openrouter:web_search"});
        assert_eq!(to_value(&tool).unwrap(), expected);
        assert_eq!(
            serde_json::from_value::<ToolUnionParam>(expected).unwrap(),
            tool
        );
    }

    #[test]
    fn every_parameter_serializes_and_round_trips() {
        let tool = ToolUnionParam::OpenRouterWebSearch(
            OpenRouterWebSearchTool::new()
                .with_engine(OpenRouterSearchEngine::Firecrawl)
                .with_mode(OpenRouterSearchMode::DeepReasoning)
                .with_max_results(25)
                .with_max_uses(3)
                .with_max_total_results(20)
                .with_search_context_size(OpenRouterSearchContextSize::Medium)
                .with_max_characters(2000)
                .with_user_location(
                    OpenRouterUserLocation::new()
                        .with_city("San Francisco")
                        .with_region("California")
                        .with_country("US")
                        .with_timezone("America/Los_Angeles"),
                )
                .with_allowed_domains(vec!["arxiv.org".to_string()])
                .with_excluded_domains(vec!["reddit.com".to_string()]),
        );
        let expected = json!({
            "type": "openrouter:web_search",
            "parameters": {
                "engine": "firecrawl",
                "mode": "deep-reasoning",
                "max_results": 25,
                "max_uses": 3,
                "max_total_results": 20,
                "search_context_size": "medium",
                "max_characters": 2000,
                "user_location": {
                    "type": "approximate",
                    "city": "San Francisco",
                    "region": "California",
                    "country": "US",
                    "timezone": "America/Los_Angeles"
                },
                "allowed_domains": ["arxiv.org"],
                "excluded_domains": ["reddit.com"]
            }
        });
        assert_eq!(to_value(&tool).unwrap(), expected);
        assert_eq!(
            serde_json::from_value::<ToolUnionParam>(expected).unwrap(),
            tool
        );
    }

    #[test]
    fn engines_and_modes_use_the_documented_names() {
        let engines = [
            (OpenRouterSearchEngine::Auto, "auto"),
            (OpenRouterSearchEngine::Native, "native"),
            (OpenRouterSearchEngine::Exa, "exa"),
            (OpenRouterSearchEngine::Firecrawl, "firecrawl"),
            (OpenRouterSearchEngine::Parallel, "parallel"),
            (OpenRouterSearchEngine::Perplexity, "perplexity"),
        ];
        for (engine, name) in engines {
            assert_eq!(to_value(engine).unwrap(), json!(name));
        }
        let modes = [
            (OpenRouterSearchMode::Instant, "instant"),
            (OpenRouterSearchMode::Fast, "fast"),
            (OpenRouterSearchMode::Auto, "auto"),
            (OpenRouterSearchMode::DeepLite, "deep-lite"),
            (OpenRouterSearchMode::Deep, "deep"),
            (OpenRouterSearchMode::DeepReasoning, "deep-reasoning"),
            (OpenRouterSearchMode::Turbo, "turbo"),
            (OpenRouterSearchMode::Basic, "basic"),
            (OpenRouterSearchMode::Advanced, "advanced"),
        ];
        for (mode, name) in modes {
            assert_eq!(to_value(mode).unwrap(), json!(name));
        }
        assert_eq!(
            to_value(OpenRouterSearchContextSize::High).unwrap(),
            json!("high")
        );
    }
}
