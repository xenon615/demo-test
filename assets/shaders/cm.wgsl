#import bevy_pbr::{
     forward_io::VertexOutput,
}

@fragment  
fn fragment(vo: VertexOutput) -> @location(0) vec4f {
    return vec4f(0 , 0., 1, 1.);
}