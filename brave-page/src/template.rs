use actix_web::{dev, error, web, FromRequest, HttpRequest};
use actix_web_lab::respond::Html;
use brave_config::GLOBAL_ENV_CONFIG;
use minijinja::Source;
use minijinja_autoreload::AutoReloader;
use std::future::{ready, Ready};
use std::path::PathBuf;
use std::{env, fs};

pub(crate) struct MiniJinjaRenderer {
    pub(crate) tmpl_env: web::Data<AutoReloader>,
}

impl MiniJinjaRenderer {
    pub(crate) fn render(
        &self,
        tmpl: &str,
        ctx: impl Into<minijinja::value::Value>,
    ) -> actix_web::Result<Html> {
        self.tmpl_env
            .acquire_env()
            .map_err(|_| error::ErrorInternalServerError("could not acquire template env"))?
            .get_template(tmpl)
            .map_err(|_| error::ErrorInternalServerError("could not find template"))?
            .render(ctx.into())
            .map(Html)
            .map_err(|err| {
                log::error!("{err}");
                error::ErrorInternalServerError("template error")
            })
    }
}

impl FromRequest for MiniJinjaRenderer {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut dev::Payload) -> Self::Future {
        let tmpl_env = <web::Data<AutoReloader>>::extract(req)
            .into_inner()
            .unwrap();

        ready(Ok(Self { tmpl_env }))
    }
}

pub(crate) fn template_init() -> AutoReloader {
    AutoReloader::new(move |notifier| {
        let mut env: minijinja::Environment<'static> = minijinja::Environment::new();

        let tmpl_path = PathBuf::from(env::current_dir().unwrap()).join("templates");

        if let Some(data) = &GLOBAL_ENV_CONFIG.template_autoload {
            if *data {
                notifier.watch_path(&tmpl_path, true);
            }
        };

        //只添加html
        let files = fs::read_dir(tmpl_path)
            .unwrap()
            .filter_map(|entry| {
                let path = entry.unwrap().path();
                if path.extension().map_or(false, |ext| ext == "html") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<PathBuf>>();

        let mut source = Source::new();
        for file in files {
            let str = fs::read_to_string(&file).unwrap();

            let mut filename = String::new();
            filename.push_str("home_");
            let name = file.file_name().unwrap().to_str().unwrap();
            filename.push_str(name);

            source.add_template(filename, str).unwrap();
        }
        env.set_source(source);

        Ok(env)
    })
}

#[test]
fn test_init() {
    template_init();
}
