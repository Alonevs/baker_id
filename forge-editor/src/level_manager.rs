use crate::math::Vector2;

pub struct Level {
    name: String,
    tiles: Vec<Tile>,
    platforms: Vec<Platform>,
    spawn_point: SpawnPoint,
    enemy_spawns: Vec<SpawnPoint>,
}

impl Level {
    pub fn new(name: String, data: &LevelData) -> Self {
        let tiles = data.tiles.iter().map(|t| Tile::new(t.x, t.y, &t.tile_type)).collect();
        let platforms = data.platforms.iter().map(|p| Platform::new(p)).collect();
        
        Level {
            name,
            tiles,
            platforms,
            spawn_point: SpawnPoint::new(data.spawn_point.x, data.spawn_point.y),
            enemy_spawns: data.enemy_spawns.iter().map(|s| SpawnPoint::new(s.x, s.y)).collect(),
        }
    }

    pub fn load(path: &str) -> Result<Self, String> {
        // Simulación de carga desde archivo
        let data = LevelData {
            tiles: vec![
                TileData { x: 0, y: 0, tile_type: "grass".to_string() },
                TileData { x: 1, y: 0, tile_type: "grass".to_string() },
            ],
            platforms: vec![
                PlatformData { x: 5, y: 3, width: 3, height: 1 },
            ],
            spawn_point: SpawnPointData { x: 1, y: 1 },
            enemy_spawns: vec![
                SpawnPointData { x: 8, y: 4 },
            ],
        };
        Ok(Level::new("default".to_string(), &data))
    }

    pub fn get_tile_at(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.iter().find(|t| t.x == x && t.y == y)
    }

    pub fn get_platform_at(&self, x: i32, y: i32) -> Option<&Platform> {
        self.platforms.iter().find(|p| 
            x >= p.x && x < p.x + p.width &&
            y >= p.y && y < p.y + p.height
        )
    }

    pub fn spawn_point(&self) -> &SpawnPoint {
        &self.spawn_point
    }

    pub fn enemy_spawns(&self) -> &[SpawnPoint] {
        &self.enemy_spawns
    }
}

pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: String,
}

impl Tile {
    fn new(x: i32, y: i32, tile_type: &str) -> Self {
        Tile { x, y, tile_type: tile_type.to_string() }
    }
}

pub struct Platform {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Platform {
    fn new(data: &PlatformData) -> Self {
        Platform {
            x: data.x,
            y: data.y,
            width: data.width,
            height: data.height,
        }
    }
}

pub struct SpawnPoint {
    pub x: i32,
    pub y: i32,
}

impl SpawnPoint {
    fn new(x: i32, y: i32) -> Self {
        SpawnPoint { x, y }
    }
}

pub struct LevelData {
    pub tiles: Vec<TileData>,
    pub platforms: Vec<PlatformData>,
    pub spawn_point: SpawnPointData,
    pub enemy_spawns: Vec<SpawnPointData>,
}

pub struct TileData {
    pub x: i32,
    pub y: i32,
    pub tile_type: String,
}

pub struct PlatformData {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct SpawnPointData {
    pub x: i32,
    pub y: i32,
}
