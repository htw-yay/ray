use ray::{obj::Sphere, Camera, Scene, Settings, V};
fn main() {
    let settings = Settings::new(1280, 720, 30);
    let mut imgbuf = settings.build();
    let scene = Scene {
        objects: vec![
            Box::new(Sphere {
                center: V(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: V(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
        camera: Camera {
            pos: V(0.0, 0.0, 0.0),
            focal_length: 1.0,
        },
    };
    scene.render(&settings, &mut imgbuf);
    imgbuf.save("out.png").unwrap();
}
