use crate::claude::gen_cargo_text_info;
use crate::state::AppState;
use crate::weather::is_raining;
use crate::webdriver::get_webdriver;
use crate::ws_broadcast;
use model::cargo::{Cargo, CargoTextInfoRequest};
use model::news::News;
use model::ws_msg::*;
use std::fs;
use thirtyfour::support::base64_encode;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::{error, info};
use utils::db::db_backup;
use utils::runtime::rand_sleep;
use uuid::Uuid;

fn get_period(job_name: &str) -> &'static str {
    match job_name {
        "launch_rocket" => "every 10 minutes",
        "ship_cargoes" => "every 60 seconds",
        "send_weather" => "every 5 minutes",
        "fetch_remote_news" => "every 6 hours",
        "backup_database" => "every 8 hours",
        "gen_cargo_text_info" => "every 3 seconds",
        "test_short" => "every 20 seconds",
        "test_long" => "every 1 minutes",
        _ => panic!("Unknown job name"),
    }
}

async fn gen_and_update_cargo_text_info_by_id(app_state: &AppState, id: Uuid) {
    info!("start generating text info for {id}");
    let img = fs::read(format!(
        "{}/backend/db/storage/texture/{}.jpg",
        app_state.config.root_dir.as_str(),
        id.to_string()
    ));

    if let Ok(data) = img {
        let _ = Cargo::set_pending_by_id(&app_state.pool, id, true).await;
        let base64_img = base64_encode(&data);
        let api_key = app_state.config.anthropic_api_key.as_str();
        let result = gen_cargo_text_info(api_key, &base64_img).await;
        match result {
            Ok((name, description)) => {
                info!("new text generated:\nname: {name}\ndescription: {description}");
                let _ = Cargo::update_text_info(
                    &app_state.pool,
                    CargoTextInfoRequest {
                        id,
                        name,
                        description,
                    },
                )
                .await;
                let _ = Cargo::set_pending_by_id(&app_state.pool, id, false).await;
            }
            Err(error) => {
                let _ = Cargo::set_pending_by_id(&app_state.pool, id, false).await;
                error!("failed to generate text info: {error:?}");
            }
        }
    } else {
        error!("failed to read cargo image");
    }
}

pub async fn init(app_state: AppState) -> Result<(), JobSchedulerError> {
    let launch_rocket = Job::new_async(get_period("launch_rocket"), {
        let app_state = app_state.clone();
        move |_, _| {
            let sender = app_state.ws_sender.clone();
            let pool = app_state.pool.clone();
            Box::pin(async move {
                info!("Launching rocket");
                let amount = Cargo::launch(&pool).await;
                let msg = WSMsg::launch(amount);
                ws_broadcast(msg, &sender);
            })
        }
    })?;

    let gen_cargo_text_info = Job::new_async(get_period("gen_cargo_text_info"), {
        let app_state = app_state.clone();
        move |_, _| {
            let app_state = app_state.to_owned();
            let pool = app_state.pool.clone();
            Box::pin(async move {
                let cargoes: Vec<Cargo> = Cargo::get_un_docs(&pool).await;

                if cargoes.len() == 0 {
                    return;
                }

                let cargo_ids: Vec<_> = cargoes.iter().map(|c| c.id).collect();

                info!(
                    "updating {} cargoes text info, id list: {cargo_ids:#?}",
                    cargoes.len()
                );

                let tasks: Vec<_> = cargo_ids
                    .into_iter()
                    .map(|id| gen_and_update_cargo_text_info_by_id(&app_state, id))
                    .collect();

                futures::future::join_all(tasks).await;
            })
        }
    })?;

    let ship_cargoes = Job::new_async(get_period("ship_cargoes"), {
        let app_state = app_state.clone();
        move |_, _| {
            let pool = app_state.pool.clone();
            Box::pin(async move {
                info!("Delivering cargoes");
                Cargo::deliver(&pool).await;
            })
        }
    })?;

    let send_weather = Job::new_async(get_period("send_weather"), {
        let app_state = app_state.clone();
        move |_, _| {
            let sender = app_state.ws_sender.clone();
            Box::pin(async move {
                rand_sleep(15000).await;
                info!("Sending weather");
                let raining = is_raining().await;
                match raining {
                    Ok(raining) => {
                        info!("Is it raining outside? {raining}.");
                        let msg = WSMsg::weather(raining);
                        ws_broadcast(msg, &sender);
                    }
                    Err(error) => {
                        error!("Failed to check weather: {error:?}");
                        let msg = WSMsg::weather(false);
                        ws_broadcast(msg, &sender);
                    }
                }
            })
        }
    })?;

    let fetch_remote_news = Job::new_async(get_period("fetch_remote_news"), {
        let app_state = app_state.clone();

        move |_, _| {
            let pool = app_state.pool.clone();
            Box::pin(async move {
                info!("initializing webdriver");
                let webdriver = get_webdriver(app_state.config.wd_port).await;
                rand_sleep(30000).await;
                info!("Fetching remote news");
                if let Ok(driver) = webdriver {
                    if let Err(err) = News::fetch_remote(&pool, &driver).await {
                        error!("Failed to fetch remote news: {err:?}");
                    };
                    if driver.quit().await.is_err() {};
                } else {
                    error!("Failed to get webdriver {:?}", webdriver.err());
                }
            })
        }
    })?;

    let backup_database = Job::new_async(get_period("backup_database"), {
        let app_state = app_state.clone();
        move |_, _| {
            let pool = app_state.pool.clone();
            let root_dir = app_state.config.root_dir.clone();
            Box::pin(async move {
                info!("Backing up database");
                if let Err(error) = db_backup(
                    &pool,
                    vec!["news", "cargo"],
                    &format!("{root_dir}/backend/db/backups"),
                )
                .await
                {
                    error!("Failed to backup database: {error}");
                }
            })
        }
    })?;

    // create scheduler
    let sched = JobScheduler::new().await?;

    // add jobs to scheduler
    sched.add(send_weather).await?;
    sched.add(launch_rocket).await?;
    sched.add(ship_cargoes).await?;
    sched.add(fetch_remote_news).await?;
    sched.add(backup_database).await?;
    sched.add(gen_cargo_text_info).await?;

    // start scheduler
    sched.start().await?;
    Ok(())
}
