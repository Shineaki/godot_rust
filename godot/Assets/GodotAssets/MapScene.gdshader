shader_type canvas_item;
uniform float blue = 1.0;
// Noise texture
uniform sampler2D noise_texture: repeat_enable, filter_nearest;
// Fog density
uniform float density: hint_range(0.0, 1.0) = 0.25;
// Fog speed
uniform vec2 speed = vec2(0.02, 0.01);
void vertex() {
	// Called for every vertex the material is visible on.
	//VERTEX += vec2(cos(TIME)*100.0, sin(TIME)*100.0);
}

void fragment() {
	// Called for every pixel the material is visible on.
	vec2 uv = UV + speed * TIME;
	// Sample the noise texture
	float noise = texture(noise_texture, uv).r;
	// Convert the noise from the (0.0, 1.0) range to the (-1.0, 1.0) range
	// and clamp it between 0.0 and 1.0 again
	float fog = clamp(noise * 2.0 - 1.0, 0.0, 1.0);
	// Apply the fog effect
	//COLOR = vec3(1.0, 1.0, 1.0);
	COLOR.rgb = vec3(0.0, 0.0, 0.0);
	COLOR.a *= fog * density;
}

//void light() {
//	// Called for every pixel for every light affecting the CanvasItem.
//	// Uncomment to replace the default light processing function with this one.
//}
