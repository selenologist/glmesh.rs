#version 330 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 tangent;
layout(location = 3) in vec2 texture;

out vec3 vPosition;
out vec3 vNormal;
out vec3 vTangent;
out vec2 vTexture;

uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;

void main()
{
	// set vertex position
    gl_Position = proj * view * model * vec4(position, 1.0);
    vPosition   =       (view * model * vec4(position, 1.0)).xyz;
    vNormal     =       (view * model * vec4(normal,   0.0)).xyz;
    vTangent    =       (view * model * vec4(tangent,  0.0)).xyz;
    vTexture    = texture;
}

