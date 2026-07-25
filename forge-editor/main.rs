use forge_editor::{GameEngine, Vector2, Input, KeyCode};

fn main() {
    println!("=== Isometric RPG Game Engine ===");
    println!("Initializing game systems...\n");

    let mut game = GameEngine::new();
    
    println!("✅ Systems initialized:");
    println!("   - Sprite Manager");
    println!("   - Animation System");
    println!("   - Physics System (gravity, collisions)");
    println!("   - Player Controller");
    println!("   - Level Manager");
    println!("   - Tile System");
    println!("   - Isometric Camera");
    println!("   - Dialogue System");
    println!("   - Dialogue UI");
    println!();

    println!("🎮 Controls:");
    println!("   - W: Jump");
    println!("   - A/D: Move left/right");
    println!("   - Space: Attack");
    println!("   - Enter: Next dialogue");
    println!();

    println!("🚀 Starting game loop...\n");

    // Simulate running the game
    game.run();
    
    println!("\n✅ Game engine simulation complete!");
}
