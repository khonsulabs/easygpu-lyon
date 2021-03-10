#version 450

layout(set = 0, binding = 0) uniform Globals {
	mat4 ortho;
	mat4 transform;
} global;

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

layout(location = 0) out vec4 f_color;

void main() {
	f_color = color;
	gl_Position = global.ortho * global.transform * vec4(position, 1.0);
}
