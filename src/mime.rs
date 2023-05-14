use paste::paste;

pub const JSON: &str = "application/json";

macro_rules! impl_mime {
    ($($format:ident: $mime:expr,)+) => {
        paste! {
            $(
                pub const [<$format:upper>]: &str = $mime;

                pub fn [<render_ $format>]<C: AsRef<str>>(content: C) {
                    println!(
                        "EVCXR_BEGIN_CONTENT {}\n{}\nEVCXR_END_CONTENT",
                        $mime,
                        content.as_ref()
                    );
                }
            )+
        }
    };
}

impl_mime! {
    bmp:    "image/bmp",
    gif:    "image/gif",
    jpeg:   "image/jpeg",
    png:    "image/png",
    svg:    "image/svg+xml",
    html:   "text/html",
}

#[inline]
pub fn render<M, C>(mime: M, content: C)
where
    M: AsRef<str>,
    C: AsRef<str>,
{
    println!(
        "EVCXR_BEGIN_CONTENT {}\n{}\nEVCXR_END_CONTENT",
        mime.as_ref(),
        content.as_ref(),
    );
}

#[rustfmt::skip]
pub fn render_json<S: AsRef<str>>(s: S) {
    let json = serde_json::from_str::<serde_json::Value>(s.as_ref())
        .expect("Fail to parse JSON string");
    let json_str = serde_json::to_string_pretty(&json)
        .expect("Fail to stringify JSON object");
    render(JSON, json_str);
}
