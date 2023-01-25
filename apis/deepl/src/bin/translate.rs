use deepl::client::{DeepLClient, DeepLSource, DeepLTranlatableLang};

#[tokio::main]
async fn main() {
    let deepl = DeepLClient::from_env().unwrap();
    let source = DeepLSource::new(
        DeepLTranlatableLang::Japanese,
        "こんにちは．今日はものすごく寒かったです．でも仕事はなくなりません．．．まあそこそこ楽しいので大丈夫ですが",
    );
    let translated = deepl
        .translate(source, DeepLTranlatableLang::English)
        .await
        .unwrap();
    println!("{}", translated);
}
