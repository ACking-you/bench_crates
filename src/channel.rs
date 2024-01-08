use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio_util::task::TaskTracker;

const RANDOM_STRING_DATA_LEN: usize = 20;
const CHAN_CAP: usize = 1000;
const PRODUCER_NUM: usize = 10;

pub trait DataProvider {
    type Type: Send + Clone + Unpin + 'static;
    const TYPE_LITERAL: &'static str;

    fn get_random_data() -> Self::Type;
}

pub struct StringData;

impl DataProvider for StringData {
    type Type = String;
    const TYPE_LITERAL: &'static str = "string";

    fn get_random_data() -> Self::Type {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(RANDOM_STRING_DATA_LEN)
            .map(char::from)
            .collect();
        rand_string
    }
}

pub struct U8Data;
impl DataProvider for U8Data {
    type Type = u8;
    const TYPE_LITERAL: &'static str = "u8";
    fn get_random_data() -> Self::Type {
        todo!()
    }
}

pub async fn use_flume_mpmc<T: DataProvider>() {
    let (tx, rx) = flume::bounded(CHAN_CAP);
    let data = T::get_random_data();
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
    (0..PRODUCER_NUM).for_each(|_| {
        let rx = rx.clone();
        tracer.spawn(async move {
            _ = rx.recv_async().await.expect("recv channel nerver fails");
        });
    });
    tracer.close();
    tracer.wait().await;
}

pub async fn use_flume_mpsc<T: DataProvider>() {
    let (tx, rx) = flume::bounded(CHAN_CAP);
    let data = T::get_random_data();
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

pub async fn use_flume_spsc<T: DataProvider>() {
    let (tx, rx) = flume::bounded(CHAN_CAP);
    let data = T::get_random_data();
    let tracer = TaskTracker::new();
    tracer.spawn(async move {
        for _ in 0..PRODUCER_NUM {
            let data = data.clone();
            tx.send_async(data)
                .await
                .expect("send channel nerver fails");
        }
    });

    for _ in 0..PRODUCER_NUM {
        _ = rx.recv_async().await.expect("recv channel nerver fails");
    }
    tracer.close();
    tracer.wait().await;
}

pub async fn use_tokio_mpsc<T: DataProvider>() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(CHAN_CAP);
    let data = T::get_random_data();
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

pub async fn use_tokio_spsc<T: DataProvider>() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(CHAN_CAP);
    let data = T::get_random_data();
    let tracer = TaskTracker::new();
    tracer.spawn(async move {
        for _ in 0..PRODUCER_NUM {
            let data = data.clone();
            tx.send(data).await.expect("send channel nerver fails");
        }
    });

    for _ in 0..PRODUCER_NUM {
        _ = rx.recv().await.expect("recv channel nerver fails");
    }
    tracer.close();
    tracer.wait().await;
}
