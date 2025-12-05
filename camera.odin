package main

import "core:fmt"
import m "core:math"
import l "core:math/linalg"
import rl "vendor:raylib"

camera: Camera

Camera :: struct {
	using raw:        rl.Camera3D,
	mode:             Camera_Mode,
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
		mode         = Free{},
		fovy         = 90,
		up           = {0, 1, 0},
		smoothing    = 5,
		position     = {0, 10, 0},
		target_pitch = m.PI * -0.125,
		// target_yaw   = m.PI * 0.5,
	}
}

Camera_Mode :: union {
	Orbit,
	Free,
}

// Camera mode that makes the camera orbit a target
Orbit :: struct {
	cam_target:     Vec3,
	offset:         Vec3,
	orbit_rotation: f32,
	look_at:        bool,
}

// Camera mode that is locked in place and manually rotated
Free :: struct {}

update_camera :: proc() {
	frametime := rl.GetFrameTime()


	camera.pitch = m.lerp(camera.pitch, camera.target_pitch, frametime * camera.smoothing)
	camera.yaw = m.lerp(camera.yaw, camera.target_yaw, frametime * camera.smoothing)

	switch v in camera.mode {
	case Free:
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

	case Orbit:
	}
}

interpolate_vector :: proc(vector: Vec3) -> Vec3 {
	true_vec := (camera.forward * vector.z) + (camera.right * vector.x)
	true_vec.y = 0
	return l.normalize0(true_vec)
}
