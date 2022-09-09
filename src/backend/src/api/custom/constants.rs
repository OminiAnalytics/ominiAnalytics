/*
 File: constants.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 1/09/2022 11:19:8
 Modified By: realbacon
 -----
 Copyright (c) 2022 Omini
 -----
*/

pub const KILL_SESSION_DELAY: u8 = 30; // 30 seconds after last alive signal -> kill session
pub const SIGNAL_TYPE_ALIVE: &str = "alive"; // Signal type: alive
pub const SIGNAL_TYPE_SESSION: &str = "session"; // Signal type: session
pub const MAX_REQUESTS_PER_IP_V4: i16 = 50; // Max requests per IP in one minute for v4 addresses
pub const MAX_REQUESTS_PER_IP_V6: i16 = 20; // Max requests per IP in one minute for v6 addresses
pub const BOT_UA_LIST: &'static [&'static str] = &[
    "googlebot",
    "baiduspider",
    "gurujibot",
    "yandexbot",
    "slurp",
    "msnbot",
    "bingbot",
    "teoma",
    "sogou",
    "exabot",
    "facebot",
    "ia_archiver",
    "facebookexternalhit",
    "AhrefsBot",
    "Googlebot",
    "SemrushBot",
    "BLEXBot",
    "linkedinbot",
    "twitterbot",
    "slackbot",
    "telegrambot",
    "applebot",
    "pingdom",
    "tumblr ",
    "Embedly",
    "spbot",
    "curl",
    "insomnia",
    "python",
    "node",
    "fetch",
    "wget",
    "okhttp",
    "httpie",
    "http.rb",
    "httpclient",
    "http",
    "http4s",
    "httpcore",
    "httpx",
    "httparty",
    "httpbin",
];
