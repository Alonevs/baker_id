#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlatformConfig {
    pub executable_name: &'static str,
    pub entry_point: &'static str,
    pub requires_natives: bool,
    pub default_resolution: [u32; 2],
}

impl PlatformConfig {
    pub fn new(
        executable_name: &'static str,
        entry_point: &'static str,
        requires_natives: bool,
        resolution: [u32; 2],
    ) -> Self {
        Self {
            executable_name,
            entry_point,
            requires_natives,
            default_resolution: resolution,
        }
    }

    pub fn get_resolution(&self) -> [u32; 2] {
        self.default_resolution
    }
}

pub static WINDOWS_CONFIG: PlatformConfig = PlatformConfig {
    executable_name: "game.exe",
    entry_point: "main.rs",
    requires_natives: false,
    default_resolution: [1920, 1080],
};

pub static LINUX_CONFIG: PlatformConfig = PlatformConfig {
    executable_name: "game",
    entry_point: "main.rs",
    requires_natives: false,
    default_resolution: [1920, 1080],
};

pub static MACOS_CONFIG: PlatformConfig = PlatformConfig {
    executable_name: "Game.app/Contents/MacOS/game",
    entry_point: "main.rs",
    requires_natives: true,
    default_resolution: [1920, 1080],
};

pub static WEB_CONFIG: PlatformConfig = PlatformConfig {
    executable_name: "index.html",
    entry_point: "main.ts",
    requires_natives: false,
    default_resolution: [1280, 720],
};

pub static ANDROID_CONFIG: PlatformConfig = PlatformConfig {
    executable_name: "game.apk",
    entry_point: "MainActivity.java",
    requires_natives: true,
    default_resolution: [1920, 1080],
};

pub static IOS_CONFIG: PlatformConfig = PlatformConfig {
    executable_name: "Game.app",
    entry_point: "ViewController.swift",
    requires_natives: true,
    default_resolution: [1920, 1080],
};
