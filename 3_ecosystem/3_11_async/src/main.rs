use futures::{future, StreamExt};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Response;
use structopt::StructOpt;
use tokio::{fs, io::AsyncWriteExt, runtime};

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(long, takes_value = true)]
    max_threads: Option<usize>,
    #[structopt(takes_value = true, value_name = "file")]
    file: String,
}

impl Options {
    fn init() -> Self {
        let mut options: Options = Options::from_args();

        options.file = format!("3_ecosystem/3_11_async/{}", options.file);

        if options.max_threads.is_none() {
            options.max_threads = Some(num_cpus::get());
        }

        options
    }
}

fn main() {
    let options = Options::init();

    let mut rt = runtime::Builder::new()
        .core_threads(options.max_threads.unwrap())
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let urls = fs::read_to_string(options.file.as_str())
            .await
            .expect("No urls file found");
        let urls: Vec<&str> = urls.split('\n').collect();

        let requests = urls.into_iter().map(reqwest::get).collect::<Vec<_>>();

        let responses = future::join_all(requests)
            .await
            .into_iter()
            .map(std::result::Result::unwrap)
            .collect::<Vec<Response>>();

        let streams = responses
            .into_iter()
            .map(|resp| async move {
                static RE: Lazy<Regex> = Lazy::new(|| {
                    Regex::new(r"((https://)|(http://))?(www\.)?(?P<name>[^.]+)").unwrap()
                });

                let captures = RE.captures(resp.url().as_ref()).unwrap();
                let filename = captures.name("name").unwrap().as_str();

                let mut file =
                    fs::File::create(format!("3_ecosystem/3_11_async/{}.html", filename))
                        .await
                        .unwrap();

                let mut stream = resp.bytes_stream();
                while let Some(chunk) = stream.next().await {
                    file.write_all(&chunk.unwrap()).await.unwrap();
                }
            })
            .collect::<Vec<_>>();

        future::join_all(streams).await;
    });
}
