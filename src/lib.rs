use worker::*;

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let user_agent = req.headers().get("User-Agent")?.unwrap_or_else(|| "User-Agent not found".to_string());
    Response::ok(user_agent)
}
