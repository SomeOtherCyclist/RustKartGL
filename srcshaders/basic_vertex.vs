
#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

void main() {
    mat4 model_view = view * model;
    v_normal = transpose(inverse(mat3(model_view))) * normal;
    gl_Position = perspective * model_view * vec4(position, 1.0);
}
