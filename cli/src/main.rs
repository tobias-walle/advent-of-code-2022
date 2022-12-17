use std::{
    collections::HashMap,
    env,
    io::{self, Write},
};

use clap::{command, Parser, Subcommand};
use colored::Colorize;
use eyre::{Context, ContextCompat, Result};
use reqwest::header::HeaderMap;
use tokio::{fs, try_join};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let session = env::var("AOC_SESSION").context("AOC_SESSION not defined in environment")?;

    let mut headers = HeaderMap::new();
    headers.insert("cookie", format!("session={session}").parse()?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    match args.command {
        Command::Download { year, day } => {
            let (problem, examples, _) = try_join!(
                download_problem(&client, year, day, "./problem.txt"),
                download_potential_examples(&client, year, day),
                download_input(&client, year, day, "./input.txt"),
            )?;
            println!("\n{}", "Problem:".cyan());
            println!("{problem}\n\n");
            choose_and_save_correct_example(examples, "./example.txt").await?;
        }
        Command::Submit {
            result,
            year,
            day,
            level,
        } => {
            let response = submit_input(&client, year, day, level, &result).await?;
            println!("\nResponse:\n{response}");
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    Download {
        #[arg(short, long)]
        year: u32,
        #[arg(short, long)]
        day: u32,
    },
    Submit {
        result: String,
        #[arg(short, long)]
        year: u32,
        #[arg(short, long)]
        day: u32,
        #[arg(short, long)]
        level: u32,
    },
}

async fn download_problem(
    client: &reqwest::Client,
    year: u32,
    day: u32,
    output_file: &str,
) -> Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    let html = client.get(url).send().await?.text().await?;
    let article = format_html_output(&html)?;
    save(output_file, article.clone()).await?;
    Ok(article)
}

async fn download_input(
    client: &reqwest::Client,
    year: u32,
    day: u32,
    output_file: &str,
) -> Result<()> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let input_text = client.get(url).send().await?.text().await?;
    save(output_file, input_text).await?;
    Ok(())
}

async fn submit_input(
    client: &reqwest::Client,
    year: u32,
    day: u32,
    level: u32,
    result: &str,
) -> Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let mut form = HashMap::new();
    form.insert("level", format!("{level}"));
    form.insert("answer", result.into());
    let response = client.post(url).form(&form).send().await?.text().await?;
    let response = format_html_output(&response)?;
    Ok(response)
}

async fn download_potential_examples(
    client: &reqwest::Client,
    year: u32,
    day: u32,
) -> Result<Vec<String>> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    let html = client.get(url).send().await?.text().await?;
    let dom = tl::parse(&html, Default::default())?;
    let parser = dom.parser();
    let examples: Vec<String> = dom
        .query_selector("pre")
        .unwrap()
        .map(|element| element.get(parser).unwrap())
        .map(|node| {
            node.inner_text(parser)
                .to_string()
                .replace("&lt;", "<")
                .replace("&gt;", ">")
        })
        .collect();
    Ok(examples)
}

async fn choose_and_save_correct_example(examples: Vec<String>, output_file: &str) -> Result<()> {
    if examples.is_empty() {
        println!("{}", "No examples found.".cyan());
        return Ok(());
    }

    println!(
        "{}",
        format!(
            "Downloaded {} potential examples, please choose one:\n",
            examples.len()
        )
        .cyan()
    );
    for (i, example) in examples.into_iter().enumerate() {
        println!("\n{}", format!("Example {}:", i + 1).cyan());
        println!("{}", &example);
        print!("\n{}", "> Save? (Y/n): ".cyan());
        io::stdout().flush()?;
        if let YesNoChoice::Yes = prompt_user()? {
            save(output_file, example).await?;
            return Ok(());
        };
    }
    println!("\n{}", "Nothing selected".cyan());
    Ok(())
}

fn format_html_output(html: &str) -> Result<String> {
    let dom = tl::parse(html, Default::default())?;
    let parser = dom.parser();
    let article = dom
        .query_selector("article")
        .unwrap()
        .next()
        .context("Cannot find article")?
        .get(parser)
        .unwrap()
        .inner_html(parser);
    let article = html_to_text(&article);
    Ok(article)
}

fn html_to_text(html: &str) -> String {
    html2text::from_read(html.as_bytes(), 80)
}

enum YesNoChoice {
    Yes,
    No,
}

fn prompt_user() -> io::Result<YesNoChoice> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let result = match input.to_lowercase().trim() {
        "y" => YesNoChoice::Yes,
        "n" => YesNoChoice::No,
        _ => YesNoChoice::Yes,
    };
    Ok(result)
}

async fn save(file: &str, content: String) -> Result<()> {
    println!("{}", format!("Saving {file}").cyan());
    fs::write(file, content.clone())
        .await
        .with_context(|| format!("Couldn't write to {file}"))?;
    Ok(())
}
