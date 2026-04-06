use crate::{load_config, remove_config, save_config};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Applications() -> Element {
    let mut tables = [0; 4].map(|_| use_signal(|| false));

    let mut keybinds = use_signal(|| {
        let mut k = load_config(Some("niri/config.kdl"), &["binds"]);
        if !k.is_empty() {
            k.remove(0);
            k.pop();
        };
        k
    });

    let mut startups = use_signal(|| {
        let mut s = load_config(Some("niri/config.kdl"), &["spawn-at-startup"]);
        if !s.is_empty() {
            s.pop();
        };
        s
    });

    let defaults = use_signal(|| {
        let mut d = load_config(Some("mimeapps.list"), &[""]);
        if !d.is_empty() {
            d.remove(0);
        };
        d
    });
    let mut defaults_ui = use_signal(|| [const { String::new() }; 8]);
    for default in &defaults() {
        let mut split = default.split('=');
        let key = split.next().unwrap_or("").to_string();
        let value = split.next().unwrap_or("").to_string();
        match key {
            k if k.starts_with("x-scheme-handler/https") => defaults_ui.write()[0] = value,
            k if k.starts_with("inode/directory") => defaults_ui.write()[1] = value,
            k if k.starts_with("x-scheme-handler/mailto") => defaults_ui.write()[2] = value,
            k if k.starts_with("video/mp4") => defaults_ui.write()[3] = value,
            k if k.starts_with("audio/mp3") => defaults_ui.write()[4] = value,
            k if k.starts_with("text/plain") => defaults_ui.write()[5] = value,
            k if k.starts_with("image/png") => defaults_ui.write()[6] = value,
            k if k.starts_with("application/pdf") => defaults_ui.write()[7] = value,
            _ => (),
        }
    }

    rsx! {
        div {
            class: "tab",
            h1 { "Applications" }
            div {
                div {
                    p { "GPU Acceleration" }
                }
                div {
                    p {
                        onclick: move |_| tables[1].toggle(),
                        "Keybinds"
                    }
                    div {
                        max_height: if *tables[1].read() { "300px" } else { "0" },
                        overflow_y: if *tables[1].read() { "auto" },
                        visibility: if !*tables[1].read() { "hidden" },

                        form {
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                let values = evt.values();
                                let value1 = values.first()
                                        .and_then(|(_, v)| if let FormValue::Text(s) = v { Some(s.clone()) } else { None })
                                        .unwrap_or_default();
                                let value2 = values.last()
                                        .and_then(|(_, v)| if let FormValue::Text(s) = v { Some(s.clone()) } else { None })
                                        .unwrap_or_default();
                                let command = format!("    {value1} {{ {value2} }}");
                                save_config(Some("niri/config.kdl"), &["binds"], &command);
                                let mut new_list = load_config(Some("niri/config.kdl"), &["binds"]);
                                if !new_list.is_empty() {
                                    new_list.remove(0);
                                    new_list.pop();
                                }
                                keybinds.set(new_list);
                            },
                            table {
                                font_size: "12px", width: "60vw",
                                tr {
                                    th { input { name: "shortcut", placeholder: "Mod+Space" } }
                                    th { input { name: "command", placeholder: "spawn \"rio\";" } }
                                    th { button { padding: "3px", r#type: "submit", "+" } }
                                }
                                for keybind in &keybinds() {{
                                    let first = keybind.split_whitespace().next().unwrap_or("").to_string();
                                    let inside = keybind.split('{').nth(1).and_then(|s| s.split('}').next()).unwrap_or("").to_string();
                                    rsx! {
                                        tr {
                                            td { "{first}" }
                                            td { "{inside}" }
                                            td { onclick: move |_| {
                                                let first_owned = first.clone();
                                                spawn(async move {
                                                    let confirmed = rfd::AsyncMessageDialog::new()
                                                        .set_level(rfd::MessageLevel::Info)
                                                        .set_title("Confirm Deletion")
                                                        .set_description(format!("Delete keybind: {}?", first_owned))
                                                        .set_buttons(rfd::MessageButtons::YesNo)
                                                        .show()
                                                        .await;
                                                    if confirmed == rfd::MessageDialogResult::Yes {
                                                        remove_config(Some("niri/config.kdl"), &[&first_owned]);
                                                        let mut new_list = load_config(Some("niri/config.kdl"), &["binds"]);
                                                        if !new_list.is_empty() {
                                                            new_list.remove(0);
                                                            new_list.pop();
                                                        }
                                                        keybinds.set(new_list);
                                                    }
                                                });
                                            }, text_align: "center",  width: "6px", "X" }
                                        }
                                    }

                                }}
                            }
                        }
                    }
                }
                div {
                    p {
                        onclick: move |_| tables[2].toggle(),
                        "Autostarts"
                    }
                    div {
                        max_height: if *tables[2].read() { "300px" } else { "0" },
                        overflow_y: if *tables[2].read() { "auto" },
                        visibility: if !*tables[2].read() { "hidden" },
                        form {
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                let values = evt.values();
                                let value = values.first()
                                        .and_then(|(_, v)| if let FormValue::Text(s) = v { Some(s.clone()) } else { None })
                                        .unwrap_or_default();
                                let command = format!("spawn-at-startup \"{value}\"");
                                save_config(Some("niri/config.kdl"), &["// startups"], &command);
                                let mut new_list = load_config(Some("niri/config.kdl"), &["spawn-at-startup"]);
                                if !new_list.is_empty() {
                                    new_list.pop();
                                }
                                startups.set(new_list);
                            },
                            table {
                                font_size: "20px",
                                tr {
                                    th { input { name: "command", placeholder: "xwayland-satellite" } }
                                    th { button { padding: "3px", r#type: "submit", "+" } }
                                }
                                for startup in &startups() {{
                                    let inside = startup.split('"').nth(1).and_then(|s| s.split('"').next()).unwrap_or("").to_string();
                                    rsx! {
                                        tr {
                                            td { "{inside}" }
                                            td { onclick: move |_| {
                                                let inside_owned = inside.clone();
                                                spawn(async move {
                                                    let confirmed = rfd::AsyncMessageDialog::new()
                                                        .set_level(rfd::MessageLevel::Info)
                                                        .set_title("Confirm Deletion")
                                                        .set_description(format!("Delete startup: {}?", inside_owned))
                                                        .set_buttons(rfd::MessageButtons::YesNo)
                                                        .show()
                                                        .await;
                                                    if confirmed == rfd::MessageDialogResult::Yes {
                                                        remove_config(Some("niri/config.kdl"), &[&inside_owned]);
                                                        let mut new_list = load_config(Some("niri/config.kdl"), &["spawn-at-startup"]);
                                                        if !new_list.is_empty() {
                                                            new_list.pop();
                                                        }
                                                        startups.set(new_list);
                                                    }
                                                });
                                            }, text_align: "center",  width: "6px", "X" }
                                        }
                                    }
                                }}
                            }
                        }
                    }
                }
                div {
                    p {
                        onclick: move |_| tables[3].toggle(),
                        "Defaults"
                    }
                    div {
                        max_height: if *tables[3].read() { "300px" } else { "0" },
                        overflow_y: if *tables[3].read() { "auto" },
                        visibility: if !*tables[3].read() { "hidden" },

                        form {
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                let values = evt.values();
                        //                         let first_owned = first.clone();
                        //                         spawn(async move {
                        //                             let confirmed = rfd::AsyncMessageDialog::new()
                        //                                 .set_level(rfd::MessageLevel::Info)
                        //                                 .set_title("Confirm Deletion")
                        //                                 .set_description(format!("Delete keybind: {}?", first_owned))
                        //                                 .set_buttons(rfd::MessageButtons::YesNo)
                        //                                 .show()
                        //                                 .await;
                        //                             if confirmed == rfd::MessageDialogResult::Yes {
                        //                                 let mut new_list = load_config(Some("niri/config.kdl"), &["binds"]);
                        //                                 if !new_list.is_empty() {
                        //                                     new_list.remove(0);
                        //                                     new_list.pop();
                        //                                 }
                        //                                 keybinds.set(new_list);
                        //                             }
                        //                         });
                                // let value1 = values.first()
                                        // .and_then(|(_, v)| if let FormValue::Text(s) = v { Some(s.clone()) } else { None })
                                        // .unwrap_or_default();
                                // let value2 = values.last()
                        //                 .and_then(|(_, v)| if let FormValue::Text(s) = v { Some(s.clone()) } else { None })
                        //                 .unwrap_or_default();
                        //         let command = format!("    {value1} {{ {value2} }}");
                        //         save_config(Some("niri/config.kdl"), &["binds"], &command);
                        //         let mut new_list = load_config(Some("niri/config.kdl"), &["binds"]);
                        //         if !new_list.is_empty() {
                        //             new_list.remove(0);
                        //             new_list.pop();
                        //         }
                        //         keybinds.set(new_list);
                            },
                            table {
                                font_size: "17px", width: "60vw",
                                tr { td { "Browser" } td { "{defaults_ui()[0]}" } }
                                tr { td { "File Manager" } td { "value" } }
                                tr { td { "Mail" } td { "value" } }
                                tr { td { "Videos" } td { "value" } }
                                tr { td { "Audio" } td { "value" } }
                                tr { td { "Text" } td { "value" } }
                                tr { td { "Images" } td { "value" } }
                                tr { td { "PDF" } td { "value" } }
                            }
                        }
                    }
                }
            }
        }
    }
}

// SAVE CONFIG for XDG_CONFIG_HOME/mimeapps.list:
//
// x-scheme-handler/mailto=firefox.desktop;
// x-scheme-handler/https=firefox.desktop;
// x-scheme-handler/http=firefox.desktop;
// video/x-theora+ogg=mpv.desktop;
// video/x-theora=mpv.desktop;
// video/x-ogm+ogg=mpv.desktop;
// video/x-ogm=mpv.desktop;
// video/x-msvideo=mpv.desktop;
// video/x-ms-wvxvideo=mpv.desktop;
// video/x-ms-wmx=mpv.desktop;
// video/x-ms-wmv=mpv.desktop;
// video/x-ms-asf=mpv.desktop;
// video/x-ms-afs=mpv.desktop;
// video/x-mpeg3=mpv.desktop;
// video/x-mpeg2=mpv.desktop;
// video/x-matroska=mpv.desktop;
// video/x-m4v=mpv.desktop;
// video/x-flv=mpv.desktop;
// video/x-flic=mpv.desktop;
// video/x-flc=mpv.desktop;
// video/x-avi=mpv.desktop;
// video/webm=firefox.desktop;mpv.desktop;
// video/vnd.rn-realvideo=mpv.desktop;
// video/vnd.mpegurl=mpv.desktop;
// video/vnd.divx=mpv.desktop;
// video/vnd.avi=mpv.desktop;
// video/quicktime=mpv.desktop;
// video/ogg=firefox.desktop;mpv.desktop;
// video/msvideo=mpv.desktop;
// video/mpeg=mpv.desktop;
// video/mp4v-es=mpv.desktop;
// video/mp4=mpv.desktop;
// video/mp2t=mpv.desktop;
// video/mkv=mpv.desktop;
// video/flv=mpv.desktop;
// video/fli=mpv.desktop;
// video/dv=mpv.desktop;
// video/divx=mpv.desktop;
// video/avi=mpv.desktop;
// video/3gpp2=mpv.desktop;
// video/3gpp=mpv.desktop;
// video/3gp=mpv.desktop;
// text/xml=firefox.desktop;
// text/x-tex=helix.desktop;
// text/x-tcl=helix.desktop;
// text/x-pascal=helix.desktop;
// text/x-moc=helix.desktop;
// text/x-makefile=helix.desktop;
// text/x-java=helix.desktop;
// text/x-csrc=helix.desktop;
// text/x-chdr=helix.desktop;
// text/x-c++src=helix.desktop;
// text/x-c++hdr=helix.desktop;
// text/x-c++=helix.desktop;
// text/x-c=helix.desktop;
// text/plain=helix.desktop;
// text/html=firefox.desktop;
// text/english=helix.desktop;
// inode/directory=org.gnome.Nautilus.desktop;yazi.desktop;
// image/x-xpixmap=com.github.PintaProject.Pinta.desktop;
// image/x-xbitmap=com.github.PintaProject.Pinta.desktop;
// image/x-tga=com.github.PintaProject.Pinta.desktop;
// image/x-targa=com.github.PintaProject.Pinta.desktop;
// image/x-portable-pixmap=com.github.PintaProject.Pinta.desktop;
// image/x-portable-graymap=com.github.PintaProject.Pinta.desktop;
// image/x-portable-bitmap=com.github.PintaProject.Pinta.desktop;
// image/x-portable-anymap=com.github.PintaProject.Pinta.desktop;
// image/x-png=com.github.PintaProject.Pinta.desktop;
// image/x-pcx=com.github.PintaProject.Pinta.desktop;
// image/x-ico=com.github.PintaProject.Pinta.desktop;
// image/x-icb=com.github.PintaProject.Pinta.desktop;
// image/x-gray=com.github.PintaProject.Pinta.desktop;
// image/x-bmp=com.github.PintaProject.Pinta.desktop;
// image/webp=firefox.desktop;
// image/tiff=com.github.PintaProject.Pinta.desktop;
// image/svg+xml=com.github.PintaProject.Pinta.desktop;firefox.desktop;
// image/png=com.github.PintaProject.Pinta.desktop;firefox.desktop;
// image/pjpeg=com.github.PintaProject.Pinta.desktop;
// image/openraster=com.github.PintaProject.Pinta.desktop;
// image/jpg=com.github.PintaProject.Pinta.desktop;
// image/jpeg=com.github.PintaProject.Pinta.desktop;firefox.desktop;
// image/gif=com.github.PintaProject.Pinta.desktop;firefox.desktop;
// image/bmp=com.github.PintaProject.Pinta.desktop;
// image/avif=firefox.desktop;
// audio/x-wavpack=mpv.desktop;
// audio/x-wav=mpv.desktop;
// audio/x-vorbis+ogg=mpv.desktop;
// audio/x-vorbis=mpv.desktop;
// audio/x-tta=mpv.desktop;
// audio/x-shorten=mpv.desktop;
// audio/x-scpls=mpv.desktop;
// audio/x-realaudio=mpv.desktop;
// audio/x-pn-windows-pcm=mpv.desktop;
// audio/x-pn-wav=mpv.desktop;
// audio/x-pn-realaudio=mpv.desktop;
// audio/x-pn-au=mpv.desktop;
// audio/x-pls=mpv.desktop;
// audio/x-musepack=mpv.desktop;
// audio/x-ms-wma=mpv.desktop;
// audio/x-ms-asf=mpv.desktop;
// audio/x-mpg=mpv.desktop;
// audio/x-mpegurl=mpv.desktop;
// audio/x-mp3=mpv.desktop;
// audio/x-mp2=mpv.desktop;
// audio/x-mp1=mpv.desktop;
// audio/x-matroska=mpv.desktop;
// audio/x-m4a=mpv.desktop;
// audio/x-ape=mpv.desktop;
// audio/x-aiff=mpv.desktop;
// audio/x-adpcm=mpv.desktop;
// audio/x-aac=mpv.desktop;
// audio/webm=firefox.desktop;mpv.desktop;
// audio/wav=mpv.desktop;
// audio/vorbis=mpv.desktop;
// audio/vnd.wave=mpv.desktop;
// audio/vnd.rn-realaudio=mpv.desktop;
// audio/vnd.dts.hd=mpv.desktop;
// audio/vnd.dts=mpv.desktop;
// audio/vnd.dolby.heaac.2=mpv.desktop;
// audio/vnd.dolby.heaac.1=mpv.desktop;
// audio/scpls=mpv.desktop;
// audio/rn-mpeg=mpv.desktop;
// audio/opus=mpv.desktop;
// audio/ogg=firefox.desktop;mpv.desktop;
// audio/musepack=mpv.desktop;
// audio/mpg=mpv.desktop;
// audio/mpegurl=mpv.desktop;
// audio/mpeg3=mpv.desktop;
// audio/mpeg2=mpv.desktop;
// audio/mpeg=mpv.desktop;
// audio/mp4=mpv.desktop;
// audio/mp3=mpv.desktop;
// audio/mp2=mpv.desktop;
// audio/mp1=mpv.desktop;
// audio/m4a=mpv.desktop;
// audio/m3u=mpv.desktop;
// audio/flac=firefox.desktop;mpv.desktop;
// audio/eac3=mpv.desktop;
// audio/dv=mpv.desktop;
// audio/amr-wb=mpv.desktop;
// audio/aiff=mpv.desktop;
// audio/ac3=mpv.desktop;
// audio/aac=mpv.desktop;
// audio/AMR=mpv.desktop;
// audio/3gpp2=mpv.desktop;
// audio/3gpp=mpv.desktop;
// application/pdf=firefox.desktop;
