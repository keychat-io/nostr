// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::net::SocketAddr;
use std::sync::Arc;

use nostr::nips::nip05;

use crate::error::Result;
use crate::nips::nip19::Nip19Profile;
use crate::PublicKey;

#[uniffi::export]
pub fn verify_nip05(public_key: &PublicKey, nip05: String, proxy: Option<String>) -> Result<()> {
    let proxy: Option<SocketAddr> = match proxy {
        Some(proxy) => Some(proxy.parse()?),
        None => None,
    };
    Ok(nip05::verify_blocking(**public_key, nip05, proxy)?)
}

#[uniffi::export]
pub fn get_nip05_profile(nip05: String, proxy: Option<String>) -> Result<Arc<Nip19Profile>> {
    let proxy: Option<SocketAddr> = match proxy {
        Some(proxy) => Some(proxy.parse()?),
        None => None,
    };
    Ok(Arc::new(nip05::get_profile_blocking(nip05, proxy)?.into()))
}
