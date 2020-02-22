#version 330 core

in vec3 position;
in vec2 texture_coordinate;

out vec2 v_texture_coordinate;

void main() {
    gl_Position = vec4(position, 1.0);
    v_texture_coordinate = vec2(texture_coordinate.x, texture_coordinate.y);
}