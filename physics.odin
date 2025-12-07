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
raycast_sphere :: proc(r: Raycast, s: Sphere) -> (intersection_point: Vec3, intersection: bool) {
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

raycast_aabb :: proc(r: Raycast, b: AABB) -> (intersection_point: Vec3, intersection: bool) {
	p := r.origin
	d := r.direction
	t_min := -math.F32_MAX
	t_max := r.toi

	for i in 0 ..< 3 {
		if math.abs(d[i]) < math.F32_EPSILON {
			if p[i] < b[0][i] || p[i] > b[1][i] {
				return
			}
		} else {
			ood := 1 / r.direction[i]
			t1 := (b[0][i] - p[i]) * ood
			t2 := (b[1][i] - p[i]) * ood
			if t1 < t2 {
				t1, t2 = t2, t1
			}
			if t1 > t_min {t_min = t1}
			if t2 > t_max {t_max = t2}
			if t_min > t_max {
				return
			}
		}
	}
	intersection_point = p + (d * t_min)
	intersection = true
	return
}

raycast :: proc {
	raycast_sphere,
	raycast_aabb,
}
