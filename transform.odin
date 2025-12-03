package main

import l "core:math/linalg"

Transform :: struct {
	translation: Vec3,
	scale:       Vec3,
	rotation:    quaternion128,
}

make_transform :: proc(
	translation: Vec3 = {0, 0, 0},
	scale: Vec3 = {1, 1, 1},
	rotation: quaternion128 = l.QUATERNIONF32_IDENTITY,
) -> Transform {
	return Transform{translation, scale, rotation}
}

// Rotates transform t to look at point v, assumes [0,1,0] as the up vector
look_at :: proc(t: ^Transform, v: Vec3, up: Vec3 = {0, 1, 0}) {
	forward := l.normalize0(v - t.translation)
	right := l.normalize0(l.cross(up, forward))
	new_up := l.normalize0(l.cross(forward, right))

	m := matrix[3, 3]f32{
		right.x, new_up.x, forward.x,
		right.y, new_up.y, forward.y,
		right.z, new_up.z, forward.z,
	}

	t.rotation = l.quaternion_from_matrix3(m)
}
