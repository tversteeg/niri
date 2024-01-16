precision mediump float;
uniform vec2 size;
varying vec2 v_coords;

uniform float radius;

float rounded_box(vec2 center, vec2 size, float radius) {
    return length(max(abs(center) - size + radius, 0.0)) - radius;
}

void main() {
    vec2 center = size / 2.0 - vec2(0.5);
    vec2 location = v_coords * size;
    vec4 mix_color;

    float distance = abs(rounded_box(location - center, size / 2.0, radius));
    float smoothedAlpha = 1.0 - smoothstep(0.0, 1.0, abs(distance));

    gl_FragColor = vec4(0.0, 0.0, 0.0, smoothedAlpha);
}
