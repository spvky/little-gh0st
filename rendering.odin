package main

import l "core:math/linalg"
import rl "vendor:raylib"

render_scene :: proc() {
	rl.BeginTextureMode(screen_texture)
	rl.ClearBackground({0, 12, 240, 255})
	rl.BeginMode3D(camera.raw)
	// Draw the scene here
	rl.DrawCubeV({0, 0, 0}, {40, 1, 40}, rl.WHITE)
	rl.DrawCubeV({0, 0, -20}, {25, 10, 1}, rl.GREEN)
	rl.DrawCubeV({6.25, 5, 20}, {12.5, 10, 1}, rl.BLUE)
	rl.DrawCubeV({-6.25, 5, 20}, {12.5, 10, 1}, rl.PURPLE)
	rl.DrawCubeV({20, 0, 0}, {1, 10, 25}, rl.RED)
	rl.DrawCubeV({-20, 0, 0}, {1, 10, 25}, rl.YELLOW)
	draw_player()
	player_raycast()
	rl.EndMode3D()
	rl.EndTextureMode()
}

calculate_screen_offset :: proc() -> (x, y: f32) {
	x = (f32(WINDOW_WIDTH) / 2) - (f32(SCREEN_WIDTH) / 2)
	y = (f32(WINDOW_HEIGHT) / 2) - (f32(SCREEN_HEIGHT) / 2)
	return
}

draw_to_screen :: proc() {
	rl.BeginDrawing()
	rl.ClearBackground(rl.BLACK)

	x_offset, y_offset := calculate_screen_offset()

	source := rl.Rectangle {
		x      = x_offset,
		y      = y_offset,
		width  = f32(SCREEN_WIDTH),
		height = -f32(SCREEN_HEIGHT),
	}
	dest := rl.Rectangle {
		x      = 0,
		y      = 0,
		width  = f32(WINDOW_WIDTH),
		height = f32(WINDOW_HEIGHT),
	}
	origin := Vec2{0, 0}
	rotation: f32 = 0
	rl.DrawTexturePro(screen_texture.texture, source, dest, origin, rotation, rl.WHITE)
	// Debug drawing
	middle: Vec2 = {f32(WINDOW_WIDTH) / 2, f32(WINDOW_HEIGHT) / 2}
	rl.DrawCircleV(middle, 1, rl.WHITE)
	rl.EndDrawing()
}

player_raycast :: proc() {
	ray := Raycast {
		origin    = camera.position,
		direction = l.normalize(camera.forward),
		toi       = 100,
	}

	sphere := Sphere{player.translation.x, player.translation.y, player.translation.z, 1}
	if i, ok := sphere_ray_intersection(ray, sphere); ok {
		rl.DrawSphere(i, 0.25, rl.GREEN)
	}
}
