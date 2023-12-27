use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio_util::task::TaskTracker;

const RANDOM_DATA_LEN: usize = 20;
const CHAN_CAP: usize = 1000;
const PRODUCER_NUM: usize = 10;

fn gen_random_data(length: usize) -> String {
    let rng = thread_rng();
    let rand_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    rand_string
}

pub async fn use_flume_mpsc() {
    let (tx, rx) = flume::bounded(CHAN_CAP);
    let data = gen_random_data(RANDOM_DATA_LEN);
    let tracer = TaskTracker::new();
    (0..PRODUCER_NUM).for_each(|_| {
        let tx = tx.clone();
        let data = data.clone();
        tracer.spawn(async move {
            tx.send_async(data)
                .await
                .expect("send channel nerver fails");
        });
    });
    for _ in 0..PRODUCER_NUM {
        _ = rx.recv_async().await.expect("recv channel nerver fails");
    }
    tracer.close();
    tracer.wait().await;
}

pub async fn use_tokio_mpsc() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(CHAN_CAP);
    let data = gen_random_data(RANDOM_DATA_LEN);
    let tracer = TaskTracker::new();
    (0..PRODUCER_NUM).for_each(|_| {
        let tx = tx.clone();
        let data = data.clone();
        tracer.spawn(async move {
            tx.send(data).await.expect("send channel nerver fails");
        });
    });
    for _ in 0..PRODUCER_NUM {
        _ = rx.recv().await.expect("recv channel nerver fails");
    }
    tracer.close();
    tracer.wait().await;
}
