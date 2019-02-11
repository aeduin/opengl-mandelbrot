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
    //Color = vec4(texture(v_texture, v_texture_coordinate).rgb, 1.0);

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

    float fraction = log(counter / 4000.0 * 32.0) / 8.0;

    Color = vec4(mapper(fraction * 3.0), mapper(fraction * 4.0), mapper(fraction * 5.0), 1.0);
}