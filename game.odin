package main

import "core:c"
import "core:math"
import rl "vendor:raylib"

world: World
screen_texture: rl.RenderTexture
run: bool
time: Time
player: Player

WINDOW_WIDTH: i32
WINDOW_HEIGHT: i32
SCREEN_WIDTH :: 800
SCREEN_HEIGHT :: 450


init :: proc() {
	WINDOW_WIDTH = 1920
	WINDOW_HEIGHT = 1080
	run = true
	rl.InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, "Game")
	init_camera()
	init_physics()
	init_player({10, 1, 0})
	screen_texture = rl.LoadRenderTexture(WINDOW_WIDTH, WINDOW_HEIGHT)
	world = make_world()
}

update :: proc() {
	update_camera()
	playing()
	render_scene()
	draw_to_screen()
	free_all(context.temp_allocator)
}

playing :: proc() {
	if !time.started {
		time.t = f32(rl.GetTime())
		time.started = true
	}
	// Non physics updates
	poll_player_input()
	apply_player_movement_delta()
	//////////////////////
	t1 := f32(rl.GetTime())
	elapsed := math.min(t1 - time.t, 0.25)
	time.t = t1
	time.simulation_time += elapsed
	for time.simulation_time >= TICK_RATE {
		physics_step()
		time.simulation_time -= TICK_RATE
	}
	alpha := time.simulation_time / TICK_RATE
}

shutdown :: proc() {
	rl.UnloadRenderTexture(screen_texture)
	rl.CloseWindow()
}

should_run :: proc() -> bool {
	when ODIN_OS != .JS {
		run = !rl.WindowShouldClose()
	}
	return run
}

parent_window_size_changed :: proc(w, h: int) {
	WINDOW_WIDTH = i32(w)
	WINDOW_HEIGHT = i32(h)
	rl.SetWindowSize(c.int(WINDOW_WIDTH), c.int(WINDOW_WIDTH))
}
