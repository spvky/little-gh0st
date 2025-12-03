package main

import "core:fmt"
import m "core:math"
import l "core:math/linalg"
import rl "vendor:raylib"

camera: Camera

Camera :: struct {
	using raw:        rl.Camera3D,
	look_sensitivity: f32,
	target_pitch:     f32,
	pitch:            f32,
	target_yaw:       f32,
	yaw:              f32,
	smoothing:        f32,
	forward:          Vec3,
	right:            Vec3,
}

init_camera :: proc() {
	camera = Camera {
		fovy         = 90,
		up           = {0, 1, 0},
		smoothing    = 5,
		position     = {0, 10, 0},
		target_pitch = m.PI * -0.125,
		// target_yaw   = m.PI * 0.5,
	}
}

update_camera :: proc() {
	frametime := rl.GetFrameTime()
	// camera.target_yaw += frametime * m.PI * 0.25

	camera.pitch = m.lerp(camera.pitch, camera.target_pitch, frametime * camera.smoothing)
	camera.yaw = m.lerp(camera.yaw, camera.target_yaw, frametime * camera.smoothing)

	forward := l.normalize(
		Vec3 {
			m.cos(camera.yaw) * m.cos(camera.pitch),
			m.sin(camera.pitch),
			m.sin(camera.yaw) * m.cos(camera.pitch),
		},
	)
	right := l.normalize(l.cross(forward, Vec3{0, 1, 0}))


	camera.forward = forward
	camera.right = right
	camera.target = camera.position + forward
}

interpolate_vector :: proc(vector: Vec3) -> Vec3 {
	true_vec := (camera.forward * vector.z) + (camera.right * vector.x)
	true_vec.y = 0
	return l.normalize0(true_vec)
}
