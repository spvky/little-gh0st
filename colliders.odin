package main

// Sphere represented as [(translation)xyz,(radius)w]
Sphere :: distinct [4]f32

// AABB represented as [(min),(max)]
AABB :: distinct [2]Vec3

// The 8 vertices of a rectanngle
Rectangle :: distinct [8][3]Vec3

// Plane Represented as [(point),(normal)]
Plane :: distinct [2]Vec3

Raycast :: struct {
	origin:    Vec3,
	direction: Vec3,
	toi:       f32,
}
