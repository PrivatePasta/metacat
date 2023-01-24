use google_drive3::api::ChangeList;
use poise::futures_util::future::join;

mod bot;
mod utils;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ChangeList>(10);

    let bot_task = tokio::task::spawn(bot::bot());
    // let watcher_task = tokio::task::spawn(utils::gdrive());

    let watcher_task = tokio::task::spawn(async move {
        // ! Some spaghetti infinite loop - hope google doesn't ban
        // * minor skill issue on my end
        loop {
            let change = utils::get_gdrive_changes().await;
            // println!("{:#?}", change);
            tx.send(change)
                .await
                .expect("Transmission of data out of gdrive thread gone wrong");
        }
    });

    println!("{:#?}", rx.recv().await); // only prints once

    join(bot_task, watcher_task)
        .await
        .0 // I guess we don't care about the other result lol?
        .expect("Issue in joining tokio tasks in main");
}

// avoid vec allocation
// let mut futures = vec![];
// futures.push(bot_task);
// futures.push(watcher_task);

// join_all(futures).await;

// ! spawn_blocking doesn't work because of  futures.push(watcher_task);
// ! |                                               ---- ^^^^^^^^^^^^ expected opaque type, found a different opaque type
// ! |

// let bot_task = tokio::task::spawn_blocking(|| bot::bot());
// let watcher_task = tokio::task::spawn_blocking(|| utils::gdrive());

// let mut futures = vec![];
// futures.push(bot_task);
// futures.push(watcher_task);

// join_all(futures).await;
// bot::bot().await;
// utils::gdrive().await;
