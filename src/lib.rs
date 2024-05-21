use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  if req.method() != Method::Get {
    return Response::error("Method Not Allowed", 405);
  }

  let target_path: String = req.path().trim_start_matches("/").to_string();

  if target_path.contains("/") || target_path.len() <= 0 {
    return Response::error("Bad URL Path", 400);
  }

  let cache: Cache = Cache::open("shortUrlKv:cache".to_string()).await;

  if let Some(cache_result) = cache.get(CacheKey::Request(&req), true).await? {
    return Ok(cache_result);
  }

  let kv: kv::KvStore = env.kv("ShortUrl")?;

  let mut cache_headers: Headers = Headers::new();
  cache_headers.append("Cache-Control", "public, max-age=86400")?;

  if let Some(dest_url) = kv.get(&target_path).text().await? {
    cache_headers.append("Location", &dest_url)?;

    let mut res: Response = Response::builder()
      .with_headers(cache_headers)
      .with_status(302)
      .ok("Found")?;
    cache.put(CacheKey::Request(&req), res.cloned()?).await?;
    
    return Ok(res);
  } else {
    let mut res: Response = Response::builder()
      .with_headers(cache_headers)
      .with_status(404)
      .ok("Destination Not Found")?;
    cache.put(CacheKey::Request(&req), res.cloned()?).await?;

    return Ok(res);
  }
}
