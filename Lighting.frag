#version 330 core

in  vec3 vPosition;
in  vec3 vNormal;
in  vec3 vTangent;
in  vec2 vTexture;

out vec3 fColor;

struct LightProperties
{
	vec4  position;
	vec4  ambient;
	vec4  diffuse;
	vec4  specular;
	vec3  attenuation;
	vec3  direction;
	float cutoffAngle;
};

struct MaterialProperties
{
	vec4  ambient;
	vec4  diffuse;
	vec4  specular;
	float shininess;
};

uniform LightProperties    uLight[3];
uniform MaterialProperties uMaterial;

uniform mat4 view;

vec3 lighting(int index){
    vec3 surface = normalize(vNormal);
    vec3 light;

    // if the 4th dimension is zero then this is a directional light 
    float attenuation = 1.0f;
    if(uLight[index].position.w == 0.0f){
        light = normalize((view * uLight[index].position).xyz);
        // attenuation is a constant 1.f for directional lights
    }
    // otherwise it is a point light
    else{
        light       = normalize((view * uLight[index].position).xyz - vPosition);
        float dist  = length(light);
        light       = normalize(light);
        attenuation = 1.f / (uLight[index].attenuation.x +
                             uLight[index].attenuation.y * dist +
                             uLight[index].attenuation.z * dist * dist);
        
    }

    vec3 eye = normalize(-vPosition);
    vec3 reflect = reflect(-light, surface);

    // calculate the ambient, diffuse and specular components
    vec4 ambient  = uLight[index].ambient * uMaterial.ambient;
    vec4 diffuse  = uLight[index].diffuse * uMaterial.diffuse * max(dot(light, surface), 0.0);
    vec4 specular = vec4(0.0f, 0.0f, 0.0f, 1.0f);

    if(dot(light, surface) > 0.0f)
    {
        specular = uLight[index].specular
                 * uMaterial    .specular
                 * pow(max(dot(eye, reflect), 0.0), uMaterial.shininess);
    }

    vec3 fColor = vec3(0.f, 0.f, 0.f);
    if(uLight[index].position.w == 0.0f){
        vec3  direction = (normalize(view * vec4(uLight[index].direction, 0.f))).xyz;
        float angle     = degrees(acos(dot(-light, direction)));
        if(angle <= uLight[index].cutoffAngle){
            fColor = ((attenuation * (diffuse + specular)).rgb + ambient.rgb) *
                     (1.f - angle / uLight[index].cutoffAngle);
        }
    }
    else{
        fColor = (attenuation * (diffuse + specular)).rgb + ambient.rgb;
    }

    return fColor;
}

void main()
{
    if(true){
        fColor = vec3(1.0, 0.0, 0.0); // uMaterial.diffuse.xyz;
    }
    else{
        fColor =
            lighting(0) +
            lighting(1) +
            lighting(2) ;
    }
}
