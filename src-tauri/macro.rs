#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn greet(name: &str) -> String {
    {
        let res = ::alloc::fmt::format(
            format_args!("Hello, {0}! You\'ve been greeted from Rust!", name),
        );
        res
    }
}
#[allow(unused_imports)]
use __cmd__greet;
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let s = Some("string".to_string());
            let s2 = s.clone();
            match &app.config().bundle.license {
                tmp => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}] {2} = {3:#?}\n",
                                "src\\main.rs",
                                15u32,
                                "&app.config().bundle.license",
                                &tmp,
                            ),
                        );
                    };
                    tmp
                }
            };
            Ok(())
        })
        .invoke_handler(move |__tauri_invoke__| {
            let __tauri_cmd__ = __tauri_invoke__.message.command();
            match __tauri_cmd__ {
                "greet" => {
                    #[allow(unused_imports)]
                    use ::tauri::ipc::private::*;
                    #[allow(unused_variables)]
                    let ::tauri::ipc::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                        acl: __tauri_acl__,
                    } = __tauri_invoke__;
                    let result = greet(
                        match ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                            plugin: ::core::option::Option::None,
                            name: "greet",
                            key: "name",
                            message: &__tauri_message__,
                            acl: &__tauri_acl__,
                        }) {
                            Ok(arg) => arg,
                            Err(err) => {
                                __tauri_resolver__.invoke_error(err);
                                return true;
                            }
                        },
                    );
                    let kind = (&result).blocking_kind();
                    kind.block(result, __tauri_resolver__);
                    return true;
                }
                _ => {
                    return false;
                }
            }
        })
        .run({
            #[allow(unused_mut, clippy::let_and_return)]
            let mut context = ::tauri::Context::new(
                ::tauri::utils::config::Config {
                    schema: None,
                    product_name: ::core::option::Option::Some("ENTER NAME HERE".into()),
                    version: ::core::option::Option::Some("0.0.0".into()),
                    identifier: "com.tauri.betatest".into(),
                    app: ::tauri::utils::config::AppConfig {
                        windows: <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::tauri::utils::config::WindowConfig {
                                    label: "main".into(),
                                    url: ::tauri::utils::config::WebviewUrl::App(
                                        ::std::path::PathBuf::from("index.html"),
                                    ),
                                    user_agent: ::core::option::Option::None,
                                    file_drop_enabled: true,
                                    center: false,
                                    x: ::core::option::Option::None,
                                    y: ::core::option::Option::None,
                                    width: 800f64,
                                    height: 600f64,
                                    min_width: ::core::option::Option::None,
                                    min_height: ::core::option::Option::None,
                                    max_width: ::core::option::Option::None,
                                    max_height: ::core::option::Option::None,
                                    resizable: true,
                                    maximizable: true,
                                    minimizable: true,
                                    closable: true,
                                    title: "beta-desktop".into(),
                                    proxy_url: ::core::option::Option::None,
                                    fullscreen: false,
                                    focus: true,
                                    transparent: false,
                                    maximized: false,
                                    visible: true,
                                    decorations: true,
                                    always_on_bottom: false,
                                    always_on_top: false,
                                    visible_on_all_workspaces: false,
                                    content_protected: false,
                                    skip_taskbar: false,
                                    theme: ::core::option::Option::None,
                                    title_bar_style: ::tauri::utils::TitleBarStyle::Visible,
                                    hidden_title: false,
                                    accept_first_mouse: false,
                                    tabbing_identifier: ::core::option::Option::None,
                                    additional_browser_args: ::core::option::Option::None,
                                    shadow: true,
                                    window_effects: ::core::option::Option::None,
                                    incognito: false,
                                    parent: ::core::option::Option::None,
                                },
                            ]),
                        ),
                        security: ::tauri::utils::config::SecurityConfig {
                            csp: ::core::option::Option::None,
                            dev_csp: ::core::option::Option::None,
                            freeze_prototype: false,
                            dangerous_disable_asset_csp_modification: ::tauri::utils::config::DisabledCspModificationKind::Flag(
                                false,
                            ),
                            asset_protocol: ::tauri::utils::config::AssetProtocolConfig {
                                scope: ::tauri::utils::config::FsScope::AllowedPaths(
                                    ::alloc::vec::Vec::new(),
                                ),
                                ..Default::default()
                            },
                            pattern: ::tauri::utils::config::PatternKind::Brownfield,
                        },
                        tray_icon: ::core::option::Option::None,
                        macos_private_api: false,
                        with_global_tauri: false,
                    },
                    build: ::tauri::utils::config::BuildConfig {
                        runner: None,
                        dev_url: ::core::option::Option::Some(
                            "http://localhost:1420/".parse().unwrap(),
                        ),
                        frontend_dist: ::core::option::Option::Some(
                            ::tauri::utils::config::FrontendDist::Directory(
                                ::std::path::PathBuf::from("../dist"),
                            ),
                        ),
                        before_dev_command: None,
                        before_build_command: None,
                        before_bundle_command: None,
                        features: None,
                    },
                    bundle: ::tauri::utils::config::BundleConfig {
                        active: true,
                        publisher: None,
                        icon: <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                "icons/32x32.png".into(),
                                "icons/128x128.png".into(),
                                "icons/128x128@2x.png".into(),
                                "icons/icon.icns".into(),
                                "icons/icon.ico".into(),
                            ]),
                        ),
                        targets: Default::default(),
                        resources: None,
                        copyright: None,
                        category: None,
                        license: ::core::option::Option::Some("MIT LOL"),
                        license_file: ::core::option::Option::None,
                        file_associations: None,
                        short_description: None,
                        long_description: None,
                        external_bin: ::core::option::Option::None,
                        windows: ::tauri::utils::config::WindowsConfig {
                            webview_install_mode: ::tauri::utils::config::WebviewInstallMode::DownloadBootstrapper {
                                silent: true,
                            },
                            ..Default::default()
                        },
                        linux: Default::default(),
                        macos: Default::default(),
                        ios: Default::default(),
                        android: Default::default(),
                    },
                    plugins: ::tauri::utils::config::PluginConfig({
                        let mut map = ::std::collections::HashMap::new();
                        map.insert(
                            "shell".into(),
                            ::serde_json::Value::Object({
                                let mut map = ::serde_json::Map::new();
                                map.insert("open".into(), ::serde_json::Value::Bool(true));
                                map
                            }),
                        );
                        map
                    }),
                },
                ::std::boxed::Box::new({
                    #[allow(unused_imports)]
                    use ::tauri::utils::assets::{
                        CspHash, EmbeddedAssets, phf, phf::phf_map,
                    };
                    EmbeddedAssets::new(
                        phf::Map {
                            key: 12913932095322966823u64,
                            disps: &[],
                            entries: &[],
                        },
                        &[],
                        phf::Map {
                            key: 12913932095322966823u64,
                            disps: &[],
                            entries: &[],
                        },
                    )
                }),
                ::std::option::Option::Some(::tauri::Icon::Rgba {
                    rgba: b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xfe\xc12\x00\xff\xc11\x00\xff\xc11\x13\xff\xc11U\xff\xc11\x88\xff\xc11\x8b\xff\xc11\\\xff\xc11\x18\xff\xc10\x00\xfe\xc12\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xfe\xc12\x00\xff\xc11\x00\xff\xc110\xff\xc11\xbe\xff\xc11\xf1\xff\xc11\xd7\xff\xc11\xd4\xff\xc11\xf0\xff\xc11\xca\xff\xc11=\xff\xc11\x00\xff\xc11\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff\xc11\x00\xff\xc11\x1b\xff\xc11\xc6\xff\xc11\xd6\xff\xc11O\xff\xc11\x13\xff\xc11\x11\xff\xc11D\xff\xc11\xc9\xff\xc11\xd4\xff\xc11&\xff\xc11\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xfe\xc10\x00\xff\xc1,4\xf5\xc18s\xf6\xc18)\xe6\xc2E\x00\xff\xc11\x12\xff\xc11\x15\xff\xc11\x00\xff\xc112\xff\xc11\xe4\xff\xc11\x84\xff\xc11\x00\xff\xc11\x00\x00\x00\x00\x00\x00\x00\x00\x00$\xc8\xdb\x00&\xc8\xda\x08\'\xc8\xd9[ \xc8\xde\x89\x00\xca\xff\x0b\xff\xc1-\x1c\xff\xc11\xbd\xff\xc11\xca\xff\xc11*\xf9\xc15\x00\xff\xc11\xaa\xff\xc11\xc0\xff\xc11\x08\xff\xc11\x00\x00\x00\x00\x00$\xc8\xdb\x00$\xc8\xdb\x06$\xc8\xdb\x81$\xc8\xdb\xf4$\xc8\xdb\x9a\x00\xd0\xff\x02\xff\xc100\xff\xc11\xeb\xff\xc11\xf5\xff\xc11C\xff\xc11\x00\xff\xc11\x9c\xff\xc11\xc9\xff\xc11\x0c\xff\xc11\x00%\xc8\xda\x00$\xc8\xdb\x00$\xc8\xdbN$\xc8\xdb\xef$\xc8\xdb\x84$\xc8\xdb\x0cy\xc5\x98\x00\xff\xc1.\x05\xff\xc11P\xff\xc11Y\xff\xc11\x08\xff\xc11\x12\xff\xc11\xcb\xff\xc11\xa4\xfe\xc12\x02\xff\xc11\x00$\xc8\xdb\x00%\xc8\xdb\x02$\xc8\xdb\xa6$\xc8\xdb\xca$\xc8\xdb\x12$\xc8\xdb\x08$\xc8\xdbV$\xc8\xdbL \xc8\xde\x04\xaf\xc4o\x00\xff\xc11\x0b\xff\xc11\x84\xff\xc11\xef\xff\xc11M\xff\xc11\x00\xfd\xc12\x00$\xc8\xdb\x00$\xc8\xdb\r$\xc8\xdb\xcb$\xc8\xdb\x99$\xc8\xdb\x00$\xc8\xdbC$\xc8\xdb\xf5$\xc8\xdb\xea\"\xc8\xdd.\xff\xbb\x00\x03\xff\xc11\x9a\xff\xc11\xf4\xff\xc11\x82\xff\xc11\x06\xff\xc11\x00\x00\x00\x00\x00$\xc8\xdb\x00$\xc8\xdb\t$\xc8\xdb\xc2$\xc8\xdb\xa6\"\xc8\xdc\x00$\xc8\xdb,$\xc8\xdb\xcd$\xc8\xdb\xbf\x1e\xc8\xe0\x1d\xff\xc0\x0e\x0c\xff\xc1.\x8d\xfb\xc13]\xfe\xc12\x08\xff\xc11\x00\x00\x00\x00\x00\x00\x00\x00\x00%\xc8\xdb\x00#\xc8\xdb\x00$\xc8\xdb\x88$\xc8\xdb\xe2$\xc8\xdb/$\xc8\xdb\x00$\xc8\xdb\x17$\xc8\xdb\x14A\xc7\xc5\x00.\xc8\xd3(/\xc8\xd3q\x1f\xc8\xdf3#\xc8\xdc\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00$\xc8\xdb\x00$\xc8\xdb)$\xc8\xdb\xd8$\xc8\xdb\xc5$\xc8\xdb?$\xc8\xdb\x0e$\xc8\xdb\x11$\xc8\xdbM$\xc8\xdb\xd4$\xc8\xdb\xc6$\xc8\xdb\x1b$\xc8\xdb\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00%\xc8\xdb\x00#\xc8\xdb\x00$\xc8\xdbA$\xc8\xdb\xce$\xc8\xdb\xef$\xc8\xdb\xd1$\xc8\xdb\xd4$\xc8\xdb\xf1$\xc8\xdb\xc0$\xc8\xdb0$\xc8\xdb\x00%\xc8\xdb\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00%\xc8\xdb\x00#\xc8\xdb\x00$\xc8\xdb\x1b$\xc8\xdba$\xc8\xdb\x8f$\xc8\xdb\x8c$\xc8\xdbX$\xc8\xdb\x14$\xc8\xdb\x00%\xc8\xda\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                        .to_vec(),
                    width: 16u32,
                    height: 16u32,
                }),
                ::std::option::Option::None,
                ::tauri::PackageInfo {
                    name: "ENTER NAME HERE".to_string(),
                    version: "0.0.0".to_string().parse().unwrap(),
                    authors: "you",
                    description: "A Tauri App",
                    crate_name: "beta-desktop",
                },
                (),
                ::tauri::Pattern::Brownfield(std::marker::PhantomData),
                ::tauri::utils::acl::resolved::Resolved {
                    acl: {
                        let mut map = ::std::collections::BTreeMap::new();
                        map.insert(
                            "app".into(),
                            ::tauri::utils::acl::plugin::Manifest {
                                default_permission: ::core::option::Option::Some(::tauri::utils::acl::PermissionSet {
                                    identifier: "default".into(),
                                    description: "Default permissions for the plugin.".into(),
                                    permissions: <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            "allow-version".into(),
                                            "allow-name".into(),
                                            "allow-tauri-version".into(),
                                        ]),
                                    ),
                                }),
                                permissions: {
                                    let mut map = ::std::collections::BTreeMap::new();
                                    map.insert(
                                        "allow-app-hide".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-app-hide".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the app_hide command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["app_hide".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-app-show".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-app-show".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the app_show command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["app_show".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-name".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-name".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the name command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["name".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-tauri-version".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-tauri-version".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the tauri_version command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["tauri_version".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-version".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-version".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the version command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["version".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-app-hide".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-app-hide".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the app_hide command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["app_hide".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-app-show".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-app-show".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the app_show command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["app_show".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-name".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-name".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the name command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["name".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-tauri-version".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-tauri-version".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the tauri_version command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["tauri_version".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-version".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-version".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the version command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["version".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map
                                },
                                permission_sets: ::std::collections::BTreeMap::new(),
                                global_scope_schema: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            "event".into(),
                            ::tauri::utils::acl::plugin::Manifest {
                                default_permission: ::core::option::Option::Some(::tauri::utils::acl::PermissionSet {
                                    identifier: "default".into(),
                                    description: "Default permissions for the plugin.".into(),
                                    permissions: <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            "allow-listen".into(),
                                            "allow-unlisten".into(),
                                            "allow-emit".into(),
                                            "allow-emit-to".into(),
                                        ]),
                                    ),
                                }),
                                permissions: {
                                    let mut map = ::std::collections::BTreeMap::new();
                                    map.insert(
                                        "allow-emit".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-emit".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the emit command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["emit".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-emit-to".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-emit-to".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the emit_to command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["emit_to".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-listen".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-listen".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the listen command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["listen".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-unlisten".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-unlisten".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the unlisten command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["unlisten".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-emit".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-emit".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the emit command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["emit".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-emit-to".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-emit-to".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the emit_to command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["emit_to".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-listen".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-listen".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the listen command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["listen".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-unlisten".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-unlisten".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the unlisten command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["unlisten".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map
                                },
                                permission_sets: ::std::collections::BTreeMap::new(),
                                global_scope_schema: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            "menu".into(),
                            ::tauri::utils::acl::plugin::Manifest {
                                default_permission: ::core::option::Option::Some(::tauri::utils::acl::PermissionSet {
                                    identifier: "default".into(),
                                    description: "Default permissions for the plugin.".into(),
                                    permissions: ::alloc::vec::Vec::new(),
                                }),
                                permissions: {
                                    let mut map = ::std::collections::BTreeMap::new();
                                    map.insert(
                                        "allow-append".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-append".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the append command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["append".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-create-default".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-create-default".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the create_default command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["create_default".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-get".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-get".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the get command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["get".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-insert".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-insert".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the insert command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["insert".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-checked".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-checked".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_checked command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_checked".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-enabled".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-enabled".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_enabled command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_enabled".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-items".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-items".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the items command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["items".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-new".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-new".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the new command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["new".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-popup".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-popup".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the popup command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["popup".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-prepend".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-prepend".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the prepend command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["prepend".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-remove".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-remove".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the remove command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["remove".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-remove-at".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-remove-at".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the remove_at command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["remove_at".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-accelerator".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-accelerator".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_accelerator command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_accelerator".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-as-app-menu".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-as-app-menu".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_as_app_menu command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_as_app_menu".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-as-help-menu-for-nsapp".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-as-help-menu-for-nsapp".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_as_help_menu_for_nsapp command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_as_help_menu_for_nsapp".into(),
                                                    ]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-as-window-menu".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-as-window-menu".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_as_window_menu command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_as_window_menu".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-as-windows-menu-for-nsapp".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-as-windows-menu-for-nsapp".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_as_windows_menu_for_nsapp command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_as_windows_menu_for_nsapp".into(),
                                                    ]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-checked".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-checked".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_checked command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_checked".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-enabled".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-enabled".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_enabled command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_enabled".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-icon".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-icon".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_icon command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_icon".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-text".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-text".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_text command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_text".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-text".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-text".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the text command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["text".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-append".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-append".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the append command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["append".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-create-default".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-create-default".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the create_default command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["create_default".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-get".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-get".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the get command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["get".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-insert".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-insert".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the insert command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["insert".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-checked".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-checked".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_checked command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_checked".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-enabled".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-enabled".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_enabled command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_enabled".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-items".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-items".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the items command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["items".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-new".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-new".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the new command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["new".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-popup".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-popup".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the popup command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["popup".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-prepend".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-prepend".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the prepend command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["prepend".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-remove".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-remove".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the remove command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["remove".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-remove-at".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-remove-at".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the remove_at command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["remove_at".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-accelerator".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-accelerator".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_accelerator command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_accelerator".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-as-app-menu".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-as-app-menu".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_as_app_menu command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_as_app_menu".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-as-help-menu-for-nsapp".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-as-help-menu-for-nsapp".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_as_help_menu_for_nsapp command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_as_help_menu_for_nsapp".into(),
                                                    ]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-as-window-menu".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-as-window-menu".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_as_window_menu command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_as_window_menu".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-as-windows-menu-for-nsapp".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-as-windows-menu-for-nsapp".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_as_windows_menu_for_nsapp command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_as_windows_menu_for_nsapp".into(),
                                                    ]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-checked".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-checked".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_checked command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_checked".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-enabled".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-enabled".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_enabled command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_enabled".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-icon".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-icon".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_icon command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_icon".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-text".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-text".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_text command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_text".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-text".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-text".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the text command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["text".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map
                                },
                                permission_sets: ::std::collections::BTreeMap::new(),
                                global_scope_schema: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            "path".into(),
                            ::tauri::utils::acl::plugin::Manifest {
                                default_permission: ::core::option::Option::Some(::tauri::utils::acl::PermissionSet {
                                    identifier: "default".into(),
                                    description: "Default permissions for the plugin.".into(),
                                    permissions: <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            "allow-resolve-directory".into(),
                                            "allow-resolve".into(),
                                            "allow-normalize".into(),
                                            "allow-join".into(),
                                            "allow-dirname".into(),
                                            "allow-extname".into(),
                                            "allow-basename".into(),
                                            "allow-is-absolute".into(),
                                        ]),
                                    ),
                                }),
                                permissions: {
                                    let mut map = ::std::collections::BTreeMap::new();
                                    map.insert(
                                        "allow-basename".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-basename".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the basename command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["basename".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-dirname".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-dirname".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the dirname command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["dirname".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-extname".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-extname".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the extname command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["extname".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-absolute".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-absolute".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_absolute command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_absolute".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-join".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-join".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the join command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["join".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-normalize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-normalize".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the normalize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["normalize".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-resolve".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-resolve".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the resolve command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["resolve".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-resolve-directory".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-resolve-directory".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the resolve_directory command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["resolve_directory".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-basename".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-basename".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the basename command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["basename".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-dirname".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-dirname".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the dirname command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["dirname".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-extname".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-extname".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the extname command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["extname".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-absolute".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-absolute".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_absolute command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_absolute".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-join".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-join".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the join command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["join".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-normalize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-normalize".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the normalize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["normalize".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-resolve".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-resolve".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the resolve command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["resolve".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-resolve-directory".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-resolve-directory".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the resolve_directory command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["resolve_directory".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map
                                },
                                permission_sets: ::std::collections::BTreeMap::new(),
                                global_scope_schema: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            "resources".into(),
                            ::tauri::utils::acl::plugin::Manifest {
                                default_permission: ::core::option::Option::Some(::tauri::utils::acl::PermissionSet {
                                    identifier: "default".into(),
                                    description: "Default permissions for the plugin.".into(),
                                    permissions: <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new(["allow-close".into()]),
                                    ),
                                }),
                                permissions: {
                                    let mut map = ::std::collections::BTreeMap::new();
                                    map.insert(
                                        "allow-close".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-close".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the close command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["close".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-close".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-close".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the close command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["close".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map
                                },
                                permission_sets: ::std::collections::BTreeMap::new(),
                                global_scope_schema: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            "tray".into(),
                            ::tauri::utils::acl::plugin::Manifest {
                                default_permission: ::core::option::Option::Some(::tauri::utils::acl::PermissionSet {
                                    identifier: "default".into(),
                                    description: "Default permissions for the plugin.".into(),
                                    permissions: ::alloc::vec::Vec::new(),
                                }),
                                permissions: {
                                    let mut map = ::std::collections::BTreeMap::new();
                                    map.insert(
                                        "allow-new".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-new".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the new command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["new".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-icon".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-icon".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_icon command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_icon".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-icon-as-template".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-icon-as-template".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_icon_as_template command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_icon_as_template".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-menu".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-menu".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_menu command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_menu".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-show-menu-on-left-click".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-show-menu-on-left-click".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_show_menu_on_left_click command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_show_menu_on_left_click".into(),
                                                    ]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-temp-dir-path".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-temp-dir-path".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_temp_dir_path command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_temp_dir_path".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-title".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-title".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_title command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_title".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-tooltip".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-tooltip".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_tooltip command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_tooltip".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-visible".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-visible".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_visible command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_visible".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-new".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-new".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the new command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["new".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-icon".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-icon".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_icon command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_icon".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-icon-as-template".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-icon-as-template".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_icon_as_template command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_icon_as_template".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-menu".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-menu".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_menu command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_menu".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-show-menu-on-left-click".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-show-menu-on-left-click".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_show_menu_on_left_click command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_show_menu_on_left_click".into(),
                                                    ]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-temp-dir-path".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-temp-dir-path".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_temp_dir_path command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_temp_dir_path".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-title".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-title".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_title command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_title".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-tooltip".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-tooltip".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_tooltip command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_tooltip".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-visible".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-visible".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_visible command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_visible".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map
                                },
                                permission_sets: ::std::collections::BTreeMap::new(),
                                global_scope_schema: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            "webview".into(),
                            ::tauri::utils::acl::plugin::Manifest {
                                default_permission: ::core::option::Option::Some(::tauri::utils::acl::PermissionSet {
                                    identifier: "default".into(),
                                    description: "Default permissions for the plugin.".into(),
                                    permissions: <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            "allow-webview-position".into(),
                                            "allow-webview-size".into(),
                                            "allow-internal-toggle-devtools".into(),
                                        ]),
                                    ),
                                }),
                                permissions: {
                                    let mut map = ::std::collections::BTreeMap::new();
                                    map.insert(
                                        "allow-create-webview".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-create-webview".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the create_webview command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["create_webview".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-create-webview-window".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-create-webview-window".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the create_webview_window command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["create_webview_window".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-internal-toggle-devtools".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-internal-toggle-devtools".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the internal_toggle_devtools command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "internal_toggle_devtools".into(),
                                                    ]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-print".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-print".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the print command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["print".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-webview-focus".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-webview-focus".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_webview_focus command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_webview_focus".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-webview-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-webview-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_webview_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_webview_position".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-webview-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-webview-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_webview_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_webview_size".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-webview-close".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-webview-close".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the webview_close command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["webview_close".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-webview-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-webview-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the webview_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["webview_position".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-webview-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-webview-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the webview_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["webview_size".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-create-webview".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-create-webview".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the create_webview command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["create_webview".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-create-webview-window".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-create-webview-window".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the create_webview_window command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["create_webview_window".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-internal-toggle-devtools".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-internal-toggle-devtools".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the internal_toggle_devtools command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "internal_toggle_devtools".into(),
                                                    ]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-print".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-print".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the print command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["print".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-webview-focus".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-webview-focus".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_webview_focus command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_webview_focus".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-webview-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-webview-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_webview_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_webview_position".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-webview-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-webview-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_webview_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_webview_size".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-webview-close".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-webview-close".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the webview_close command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["webview_close".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-webview-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-webview-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the webview_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["webview_position".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-webview-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-webview-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the webview_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["webview_size".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map
                                },
                                permission_sets: ::std::collections::BTreeMap::new(),
                                global_scope_schema: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            "window".into(),
                            ::tauri::utils::acl::plugin::Manifest {
                                default_permission: ::core::option::Option::Some(::tauri::utils::acl::PermissionSet {
                                    identifier: "default".into(),
                                    description: "Default permissions for the plugin.".into(),
                                    permissions: <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            "allow-scale-factor".into(),
                                            "allow-inner-position".into(),
                                            "allow-outer-position".into(),
                                            "allow-inner-size".into(),
                                            "allow-outer-size".into(),
                                            "allow-is-fullscreen".into(),
                                            "allow-is-minimized".into(),
                                            "allow-is-maximized".into(),
                                            "allow-is-focused".into(),
                                            "allow-is-decorated".into(),
                                            "allow-is-resizable".into(),
                                            "allow-is-maximizable".into(),
                                            "allow-is-minimizable".into(),
                                            "allow-is-closable".into(),
                                            "allow-is-visible".into(),
                                            "allow-title".into(),
                                            "allow-current-monitor".into(),
                                            "allow-primary-monitor".into(),
                                            "allow-available-monitors".into(),
                                            "allow-theme".into(),
                                            "allow-internal-toggle-maximize".into(),
                                            "allow-internal-on-mousemove".into(),
                                            "allow-internal-on-mousedown".into(),
                                        ]),
                                    ),
                                }),
                                permissions: {
                                    let mut map = ::std::collections::BTreeMap::new();
                                    map.insert(
                                        "allow-available-monitors".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-available-monitors".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the available_monitors command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["available_monitors".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-center".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-center".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the center command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["center".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-close".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-close".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the close command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["close".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-create".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-create".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the create command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["create".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-current-monitor".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-current-monitor".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the current_monitor command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["current_monitor".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-destroy".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-destroy".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the destroy command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["destroy".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-hide".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-hide".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the hide command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["hide".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-inner-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-inner-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the inner_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["inner_position".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-inner-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-inner-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the inner_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["inner_size".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-internal-on-mousedown".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-internal-on-mousedown".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the internal_on_mousedown command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["internal_on_mousedown".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-internal-on-mousemove".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-internal-on-mousemove".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the internal_on_mousemove command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["internal_on_mousemove".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-internal-toggle-maximize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-internal-toggle-maximize".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the internal_toggle_maximize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "internal_toggle_maximize".into(),
                                                    ]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-closable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-closable".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_closable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_closable".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-decorated".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-decorated".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_decorated command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_decorated".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-focused".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-focused".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_focused command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_focused".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-fullscreen".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-fullscreen".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_fullscreen command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_fullscreen".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-maximizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-maximizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_maximizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_maximizable".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-maximized".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-maximized".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_maximized command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_maximized".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-minimizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-minimizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_minimizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_minimizable".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-minimized".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-minimized".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_minimized command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_minimized".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-resizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-resizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_resizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_resizable".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-is-visible".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-is-visible".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the is_visible command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_visible".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-maximize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-maximize".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the maximize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["maximize".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-minimize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-minimize".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the minimize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["minimize".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-outer-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-outer-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the outer_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["outer_position".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-outer-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-outer-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the outer_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["outer_size".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-primary-monitor".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-primary-monitor".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the primary_monitor command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["primary_monitor".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-request-user-attention".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-request-user-attention".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the request_user_attention command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["request_user_attention".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-scale-factor".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-scale-factor".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the scale_factor command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["scale_factor".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-always-on-bottom".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-always-on-bottom".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_always_on_bottom command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_always_on_bottom".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-always-on-top".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-always-on-top".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_always_on_top command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_always_on_top".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-closable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-closable".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_closable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_closable".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-content-protected".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-content-protected".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_content_protected command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_content_protected".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-cursor-grab".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-cursor-grab".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_cursor_grab command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_cursor_grab".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-cursor-icon".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-cursor-icon".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_cursor_icon command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_cursor_icon".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-cursor-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-cursor-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_cursor_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_cursor_position".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-cursor-visible".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-cursor-visible".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_cursor_visible command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_cursor_visible".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-decorations".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-decorations".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_decorations command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_decorations".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-effects".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-effects".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_effects command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_effects".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-focus".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-focus".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_focus command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_focus".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-fullscreen".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-fullscreen".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_fullscreen command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_fullscreen".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-icon".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-icon".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_icon command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_icon".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-ignore-cursor-events".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-ignore-cursor-events".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_ignore_cursor_events command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_ignore_cursor_events".into(),
                                                    ]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-max-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-max-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_max_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_max_size".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-maximizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-maximizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_maximizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_maximizable".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-min-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-min-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_min_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_min_size".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-minimizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-minimizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_minimizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_minimizable".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_position".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-progress-bar".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-progress-bar".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_progress_bar command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_progress_bar".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-resizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-resizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_resizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_resizable".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-shadow".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-shadow".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_shadow command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_shadow".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_size".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-skip-taskbar".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-skip-taskbar".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_skip_taskbar command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_skip_taskbar".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-title".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-title".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_title command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_title".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-set-visible-on-all-workspaces".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-set-visible-on-all-workspaces".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the set_visible_on_all_workspaces command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_visible_on_all_workspaces".into(),
                                                    ]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-show".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-show".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the show command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["show".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-start-dragging".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-start-dragging".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the start_dragging command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["start_dragging".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-theme".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-theme".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the theme command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["theme".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-title".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-title".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the title command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["title".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-toggle-maximize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-toggle-maximize".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the toggle_maximize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["toggle_maximize".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-unmaximize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-unmaximize".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the unmaximize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["unmaximize".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "allow-unminimize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "allow-unminimize".into(),
                                            description: ::core::option::Option::Some(
                                                "Enables the unminimize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["unminimize".into()]),
                                                ),
                                                deny: ::alloc::vec::Vec::new(),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-available-monitors".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-available-monitors".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the available_monitors command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["available_monitors".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-center".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-center".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the center command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["center".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-close".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-close".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the close command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["close".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-create".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-create".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the create command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["create".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-current-monitor".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-current-monitor".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the current_monitor command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["current_monitor".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-destroy".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-destroy".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the destroy command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["destroy".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-hide".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-hide".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the hide command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["hide".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-inner-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-inner-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the inner_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["inner_position".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-inner-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-inner-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the inner_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["inner_size".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-internal-on-mousedown".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-internal-on-mousedown".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the internal_on_mousedown command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["internal_on_mousedown".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-internal-on-mousemove".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-internal-on-mousemove".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the internal_on_mousemove command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["internal_on_mousemove".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-internal-toggle-maximize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-internal-toggle-maximize".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the internal_toggle_maximize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "internal_toggle_maximize".into(),
                                                    ]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-closable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-closable".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_closable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_closable".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-decorated".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-decorated".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_decorated command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_decorated".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-focused".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-focused".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_focused command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_focused".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-fullscreen".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-fullscreen".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_fullscreen command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_fullscreen".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-maximizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-maximizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_maximizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_maximizable".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-maximized".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-maximized".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_maximized command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_maximized".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-minimizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-minimizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_minimizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_minimizable".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-minimized".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-minimized".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_minimized command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_minimized".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-resizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-resizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_resizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_resizable".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-is-visible".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-is-visible".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the is_visible command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["is_visible".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-maximize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-maximize".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the maximize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["maximize".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-minimize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-minimize".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the minimize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["minimize".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-outer-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-outer-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the outer_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["outer_position".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-outer-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-outer-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the outer_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["outer_size".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-primary-monitor".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-primary-monitor".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the primary_monitor command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["primary_monitor".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-request-user-attention".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-request-user-attention".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the request_user_attention command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["request_user_attention".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-scale-factor".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-scale-factor".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the scale_factor command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["scale_factor".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-always-on-bottom".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-always-on-bottom".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_always_on_bottom command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_always_on_bottom".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-always-on-top".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-always-on-top".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_always_on_top command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_always_on_top".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-closable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-closable".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_closable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_closable".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-content-protected".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-content-protected".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_content_protected command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_content_protected".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-cursor-grab".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-cursor-grab".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_cursor_grab command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_cursor_grab".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-cursor-icon".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-cursor-icon".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_cursor_icon command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_cursor_icon".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-cursor-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-cursor-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_cursor_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_cursor_position".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-cursor-visible".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-cursor-visible".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_cursor_visible command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_cursor_visible".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-decorations".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-decorations".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_decorations command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_decorations".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-effects".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-effects".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_effects command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_effects".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-focus".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-focus".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_focus command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_focus".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-fullscreen".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-fullscreen".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_fullscreen command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_fullscreen".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-icon".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-icon".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_icon command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_icon".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-ignore-cursor-events".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-ignore-cursor-events".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_ignore_cursor_events command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_ignore_cursor_events".into(),
                                                    ]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-max-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-max-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_max_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_max_size".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-maximizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-maximizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_maximizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_maximizable".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-min-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-min-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_min_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_min_size".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-minimizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-minimizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_minimizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_minimizable".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-position".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-position".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_position command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_position".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-progress-bar".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-progress-bar".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_progress_bar command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_progress_bar".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-resizable".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-resizable".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_resizable command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_resizable".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-shadow".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-shadow".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_shadow command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_shadow".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-size".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-size".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_size command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_size".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-skip-taskbar".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-skip-taskbar".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_skip_taskbar command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_skip_taskbar".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-title".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-title".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_title command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["set_title".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-set-visible-on-all-workspaces".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-set-visible-on-all-workspaces".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the set_visible_on_all_workspaces command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([
                                                        "set_visible_on_all_workspaces".into(),
                                                    ]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-show".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-show".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the show command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["show".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-start-dragging".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-start-dragging".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the start_dragging command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["start_dragging".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-theme".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-theme".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the theme command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["theme".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-title".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-title".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the title command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["title".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-toggle-maximize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-toggle-maximize".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the toggle_maximize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["toggle_maximize".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-unmaximize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-unmaximize".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the unmaximize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["unmaximize".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map.insert(
                                        "deny-unminimize".into(),
                                        ::tauri::utils::acl::Permission {
                                            version: ::core::option::Option::None,
                                            identifier: "deny-unminimize".into(),
                                            description: ::core::option::Option::Some(
                                                "Denies the unminimize command without any pre-configured scope."
                                                    .into(),
                                            ),
                                            commands: ::tauri::utils::acl::Commands {
                                                allow: ::alloc::vec::Vec::new(),
                                                deny: <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new(["unminimize".into()]),
                                                ),
                                            },
                                            scope: ::tauri::utils::acl::Scopes {
                                                allow: ::core::option::Option::None,
                                                deny: ::core::option::Option::None,
                                            },
                                        },
                                    );
                                    map
                                },
                                permission_sets: ::std::collections::BTreeMap::new(),
                                global_scope_schema: ::core::option::Option::None,
                            },
                        );
                        map
                    },
                    allowed_commands: {
                        let mut map = ::std::collections::BTreeMap::new();
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:app|name".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:app|tauri_version".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:app|version".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:event|emit".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:event|emit_to".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:event|listen".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:event|unlisten".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:path|basename".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:path|dirname".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:path|extname".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:path|is_absolute".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:path|join".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:path|normalize".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:path|resolve".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:path|resolve_directory".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:resources|close".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|available_monitors".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|current_monitor".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|inner_position".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|inner_size".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|internal_on_mousedown".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|internal_on_mousemove".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|internal_toggle_maximize".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_closable".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_decorated".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_focused".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_fullscreen".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_maximizable".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_maximized".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_minimizable".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_minimized".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_resizable".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|is_visible".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|outer_position".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|outer_size".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|primary_monitor".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|scale_factor".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|theme".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map.insert(
                            ::tauri::utils::acl::resolved::CommandKey {
                                name: "plugin:window|title".into(),
                                context: ::tauri::utils::acl::ExecutionContext::Local,
                            },
                            ::tauri::utils::acl::resolved::ResolvedCommand {
                                referenced_by: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::tauri::utils::acl::resolved::ResolvedCommandReference {
                                            capability: "main-capability".into(),
                                            permission: "default".into(),
                                        },
                                    ]),
                                ),
                                windows: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new(["main".parse().unwrap()]),
                                ),
                                scope: ::core::option::Option::None,
                            },
                        );
                        map
                    },
                    denied_commands: ::std::collections::BTreeMap::new(),
                    command_scope: ::std::collections::BTreeMap::new(),
                    global_scope: {
                        let mut map = ::std::collections::BTreeMap::new();
                        map.insert(
                            "menu".into(),
                            ::tauri::utils::acl::resolved::ResolvedScope {
                                allow: ::alloc::vec::Vec::new(),
                                deny: ::alloc::vec::Vec::new(),
                            },
                        );
                        map.insert(
                            "tray".into(),
                            ::tauri::utils::acl::resolved::ResolvedScope {
                                allow: ::alloc::vec::Vec::new(),
                                deny: ::alloc::vec::Vec::new(),
                            },
                        );
                        map
                    },
                },
            );
            context
        })
        .expect("error while running tauri application");
}
