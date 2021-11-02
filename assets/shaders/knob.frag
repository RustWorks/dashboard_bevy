#version 450
#define PI 3.1415926538

layout(location = 0) in vec4 gl_FragCoord;
layout(location = 1) in vec3 v_Position;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform KnobShader_color{
    vec4 color;
};
layout(set = 2, binding = 1) uniform KnobShader_clearcolor{
    vec4 clear_color;
};
layout(set = 2, binding = 2) uniform KnobShader_zoom{
    float zoom;
};
layout(set = 2, binding = 3) uniform KnobShader_hovered{
    float hovered;
};
layout(set = 2, binding = 4) uniform KnobShader_size{
    float size;
};

layout(set = 2, binding = 5) uniform KnobShader_angle{
    float angle;
};

/////////////// unused ///////////////
float sdBox( in vec2 p, in vec2 b )
{
    vec2 d = abs(p)-b;
    return length(max(d,0.0)) + min(max(d.x,d.y),0.0);
}


float sdSegment( in vec2 p, in vec2 a, in vec2 b )
{
    vec2 pa = p-a, ba = b-a;
    float h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
    return length( pa - ba*h );
}


float sdSquareEdge(vec2 p, float r, float w)
{
    float d = sdBox( p, vec2(r,r) );
    float s1 = smoothstep(-0.005, 0.01, d);

    float width = 0.01;
    float s2 = smoothstep(-0.005-w, 0.002-w, d);
    return 1.0 - abs(s1-s2);
}
/////////////// unused ///////////////

float sdArc( in vec2 p, in vec2 sca, in vec2 scb, in float ra, float rb )
{
    p *= mat2(sca.x,sca.y,-sca.y,sca.x);
    p.x = abs(p.x);
    float k = (scb.y*p.x>scb.x*p.y) ? dot(p,scb) : length(p);
    return sqrt( dot(p,p) + ra*ra - 2.0*ra*k ) - rb;
}


float sdCircle( vec2 p, float r)
{
    float d = length(p) - r;
    return d;
}


void main( )
{
    vec2 pos = vec2(0.5, 0.5);
    vec2 uv_original = (Vertex_Uv.xy-pos);

    // float d = sdArc(uv_original, vec2(0.5,0.5), vec2(0.5,0.5), 0.2, 0.1);
    // vec4 arc_color = mix( clear_color, color ,   d );
    // o_Target = arc_color;

        // normalized pixel coordinates
    vec2 p = uv_original * 2; //(gl_FragCoord*2.0-iResolution.xy)/iResolution.y;

    // float angle2 = 45.0 * PI / 180.0;

    // animation
    float time = 10.0;
    float ta =  3.14*(0.5+0.5*cos(time*0.52+2.0));
    float tb = angle; //3.14*(0.5+0.5*cos(time*0.31+2.0));
    float rb = 0.15*(0.5+0.5*cos(time*0.41+3.0));
    
    // distance
    float d = sdArc(p,vec2(sin(ta),cos(ta)),vec2(sin(tb),cos(tb)), 0.7, rb);
    
    // coloring
    // vec4 col = vec4(1.0) - sign(d)*vec4(0.1,0.4,0.7, 1.0);
	// col *= 1.0 - exp(-2.0*abs(d));
	// col *= 0.8 + 0.2*cos(128.0*abs(d));
	vec4 col = mix( clear_color, color, 1.0-smoothstep(0.0,0.15,d) );



	o_Target = col;
}
