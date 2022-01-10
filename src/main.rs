//! Web service for primes with large numbers of bits.
//!
//! Bart Massey 2021

mod prime;
use prime::*;

// Web code taken from Tide examples.
// Form taken from Code adapted from *Programming Rust* 2nd ed, Blandy et al 2021.

use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct Bits {
    bits: u32,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/prime").post(prime_bits);
    app.at("/").get(request_prime);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn request_prime(_req: tide::Request<()>) -> tide::Result {
    Ok(tide::Response::builder(tide::StatusCode::Ok)
        .body(
            r#"
            <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
            <html xmlns="http://www.w3.org/1999/xhtml" lang="en" xml:lang="en"><head>
                <meta charset="UTF-8"/>
                <meta name="google" content="notranslate">
                <meta http-equiv="Content-Language" content="en">
            </head><body>
            <title>N-bit Prime</title>
            <form action="/prime" method="post">
                <label>bits:</label>
                <input type="text" name="bits"/>
                <button type="submit">Ok</button>
            </form></body></html>
            "#
        )
        .content_type("text/html;charset=utf-8")
        .build()
    )
}

async fn prime_bits(mut req: tide::Request<()>) -> tide::Result {
    let Bits { bits } = req.body_form().await?;
    let prime = prime(bits);

    Ok(tide::Response::builder(tide::StatusCode::Ok)
        .body(format!(
            r#"
            <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
            <html xmlns="http://www.w3.org/1999/xhtml" lang="en" xml:lang="en"><head>
                <meta charset="UTF-8"/>
                <meta name="google" content="notranslate">
                <meta http-equiv="Content-Language" content="en">
            </head><body>
            <title>{bits}-bit Prime</title>
            <p>{prime:width$x}</p>
            </body></html>
            "#,
            bits = bits,
            prime = prime,
            width = bits as usize / 4,
        ))
        .content_type("text/html;charset=utf-8")
        .build()
    )
}
