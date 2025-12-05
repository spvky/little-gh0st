package main

import "core:log"
import "core:math"
import l "core:math/linalg"

rigidbodies: [dynamic]^Rigidbody

TICK_RATE :: 1.0 / 200.0

Rigidbody :: struct {
	using transform: Transform,
	velocity:        Vec3,
}

init_physics :: proc() {
	rigidbodies = make([dynamic]^Rigidbody, 0, 32)
}

physics_step :: proc() {
	apply_rigidbody_velocities()
}

apply_rigidbody_velocities :: proc() {
	for &rb in rigidbodies {
		rb.translation += rb.velocity * TICK_RATE
	}
}

// Sphere ray collision using a quadratic equation, Real Time Collision Detection pg.177
sphere_ray_intersection :: proc(
	r: Raycast,
	s: Sphere,
) -> (
	intersection_point: Vec3,
	intersection: bool,
) {
	m := r.origin - s.xyz
	b := l.dot(m, r.direction)
	c := l.dot(m, m) - (s.w * s.w)
	d := (b * b) - c
	if d >= 0 {
		intersection = true
		t := -b - math.sqrt(d)
		intersection_point = r.origin + (r.direction * t)
	}
	return
}
