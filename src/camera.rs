use glium::Surface;

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: [f32; 3],
    pub rotation: [f32; 3]
}

pub fn new() -> Camera {
    Camera {
        position: [0.0; 3],
        rotation: [0.0; 3]
    }
}

pub fn view(camera: &Camera) -> [[f32; 4]; 4] {
    let position = camera.position;
    //let r = camera.rotation;
    let up = [0.0, 1.0, 0.0f32];

    let direction = [-2.0, 1.0, 1.0f32];

    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]
    ];

    let p = [
        - position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] - s_norm[2],
        - position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        - position[0] * f[0] - position[1] * f[1] - position[2] * f[2]
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0],      p[1], p[2], 1.0]
    ]
}

pub fn perspective(target: &glium::Frame) -> [[f32; 4]; 4] {
    let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    let fov = std::f32::consts::PI / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio,      0.0,    0.0,                                0.0],
        [0.0,                   f,      0.0,                                0.0],
        [0.0,                   0.0,    (zfar+znear)/(zfar-znear),          1.0],
        [0.0,                   0.0,    -(2.0*zfar*znear)/(zfar-znear),     0.0]
    ]
}