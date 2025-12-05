package main


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

sphere_ray_intersection :: proc(
	r: Raycast,
	s: Sphere,
) -> (
	intersection_point: Vec3,
	intersection: bool,
) {
	return
}
