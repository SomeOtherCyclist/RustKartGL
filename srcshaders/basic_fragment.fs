#version 140

in vec3 v_normal;

out vec4 colour;

uniform vec3 u_light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_colour = vec3(0.2, 0.0, 0.0);
    vec3 regular_colour = vec3(1.0, 0.0, 0.0);
    colour = vec4(mix(dark_colour, regular_colour, brightness), 1.0);
}
