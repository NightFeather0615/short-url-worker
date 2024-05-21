use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  let kv: kv::KvStore = env.kv("ShortUrl")?;

  let target_path: String = req.path().trim_start_matches("/").to_string();

  if target_path.contains("/") || target_path.len() <= 0 {
    return Response::error("Bad URL Path", 400);
  }

  return match kv.get(target_path.as_str()).text().await? {
    Some(dest_url) => Response::redirect(Url::parse(dest_url.as_str())?),
    None => Response::error("Destination Not Found", 404)
  };
}
