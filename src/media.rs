use std::collections::HashMap;
use std::sync::OnceLock;

use gtk::gio::{BusType, Cancellable, DBusCallFlags, DBusProxy, DBusProxyFlags};
use gtk::glib::Variant;
use gtk::prelude::*;

use crate::app::App;
const MPRIS_PREFIX: &str = "org.mpris.MediaPlayer2.";
const PLAYER_PATH: &str = "/org/mpris/MediaPlayer2";
const PLAYER_IFACE: &str = "org.mpris.MediaPlayer2.Player";
const PROPS_IFACE: &str = "org.freedesktop.DBus.Properties";
const TIMEOUT_MS: i32 = 120;

pub static DBUS_PROXY: OnceLock<Option<DBusProxy>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct MediaSource {
    pub app: App,
    pub label: String,
    pub playing: bool,
}

pub fn current_source<T, F>(proc_source: F) -> Option<T>
where
    F: Fn(MediaSource) -> T,
{
    let dbus_proxy = DBUS_PROXY
        .get_or_init(|| {
            DBusProxy::for_bus_sync(
                BusType::Session,
                DBusProxyFlags::DO_NOT_LOAD_PROPERTIES,
                None::<&gtk::gio::DBusInterfaceInfo>,
                "org.freedesktop.DBus",
                "/org/freedesktop/DBus",
                "org.freedesktop.DBus",
                None::<&Cancellable>,
            )
            .ok()
        })
        .as_ref()?;
    let names = dbus_proxy
        .call_sync(
            "ListNames",
            None,
            DBusCallFlags::NONE,
            TIMEOUT_MS,
            None::<&Cancellable>,
        )
        .ok()?
        .child_value(0)
        .get::<Vec<String>>()?;
    let mut fallback_source = None;
    for name in names
        .into_iter()
        .filter(|name| name.starts_with(MPRIS_PREFIX))
    {
        let Some(source) = source_info_from_player(name.as_str()) else {
            continue;
        };
        if source.playing {
            return Some(proc_source(source));
        }
        fallback_source.get_or_insert(source);
    }
    fallback_source.map(proc_source)
}

fn source_info_from_player(name: &str) -> Option<MediaSource> {
    let proxy = DBusProxy::for_bus_sync(
        BusType::Session,
        DBusProxyFlags::DO_NOT_LOAD_PROPERTIES,
        None::<&gtk::gio::DBusInterfaceInfo>,
        name,
        PLAYER_PATH,
        PROPS_IFACE,
        None::<&Cancellable>,
    )
    .ok()?;
    let properties = proxy
        .call_sync(
            "GetAll",
            Some(&(PLAYER_IFACE,).to_variant()),
            DBusCallFlags::NONE,
            TIMEOUT_MS,
            None::<&Cancellable>,
        )
        .ok()?
        .child_value(0)
        .get::<HashMap<String, Variant>>()?;
    let metadata = unbox_variant(properties.get("Metadata")?).get::<HashMap<String, Variant>>()?;
    let title = unbox_variant(metadata.get("xesam:title")?)
        .str()?
        .to_owned();
    if title.is_empty() {
        return None;
    }
    let app = metadata
        .get("mpris:trackid")
        .map(unbox_variant)
        .and_then(|value| value.str().map(str::to_owned))
        .and_then(|str| {
            if str.to_lowercase().contains("firefox") {
                Some(App::FireFox)
            } else {
                None
            }
        })
        .unwrap_or(App::Default);
    let artist = metadata
        .get("xesam:artist")
        .map(unbox_variant)
        .and_then(|value| value.get::<Vec<String>>())
        .and_then(|artists| artists.into_iter().next())
        .filter(|artist| !artist.is_empty());
    let playing = properties
        .get("PlaybackStatus")
        .map(unbox_variant)
        .and_then(|value| value.str().map(str::to_owned))
        .map(|status| status == "Playing")
        .unwrap_or(false);
    let label = match artist {
        Some(artist) => format!("{artist}-{title}"),
        None => title,
    };
    Some(MediaSource {
        app,
        label,
        playing,
    })
}

fn unbox_variant(value: &Variant) -> Variant {
    if value.type_().as_str() == "v" {
        value.as_variant().unwrap_or_else(|| value.clone())
    } else {
        value.clone()
    }
}
