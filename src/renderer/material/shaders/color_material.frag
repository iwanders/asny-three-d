uniform vec4 surfaceColor;

#ifdef USE_TEXTURE
uniform sampler2D tex;
uniform mat3 textureTransformation;
#endif

in vec4 col;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = surfaceColor * col;
    
    #ifdef USE_TEXTURE
    vec4 tex_color = texture(tex, (textureTransformation * vec3(uvs, 1.0)).xy);
    outColor *= vec4(rgb_from_srgb(tex_color.rgb), tex_color.a);
    #endif

    outColor.rgb = srgb_from_rgb(outColor.rgb);
}