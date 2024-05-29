use zenoh::subscriber::FlumeSubscriber;
use zenoh::publication::Publisher;

pub struct PublisherData {
    pub publisher: Publisher<'static>,
}

pub struct SubscriberData {
    pub subscriber: FlumeSubscriber<'static>,
}
