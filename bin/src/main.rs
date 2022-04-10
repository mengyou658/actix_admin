use std::time::Duration;
use actix_web::{App, HttpResponse, HttpServer, middleware, web, error, dev};
use app::{
    apps,
    my_env::{self, RT},
    tasks,
    utils::{self, cert::CERT_KEY},
};
use configs::CFG;
use actix_files::Files;
use actix_web::body::BoxBody;
use actix_web::dev::JsonBody::Body;
use actix_web::dev::ServiceResponse;
use actix_web::http::{header, StatusCode};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
//

use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use app::middleware::handler::{get_error_handlers, get_json_config};
use db::common::res::Res;

// 路由日志追踪

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", &CFG.log.log_level);
    }
    my_env::setup();
    //  设置日志追踪
    // if &CFG.log.log_level == "TRACE" {
    //     LogTracer::builder()
    //         .with_max_level(log::LevelFilter::Trace)
    //         .init()
    //         .unwrap();
    // }

    // 系统变量设置
    let log_env = my_env::get_log_level();

    //  日志设置
    let format = my_env::get_log_format();

    // 文件输出
    let file_appender = tracing_appender::rolling::hourly(&CFG.log.dir, &CFG.log.file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 标准控制台输出
    let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let logger = Registry::default()
        .with(EnvFilter::from_default_env().add_directive(log_env.into()))
        .with(fmt::Layer::default().with_writer(std_non_blocking).event_format(format.clone()).pretty())
        .with(fmt::Layer::default().with_writer(non_blocking).event_format(format))
        // .with(console_layer)
        ;
    tracing::subscriber::set_global_default(logger).unwrap();

    // apis全局初始化
    utils::ApiUtils::init_all_api().await;
    // 定时任务初始化
    tasks::timer_task_init().await.expect("定时任务初始化失败");
    // let metrics = TokioMetrics::new();
    // 启动app  注意中间件顺序 最后的先执行，尤其AddData
    // 顺序不对可能会导致数据丢失，无法在某些位置获取数据

    let app = HttpServer::new(|| {

        // custom `Json` extractor configuration
        // let json_config = web::JsonConfig::default()
        //     // limit request payload size
        //     .limit(4096)
        //     // only accept text/plain content type
        //     .content_type(|mime| mime == mime::TEXT_PLAIN)
        //     // use custom error handler
        //     .error_handler(|err, req| {
        //         error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        //     });

        App::new()
            .app_data(get_json_config())
            // 跨域
            .wrap(middleware::DefaultHeaders::new()
                .add(("Access-Control-Allow-Origin", "*"))
                .add(("Access-Control-Allow-Method", "GET,POST,PUT,DELETE,OPTIONS"))
                .add(("Access-Control-Allow-Credentials", "true"))
            )
            .wrap(get_error_handlers())
            // 静态资源
            .service(Files::new(&CFG.web.upload_url, &CFG.web.upload_dir).prefer_utf8(true))
            // api
            .service(apps::api(web::scope(&CFG.server.api_prefix)))
    });

    let server = app.bind(&CFG.server.address)?.run().await;
    server
}
