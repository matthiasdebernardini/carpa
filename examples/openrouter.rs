//! Asks Claude a question with OpenRouter's web search server tool, then asks Jev one yes/no
//! question, both through OpenRouter with one key.
//!
//! Needs OPENROUTER_API_KEY. Pass `firecrawl` to search with Firecrawl instead of OpenRouter's
//! default engine:
//!
//! ```bash
//! cargo run --example openrouter -- firecrawl
//! ```

use std::env;

use carpa::jev::{JevClient, Outcome, Questions};
use carpa::{
    Anthropic, ContentBlock, KnownModel, MessageCreateParams, OpenRouterSearchEngine,
    OpenRouterWebSearchTool, ToolUnionParam,
};

const CLAUDE: KnownModel = KnownModel::ClaudeHaiku55;
const JEV: &str = "typesafe/jev-1.14";
// `pause_turn` replies resend the turn; stop a search that keeps going.
const MAX_TURNS: u32 = 3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(key) = env::var("OPENROUTER_API_KEY")
        .ok()
        .filter(|key| !key.trim().is_empty())
    else {
        return Err(
            "OPENROUTER_API_KEY is not set; create a key at https://openrouter.ai/settings/keys"
                .into(),
        );
    };

    let mut search = OpenRouterWebSearchTool::new().with_max_results(3);
    match env::args().nth(1).as_deref() {
        None => {}
        Some("firecrawl") => search = search.with_engine(OpenRouterSearchEngine::Firecrawl),
        Some(other) => return Err(format!("unknown argument {other:?}; expected firecrawl").into()),
    }

    let claude = Anthropic::openrouter(&key)?;
    let params = MessageCreateParams::simple(
        "What is the latest stable Rust release? Answer in two sentences and cite a source.",
        CLAUDE,
    )
    .with_tools(vec![ToolUnionParam::OpenRouterWebSearch(search)]);
    // No client tools: OpenRouter runs every search itself.
    let run = claude.run_tools(params, &[], MAX_TURNS).await?;
    for response in &run.responses {
        for block in &response.content {
            match block {
                ContentBlock::Text(text) => println!("{}", text.text),
                ContentBlock::ServerToolUse(call) => println!("[{}] {}", call.name, call.input),
                other => println!("{}", other.label()),
            }
        }
        println!(
            "tokens: {} in, {} out",
            response.usage.input_tokens, response.usage.output_tokens
        );
    }

    let jev = JevClient::openrouter(&key, JEV)?;
    let state = "I was charged twice for my subscription. Please refund one charge.";
    let questions = Questions::new().noul("refund", "Does the customer ask for money back?");
    let response = jev.ask(state, &questions).await?;
    match response.yes_no("refund", 0.8)? {
        Outcome::Answered(refund) => println!("jev ({}): refund = {refund}", response.model),
        unsure => println!("jev ({}): unsure, {unsure:?}", response.model),
    }
    match response.cost {
        Some(cost) => println!("jev cost: ${cost:.6}"),
        None => println!("jev cost: not reported"),
    }
    Ok(())
}
