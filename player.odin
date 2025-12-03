package main

import rl "vendor:raylib"

Player :: struct {
	using stats:     Player_Stats,
	using rigidbody: Rigidbody,
	movement_delta:  Vec3,
}

Player_Stats :: struct {
	move_speed:   f32,
	max_health:   f32,
	attack_speed: f32,
}

DEFAULT_STATS: Player_Stats : {move_speed = 5, max_health = 100, attack_speed = 0.33}


init_player :: proc(translation: Vec3 = {0, 0, 0}) {
	player = Player {
		stats      = DEFAULT_STATS,
		transform  = make_transform(translation),
		velocity   = {0, 0, 1},
		move_speed = 1,
	}

	append(&rigidbodies, &player.rigidbody)
}

draw_player :: proc() {
	rl.DrawSphere(player.translation, 1, rl.BLACK)
}

apply_player_movement_delta :: proc() {
	player.velocity = player.move_speed * player.movement_delta
}
