use fast_log::Config;
use log::LevelFilter;

pub fn init_log() {
    //初始化环境变量
    dotenvy::dotenv().ok();

    //设置may的最小线程(由于fast_log使用的是may)
    may::config().set_workers(2);

    fast_log::init(
        Config::new()
            .console()
            .file("log/brave-rust.log")
            .level(LevelFilter::Debug)
            .chan_len(Some(100000)),
    )
    .unwrap();
}
