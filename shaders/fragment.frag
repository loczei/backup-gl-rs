#version 330 core

out vec4 FragColor;
in vec3 ourColor;
in vec2 TexCord;

uniform sampler2D texture1;
uniform sampler2D texture2;

uniform float mixValue;

void main() {
	FragColor = mix(texture(texture1, TexCord), texture(texture2, TexCord), mixValue);

}
