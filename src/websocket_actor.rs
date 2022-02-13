//! Each websocket connection spawns one actor. This actor is responsible for
//! handling the websocket connection.

use crate::{
    messages::{BoxAddr, TraitAddr},
    user::UserUuid,
};

use actix::{
    Actor, ActorContext, AsyncContext, Context, Handler, Message, StreamHandler, Supervised,
    SystemService,
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::{debug, info};
use std::time::{Duration, Instant};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct WebsocketActor {
    hb: Instant,
    uuid: UserUuid,
    backing_actor: BoxAddr,
    last_send: String,
}

impl Actor for WebsocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Register self to get updates to the game state
        // self.backing_actor
        //     .do_send(Subscribe(ctx.address(), self.uuid.clone()));
        start_heartbeat(ctx); // No need to pass self, because the contex knows about it.
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        // // Unregister self
        // self.backing_actor.do_send(Unsubscribe(ctx.address()));
    }
}

/// Heartbeat handler that will kill the process if the client disconnects.
fn start_heartbeat(ctx: &mut ws::WebsocketContext<WebsocketActor>) {
    ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
        // Are we dead yet?
        if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
            println!("Websocket Client heartbeat failed, disconnecting!");
            ctx.stop();
        } else {
            ctx.ping(b"");
        }
    });
}

/// Delegate raw websocket messages to better places.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text), // self.handle_text(text, ctx),
            Ok(ws::Message::Binary(_)) => {
                debug!("Received binary message which is not expected.");
            }
            _ => (),
        }
    }
}

/// Sets up a websocket connection ensuring there is a uuid.
pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    debug!("Websocket connection!");
    if let Some(uuid) = UserUuid::from_query_string(req.query_string()) {
        let router = DynamicRouteService::from_registry();
        let m = GetActorForRoute("/".to_owned());
        debug!("{uuid}");
        let addr = router.send(m).await.map_err(|_| {
            actix_web::error::ErrorInternalServerError(
                "Internal Server Error in the actor system. (Mailbox Error)",
            )
        })?;
        let addr = addr.ok_or_else(|| actix_web::error::ErrorNotFound("Route not found."))?;

        let resp = ws::start(
            WebsocketActor {
                hb: Instant::now(),
                uuid,
                backing_actor: addr,
                last_send: "".to_owned(),
            },
            &req,
            stream,
        );
        return resp;
    }
    info!("No uuid found in query string: {}", req.query_string());
    // Return 401 Unauthorized if we can't find a UUID
    Err(actix_web::error::ErrorUnauthorized(
        "No UUID found in request",
    ))
}

#[derive(Default)]
struct DynamicRouteService;

impl Supervised for DynamicRouteService {}

impl SystemService for DynamicRouteService {}

impl Actor for DynamicRouteService {
    type Context = Context<Self>;
}

struct GetActorForRoute(String);

impl Message for GetActorForRoute {
    type Result = Option<BoxAddr>;
}

/// Finds the actor that is responsible for handling this route.
impl Handler<GetActorForRoute> for DynamicRouteService {
    type Result = Option<BoxAddr>;

    fn handle(&mut self, msg: GetActorForRoute, _: &mut Context<Self>) -> Self::Result {
        Some(BoxAddr(Box::new(DummyActor)))
    }
}

/// Dummy route
#[derive(Debug)]
struct DummyActor;

impl TraitAddr for DummyActor {
    fn send(&self, msg: crate::messages::BoxMsg) {
        todo!()
    }
}
