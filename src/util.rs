use minreq::{Error, Request, Response};

pub fn get_page(url: &str) -> Result<String, Error> {
    let req: Request = minreq::get(url);
    let res: Response = req.send()?;
    let body: &str = res.as_str()?;
    Ok(String::from(body))
}

pub fn get_image(url: &str) -> Result<Vec<u8>, Error> {
    let req: Request = minreq::get(url);
    let res: Response = req.send()?;
    Ok(res.as_bytes().to_vec())
}
