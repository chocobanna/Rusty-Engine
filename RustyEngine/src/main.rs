mod rusted_gl;

fn main() {
    // Force fallback adapter so pixels can create a pixel buffer even without a dedicated GPU adapter.
    std::env::set_var("WGPU_FORCE_FALLBACK_ADAPTER", "1");
    
    // Draw a window 800x600 with a line.
    rusted_gl::line::draw_line(800, 600);
}
