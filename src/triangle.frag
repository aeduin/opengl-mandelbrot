#version 400 core

out vec4 Color;

in vec2 v_texture_coordinate;

uniform dvec2 center;
uniform double scale;
uniform float max_mandel_number;

void main()
{
    //Color = vec4(texture(v_texture, v_texture_coordinate).rgb, 1.0);

    double a = 0.0;
    double b = 0.0;
    float counter = 0;

    double mandelX = v_texture_coordinate.x * scale + center.x;
    double mandelY = v_texture_coordinate.y * scale + center.y;

    while(a * a + b * b < 4.0) {
        if(++counter >= max_mandel_number) {
            Color = vec4(0.0, 0.0, 0.0, 1.0);
            return;
        }
        
        double tempA = a * a - b * b + mandelX;
        b = 2.0 * a * b + mandelY;
        a = tempA;
    }
/*
    float red = min(counter / max_mandel_number, 1.0f);
    float blue = min(counter / max_mandel_number * 2.0f, 1.0f);
    float green = min(counter / max_mandel_number * 1.5f, 1.0f);
*/

    float red = 1.0 - 1000.0 / (counter + 1000.0);
    float green = 1.0 - 600.0 / (counter + 600.0);
    float blue = 1.0 - 300.0 / (counter + 300.0);

    Color = vec4(red, green, blue, 1.0);
}