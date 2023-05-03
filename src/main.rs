use std::convert::Infallible;
use std::io;
use actix_web::{App, HttpServer, middleware, Responder, web};
use actix_web_lab::sse;
use actix_web_lab::sse::Event;
use async_std::task::block_on;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;
use futures_util::stream::StreamExt;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/sse",web::get().to(sse))

    })
        .bind(("0.0.0.0", 9090))?
        .run()
        .await
}

async fn sse() -> impl Responder {

    let (sender_response, receiver_response) = mpsc::channel::<Event>(1000);

    let data_string = r#"{"aId":"12345678","bId":"987654321","cId":"abcd","mTNMoCqZU1":"ZlrWCKb059","3yP7piSDG7":"GtfCgYLgH2","3nuoqDxyRz":{"OucP7ijiVU":0.14,"NWn5eWTRaI":0.0,"r4IOJ70eyL":"jf7NIgxIps"},"z1B2mZkewj":{"cqK8P5PywK":0.0,"WmYFwzxjiY":0.0,"ubsmYvJ5Hg":"IWxVYO9JKf"},"dYHmrI5Y9z":{"I8rQVmnQeb":0.0,"NalXDhKiIa":0.0,"d8PTvwdTWr":"eI35Jtaypy"},"0B95i170aM":{"ZOOqle02qc":0.14,"NIFqhnXQPF":"IzefB2DUMD"},"eI35Jtaypy":{"K25PRv3UQ0":0.14,"crqvveK9et":"vk5tc1h5rV"},"K25PRv3UQ0":{"kOaKR8HL4O":0.14,"IjAsfT3tvI":"jDxH8kt2V9"},"lZThnkPs5H":{"IXbxrs8ogd":{"QEnsb0MHBa":"200","v6RNnCGvSa":"Kt8FMmMm5w"}},"EJSK7OWYCY":{"ghbtUQUWZD":1.0,"rLuBOaDwNU":0.0,"5ZVLA2cudi":0.0,"vo0Kl4voMe":0.0}}"#;
    let _spawn_handle = std::thread::spawn(move || block_on(async {
        loop {
            let data = sse::Data::new(data_string).id(Uuid::new_v4().to_string()).event("dummy_event");
            let send_result = sender_response.send(sse::Event::Data(data)).await;

            if let Err(_) = send_result {
                break;
            }

        }


    }));

    let response_stream = ReceiverStream::new(receiver_response);
    let stream = response_stream.map(|sse_event| {
        Ok::<Event, Infallible>(sse_event)
    });

    sse::Sse::from_stream(stream)


    // let (sender, sse) = sse::channel(2);
    //
    // actix_web::rt::spawn(async move {
    //     loop {
    //         let time = time::OffsetDateTime::now_utc();
    //         let msg = sse::Data::new(time.format(&Rfc3339).unwrap()).event("timestamp");
    //
    //         if sender.send(msg).await.is_err() {
    //             tracing::warn!("client disconnected; could not send SSE message");
    //             break;
    //         }
    //
    //         sleep(Duration::from_secs(10)).await;
    //     }
    // });
    //
    // sse.with_keep_alive(Duration::from_secs(3))
}
