use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::GZipPacker;
use fast_log::Config;
use log::LevelFilter;

pub fn init_log() {
    //初始化环境变量
    dotenvy::dotenv().ok();

    //设置may的最小线程(由于fast_log使用的是may)
    may::config().set_workers(2);

    #[cfg(debug_assertions)]
    let level = LevelFilter::Info;
    #[cfg(not(debug_assertions))]
    let level = LevelFilter::Warn;

    fast_log::init(
        Config::new()
            .console()
            .file_split(
                "logs/",
                LogSize::MB(3),
                RollingType::KeepNum(10),
                GZipPacker {},
            )
            .level(level)
            .chan_len(Some(100000)),
    )
    .unwrap();
}
