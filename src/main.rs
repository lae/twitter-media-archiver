#[macro_use] extern crate clap;
#[macro_use] extern crate failure;
#[macro_use] extern crate failure_derive;
extern crate glob;
extern crate serde_json;

use failure::Error;
use glob::glob;
use serde_json::Value;

use std::path;
use std::fs;


#[derive(Debug, Fail)]
enum ArchiverError {
    #[fail(display = "Tweet data was not found in {}, is it an actual archive?", _0)]
    MissingTweetData(String),
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", pretty_error(&err));

        let backtrace = err.backtrace().to_string();
        if !backtrace.trim().is_empty() {
            eprintln!("{}", backtrace);
        }
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    let cli = clap_app!(app =>
        (version: "0.1")
        (about: "Downloads media locally from a Twitter archive.")
        (author: "lae")
        (@arg ARCHIVE_PATH: +required "Path to the extracted Twitter archive folder.")
        (@arg videos: --videos "Prints out a list of tweet URLs with a video or GIF.")
    ).get_matches();

    let path = cli.value_of("ARCHIVE_PATH").unwrap();
    let print_videos = cli.is_present("videos");

    // Collect a list of files that should contain tweets and fail if none are found
    let files = glob(&format!("{}/data/js/tweets/*.js", path))?
                .filter_map(|p| p.ok())
                .collect::<Vec<path::PathBuf>>();
    if files.is_empty() {
        bail!(ArchiverError::MissingTweetData(path.to_string()));
    }

    for file in files {
        // Read the json contents into a string, skipping the "Grailbird" line
        let content = fs::read_to_string(&file)?.lines().skip(1).collect::<String>();
        // Parse the tweets in the current file into a vector of json objects
        let mut tweets = serde_json::from_str::<Value>(&content)?.as_array().unwrap().to_vec();
        tweets.retain(|tweet| {
            // skip retweets
            if !tweet["retweeted_status"].is_null() { return false; }
            let media = tweet["entities"]["media"].as_array().unwrap();
            if !media.is_empty() {
                // if print_videos is set, only keep video/gif tweets, otherwise only keep images
                let is_video = media[0]["media_url_https"].as_str().unwrap().to_string().contains("ext_tw_video_thumb");
                print_videos && is_video || !print_videos && !is_video
            } else {
                // skip tweets that don't have media metadata (text-only tweets)
                false
            }
        });
        if print_videos {
            for tweet in tweets {
                println!("https://twitter.com/{}/status/{}", tweet["user"]["screen_name"].as_str().unwrap(), tweet["id"]);
            }
        } else {
            for tweet in tweets {
                let media = tweet["entities"]["media"].as_array().unwrap().iter().map(|m| m["media_url_https"].as_str().unwrap().to_string()).collect::<Vec<String>>();
                //println!("https://twitter.com/{}/status/{}", tweet["user"]["screen_name"].as_str().unwrap(), tweet["id"]);
                println!("{:?}", media);
            }
        }
    }
    Ok(())
}

fn pretty_error(err: &failure::Error) -> String {
    let mut pretty = err.to_string();
    let mut prev = err.as_fail();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        prev = next;
    }
    pretty
}
