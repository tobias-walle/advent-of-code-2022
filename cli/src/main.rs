use std::{
    collections::HashMap,
    env,
    io::{self, Write},
};

use clap::{command, Parser, Subcommand};
use colored::Colorize;
use eyre::{Context, Result};
use regex::{Captures, Regex};
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
        Command::Download { example, year, day } => {
            if example {
                let examples = download_potential_examples(&client, year, day).await?;
                choose_and_save_correct_example(examples, "./example.txt").await?;
            } else {
                let (problem, _) = try_join!(
                    download_problem(&client, year, day, "./problem.md"),
                    download_input(&client, year, day, "./input.txt"),
                )?;
                println!("\n{}", "Problem:".cyan());
                println!("{problem}\n\n");
            }
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
        example: bool,
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
    save(output_file, &article).await?;
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
    save(output_file, &input_text).await?;
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
            "Downloaded {} potential examples, please choose one:",
            examples.len()
        )
        .cyan()
    );
    for (i, example) in examples.iter().enumerate().rev() {
        println!();
        println!("{}", format!("Example {}:", i).cyan());
        let lines: Vec<_> = example.lines().collect();
        let short_example = limit_size(&lines, 10).join("\n");
        println!("{}", short_example);
    }
    println!();
    print!("{}", "> Choose example: ".cyan());
    io::stdout().flush()?;
    match read_user_input()?.parse::<usize>() {
        Ok(n) if n < examples.len() => {
            save(output_file, &examples[n]).await?;
            Ok(())
        }
        _ => {
            println!();
            println!("{}", "Nothing selected".cyan());
            Ok(())
        }
    }
}

fn limit_size<T>(list: &[T], limit: usize) -> &[T] {
    list.get(..limit).unwrap_or(list)
}

fn format_html_output(html: &str) -> Result<String> {
    let dom = tl::parse(html, Default::default())?;
    let parser = dom.parser();
    let articles: Vec<_> = dom
        .query_selector("article")
        .unwrap()
        .map(|node| node.get(parser).unwrap().inner_html(parser))
        .collect();
    let articles = articles.join("\n");
    let html = format!("<div>{articles}</div>");
    let article = html_to_text(&html);
    Ok(article)
}

fn html_to_text(html: &str) -> String {
    let text = html2text::from_read(html.as_bytes(), 80);

    // Wrap multiline code examples with triple ```
    let code_regex = Regex::new(r"`([^`]+)`").unwrap();
    let text = code_regex.replace_all(&text, |caps: &Captures| {
        let content = &caps[1];
        if content.contains('\n') {
            format!("```\n{content}\n```")
        } else {
            format!("`{content}`")
        }
    });

    text.to_string()
}

fn read_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

async fn save(file: &str, content: &str) -> Result<()> {
    println!("{}", format!("Saving {file}").cyan());
    fs::write(file, content)
        .await
        .with_context(|| format!("Couldn't write to {file}"))?;
    Ok(())
}
