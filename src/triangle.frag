#version 400 core
#extension GL_ARB_gpu_shader_int64 : require

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

uint64_t mult(uint64_t x, uint64_t y) {
    uint64_t x1 = x;
    uint64_t x2 = x >> 32;
    uint64_t y1 = y;
    uint64_t y2 = y >> 32;

    return 1;
}

void main()
{
    double a = 0.0;
    double b = 0.0;
    float counter = 0;

    double mandelX = v_texture_coordinate.x * scale + center.x;
    double mandelY = v_texture_coordinate.y * scale + center.y;

    // if(mandelX * mandelX + mandelY * mandelY > max_distance_squared) {
    //     counter = 1;
    // }
    // else {
    //     double max_64 = double(uint64_t(1)<<63);
        

    //     uint64_t mx = uint64_t(mandelX * max_64 / 2.0);
    //     uint64_t my = uint64_t(mandelY * max_64 / 2.0);
    // }

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