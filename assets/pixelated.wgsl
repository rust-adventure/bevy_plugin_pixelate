#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_view_bindings::globals,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    prepass_utils::{prepass_depth, prepass_normal}
}
#endif

struct MyExtendedMaterial {
    quantize_steps: u32,
}

@group(1) @binding(100)
var<uniform> my_extended_material: MyExtendedMaterial;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
    #ifdef MULTISAMPLED
    @builtin(sample_index) sample_index: u32,
#endif
) -> FragmentOutput {
    #ifndef MULTISAMPLED
        let sample_index = 0u;
    #endif
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // we can optionally modify the input before lighting and alpha_discard is applied
    // pbr_input.material.base_color.b = pbr_input.material.base_color.r;

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);
    
    let color_oklab = oklab_from_linear(out.color.xyz);
    let quantized_lightness = floor(color_oklab.x * f32(my_extended_material.quantize_steps)) / f32(my_extended_material.quantize_steps);
    out.color = vec4<f32>(linear_from_oklab(vec3<f32>(quantized_lightness, color_oklab.y, color_oklab.z)).xyz, out.color.a);
   
    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // we can optionally modify the final result here
    // out.color = out.color;
#endif

    let s = detect_silho_coord(
        vec2i(
            i32(in.position.x),
            i32(in.position.y)
        ),
        sample_index
    );

    let show_depth = 0u;
    let show_normals = 0u;
    let show_outline = 0u;
    let show_normal_borders = 0u;

    if show_depth == 1u {
        let depth = prepass_depth(in.position, sample_index);
        out.color = vec4(depth, depth, depth, 1.0);
    } else if show_normals == 1u {
        let normal = prepass_normal(in.position, sample_index);
        out.color = vec4(normal, 1.0);
    } else if show_outline == 1u {
        out.color = vec4(f32(s), f32(s), f32(s), 1.);
    } else if show_normal_borders == 1u {
        let edge_mask = normal_edges(in.position, sample_index);
        out.color = vec4f(vec3(edge_mask), 1.0);
    } else {
        out.color = mix(
            out.color,
            // out.color * 100.,
            // vec4(0.,0.,0.,1.),
            out.color / 4.,
            s * 1.2
        );
        let edge_color = vec3(0.,0.,1.);
        let edge_strength = 2.;
        out.color = mix(
            out.color,
            out.color * 4.,
            normal_edges(in.position, sample_index)
        );
    }

    return out;
}

fn lightness_step(x: f32) -> f32 {
    if x < 10. {
        return 0.;
    } else if x < 25. {
        return 25.;
    } else if x < 50. {
        return 50.;
    } else if x < 75. {
        return 75.;
    } else {
        return 100.;
    }
}

fn get_tolerance(d: f32, k: f32) -> f32
{
    let FAR = -10.;
    let NEAR = 1.;
    // -------------------------------------------
    // Find a tolerance for depth that is constant
    // in view space (k in view space).
    //
    // tol = k*ddx(ZtoDepth(z))
    // -------------------------------------------
    
    let A: f32 = -   (FAR+NEAR)/(FAR - NEAR);
    let B: f32 = -2.0 * FAR * NEAR /(FAR -NEAR);
    
    let new_d = d * 2.0 - 1.0;
    
    return -k*(new_d+A)*(new_d+A)/B;
}

fn detect_silho(frag_coord: vec2i, dir: vec2i, sample_index: u32) -> f32
{
    // -------------------------------------------
    //   x0 ___ x1----o 
    //          :\    : 
    //       r0 : \   : r1
    //          :  \  : 
    //          o---x2 ___ x3
    //
    // r0 and r1 are the differences between actual
    // and expected (as if x0..3 where on the same
    // plane) depth values.
    // -------------------------------------------

    let coord0 = (frag_coord + dir * -2);
    let pos0 = vec4f(f32(coord0.x), f32(coord0.y), 0., 1.); 
    let x0: f32 = abs(prepass_depth(
        pos0,
        sample_index
    ));

    let coord1 = (frag_coord + dir * -1);
    let pos1 = vec4f(f32(coord1.x), f32(coord1.y), 0., 1.); 
    let x1: f32 = abs(prepass_depth(pos1, sample_index));

    let coord2 = (frag_coord + dir * 0);
    let pos2 = vec4f(f32(coord2.x), f32(coord2.y), 0., 1.); 
    let x2: f32 = abs(prepass_depth(pos2, sample_index));

    let coord3 = (frag_coord + dir * 1);
    let pos3 = vec4f(f32(coord3.x), f32(coord3.y), 0., 1.); 
    let x3: f32 = abs(prepass_depth(pos3, sample_index));
    
    let d0: f32 = (x1-x0);
    let d1: f32 = (x2-x3);
    
    let r0: f32 = x1 + d0 - x2;
    let r1: f32 = x2 + d1 - x1;
    
    let tol: f32 = get_tolerance(x2, 0.04);
    
    return smoothstep(0.0, tol*tol, max( - r0*r1, 0.0));
    // return 0.;
}

fn detect_silho_coord(frag_coord: vec2i, sample_index: u32) -> f32
{
    return max(
        detect_silho(frag_coord, vec2i(1,0), sample_index), // Horizontal
        detect_silho(frag_coord, vec2i(0,1), sample_index)  // Vertical
    );
}

fn normal_edges(position: vec4f, sample_index: u32) -> f32 {
    let normal = prepass_normal(position, sample_index);

    let neighbour_left = prepass_normal(position, sample_index);
	let neighbour_right = prepass_normal( position + vec4(0.5, 0., 0., 0.), sample_index);
	
	let neighbour_top = prepass_normal( position, sample_index);
	let neighbour_bottom = prepass_normal( position + vec4(0., 0.5, 0.,0.), sample_index);
	
	// ALBEDO = albedo.rgb * texture(texture_albedo, UV).rgb;
    // let ALBEDO = vec3(1.,1.,1.);
	
    let edge_color = vec3(0.,0.,1.);
    let edge_strength = 2.;
    // if (abs(dot(neighbour_left, neighbour_right)) > 0.0) {
	// 	// return mix(albedo, edge_color.rgb, edge_strength);
    //     return 1.0;
	// } else if (abs(dot(neighbour_top, neighbour_bottom)) > 0.0) {
    // 	// return mix(albedo, edge_color.rgb, edge_strength);
    //     return 1.0;
	// } else {
    //     return 0.;
    // }
	// compare normals: if they differ, we draw an edge
	// by mixing in the edge_color, by edge_strength amount
	// feel free to try other ways to mix, such as multiply for more textured objects.
	// if (abs(vec3_avg(neighbour_left) - vec3_avg(neighbour_right)) > 0.0) {
	// 	// return mix(albedo, edge_color.rgb, edge_strength);
    //     return 1.0;
	// } else if (abs(vec3_avg(neighbour_top) - vec3_avg(neighbour_bottom)) > 0.0) {
    // 	// return mix(albedo, edge_color.rgb, edge_strength);
    //     return 1.0;
	// } else {
    //     return 0.;
    // }
    return 1. - min(
        dot(neighbour_left, neighbour_right),
        dot(neighbour_top, neighbour_bottom)
    );
}

fn vec3_avg(color: vec3f) -> f32 {
	return (color.r + color.g + color.b) / 3.0;
}
//By Björn Ottosson
//https://bottosson.github.io/posts/oklab
//Shader functions adapted by "mattz"
//https://www.shadertoy.com/view/WtccD7

fn oklab_from_linear(linear: vec3f) -> vec3f
{
    let im1: mat3x3<f32> = mat3x3<f32>(0.4121656120, 0.2118591070, 0.0883097947,
                          0.5362752080, 0.6807189584, 0.2818474174,
                          0.0514575653, 0.1074065790, 0.6302613616);
                       
    let im2: mat3x3<f32> = mat3x3<f32>(0.2104542553, 1.9779984951, 0.0259040371,
                          0.7936177850, -2.4285922050, 0.7827717662,
                          -0.0040720468, 0.4505937099, -0.8086757660);
                       
    let lms: vec3f = im1 * linear;
            
    return im2 * (sign(lms) * pow(abs(lms), vec3(1.0/3.0)));
}

fn linear_from_oklab(oklab: vec3f) -> vec3f
{
    let m1: mat3x3<f32> = mat3x3<f32>(1.000000000, 1.000000000, 1.000000000,
                         0.396337777, -0.105561346, -0.089484178,
                         0.215803757, -0.063854173, -1.291485548);
                       
    let m2: mat3x3<f32> = mat3x3<f32>(4.076724529, -1.268143773, -0.004111989,
                         -3.307216883, 2.609332323, -0.703476310,
                         0.230759054, -0.341134429, 1.706862569);
    let lms: vec3f = m1 * oklab;
    
    return m2 * (lms * lms * lms);
}
//By Inigo Quilez, under MIT license
//https://www.shadertoy.com/view/ttcyRS
fn oklab_mix(lin1: vec3f, lin2: vec3f, a: f32) -> vec3f
{
    // https://bottosson.github.io/posts/oklab
    let kCONEtoLMS: mat3x3<f32> = mat3x3<f32>(                
         0.4121656120,  0.2118591070,  0.0883097947,
         0.5362752080,  0.6807189584,  0.2818474174,
         0.0514575653,  0.1074065790,  0.6302613616);
    let kLMStoCONE: mat3x3<f32> = mat3x3<f32>(
         4.0767245293, -1.2681437731, -0.0041119885,
        -3.3072168827,  2.6093323231, -0.7034763098,
         0.2307590544, -0.3411344290,  1.7068625689);
                    
    // rgb to cone (arg of pow can't be negative)
    let lms1: vec3f = pow( kCONEtoLMS*lin1, vec3(1.0/3.0) );
    let lms2: vec3f = pow( kCONEtoLMS*lin2, vec3(1.0/3.0) );
    // lerp
    var lms: vec3f = mix( lms1, lms2, a );
    // gain in the middle (no oklab anymore, but looks better?)
    lms *= 1.0+0.2*a*(1.0-a);
    // cone to rgb
    return kLMStoCONE*(lms*lms*lms);
}