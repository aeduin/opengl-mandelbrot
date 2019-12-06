#version 400 core

out vec4 Color;

in vec2 v_texture_coordinate;

uniform dvec2 center;
uniform double scale;
uniform float max_mandel_number;
const double max_distance_squared = 4.0;

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

float mapper(float input) {
    input -= floor(input);

    if(input < .5) {
        return input * 2.0;
    }
    else {
        return 2.0 - input * 2.0 ;
    }
}

void main()
{
    double a = 0.0;
    double b = 0.0;
    float counter = 0;

    double mandelX = v_texture_coordinate.x * scale + center.x;
    double mandelY = v_texture_coordinate.y * scale + center.y;

    while(a * a + b * b < max_distance_squared){
        if(++counter >= max_mandel_number) {
            Color = vec4(0.0, 0.0, 0.0, 1.0);
            return;
        }
        
        double tempA = a * a - b * b + mandelX;
        b = 2.0 * a * b + mandelY;
        a = tempA;
    }

    float fraction = log(counter / 4000.0 * 32.0) / 24.0;

    float r = mapper(fraction * 7.0);
    float g = mapper(fraction * 11.0);
    float blue = mapper(fraction * 13.0);

    Color = vec4(r * sqrt(g) * sqrt(blue), g * g, blue, 1.0);
}