use legion::Schedule;

pub mod render;
pub mod tick;
pub mod chunk;


pub fn populate() -> (Schedule, Schedule){
    let step_schedule = Schedule::builder()
        .add_thread_local(tick::input_update_system())
        .add_system(tick::step_animation_system(0.0))
        .add_system(render::z_y_axis_player_system())
        .add_thread_local(tick::step_physics_system())
        .add_thread_local(tick::integrate_physics_system())
        .add_thread_local(tick::move_player_system())
        .add_system(render::track_player_system())
        .add_thread_local(tick::animate_player_system())
        .add_system(chunk::update_player_chunk_system())
        .add_system(chunk::update_monster_chunk_system())
        .add_system(chunk::create_new_chunks_system())
        .add_thread_local(chunk::load_chunks_system())
        .add_thread_local(chunk::load_chunk_bodies_system())
        .add_thread_local(chunk::unload_chunks_system())
        .add_system(chunk::free_chunks_system())
        .flush()
        .build();

    let draw_schedule = Schedule::builder()
        .add_thread_local(render::camera_system())
        .add_thread_local(render::clear_screen_system())
        .add_thread_local(render::render_system())
        .flush()
        .add_thread_local(render::camera_ui_system())
        .add_thread_local(tick::debug_input_system(false))
        .add_thread_local(render::draw_fps_system())
        .build();

    (step_schedule, draw_schedule)
}
