use clipstash::data::AppDatabase;
use clipstash::web::renderer::Renderer;
use dotenv::dotenv;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite://data.db")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_dir: PathBuf,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

    let _handle = rt.handle().clone();

    rt.block_on(async move {
        println!("This is the Template Directory {:?}", opt.template_dir);
        let renderer = Renderer::new(opt.template_dir);

        let database = AppDatabase::new(&opt.connection_string).await;

        let config = clipstash::RocketConfig { renderer, database };

        let server = clipstash::rocket(config).launch().await;
        match server {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to launch rocket: {}", e);
            }
        }
    });
}
