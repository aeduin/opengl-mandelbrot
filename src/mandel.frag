#version 430

out vec4 Color;
in vec2 v_texture_coordinate;

uniform int color_function_id;
uniform float time;
layout(r32i) uniform iimage2D image;

uniform int buffer_width;
uniform int buffer_height;


float mapper(float input) {
    input -= floor(input);

    if(input < .5) {
        return input * 2.0;
    }
    else {
        return 2.0 - input * 2.0 ;
    }
}

void main() {
    int x = int(v_texture_coordinate.x * float(buffer_width));
    int y = int(v_texture_coordinate.y * float(buffer_height));

    float counter = float(imageLoad(image, ivec2(x, y)));

    if(color_function_id == 0) {
        float fraction = log(counter / 4000.0 * 32.0) / 24.0;

        float r = mapper(fraction * 7.0);
        float g = mapper(fraction * 11.0);
        float b = mapper(fraction * 13.0);

        Color = vec4(r * sqrt(g) * sqrt(b), g * g, b, 1.0);
    }
    else if(color_function_id == 1) {
        float fraction = log(counter / 4000.0 * 32.0) / 24.0;

        float r = mapper(fraction * 7.0);
        float g = mapper(fraction * 11.0);
        float b = mapper(fraction * 13.0);

        Color = vec4(r * g * b, g, b, 1.0);
    }
    else if(color_function_id == 2) {
        float fraction = log(counter / 4000.0 * 32.0) / 24.0;

        float r = mapper(mod(fraction * 7.0, .5));
        float g = mapper(mod(fraction * 11.0, .5));
        float b = mapper(mod(fraction * 13.0, .5));

        Color = vec4(r * g * b, g, b, 1.0);
    }
    else if(color_function_id == 3) {
        float fraction = log(counter / 4000.0 * 32.0) / 24.0;

        float r = mapper(fraction * 3.0);
        float g = mapper(fraction * 11.0);
        float b = mapper(fraction * 31.0);

        Color = vec4(r * sqrt(g) * sqrt(b), g * b, b, 1.0);
    }
    else if(color_function_id == 4) {
        float fraction = log(counter / 4000.0 * 32.0) / 24.0 + time * 0.018;

        float r = mapper(fraction * 3.0);
        float g = mapper(fraction * 11.0);
        float b = mapper(fraction * 31.0);

        Color = vec4(r * sqrt(g) * sqrt(b), g * b, b, 1.0);
    }
    else if(color_function_id == 5) {
        float fraction = log(counter / 4000.0 * 32.0) / 24.0 + time * 0.02;

        float r = mapper(fraction * 7.0);
        float g = mapper(fraction * 11.0);
        float b = mapper(fraction * 13.0);

        Color = vec4(r * sqrt(g) * sqrt(b), g * g, b, 1.0);
    }
    else if(color_function_id == 6) {
        float fraction = log(counter / 4000.0 * 32.0) / 24.0 + time * 0.02;

        float r = pow(mapper(fraction * 7.0), 1.0 + time *0.1);
        float g = pow(mapper(fraction * 11.0), 1.0 + time *0.15);
        float b = pow(mapper(fraction * 13.0), 1.0 + time *0.2);

        Color = vec4(r * sqrt(g) * sqrt(b), g * g, b, 1.0);
    }
}