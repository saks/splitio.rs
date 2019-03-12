use lazy_static::lazy_static;
use simple_logger;
use splitio::cache::Redis;
use splitio::Client;

lazy_static! {
    static ref SPLIT_CACHE: Redis =
        Redis::from_url("redis://173.17.0.3").expect("Failed to connect to redis");
    static ref CLIENT: Client<'static, Redis> = Client::new(&*SPLIT_CACHE);
}

fn main() -> Result<(), splitio::errors::Error> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    CLIENT.get_treatment("123", "XXX")?;

    Ok(())
}
