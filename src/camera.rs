use rscam::Frame;

pub const CAM_W: u32 = 640;
pub const CAM_H: u32 = 480;
pub const CAM_FPS: u32 = 30;

pub struct Camera {
    inner: rscam::Camera,
}

impl Camera {
    pub fn open() -> Result<Self, rscam::Error> {
        let mut cam = rscam::new("/dev/video0")?;
        cam.start(&rscam::Config {
            interval: (1, CAM_FPS),
            resolution: (CAM_W, CAM_H),
            format: b"YUYV",
            ..Default::default()
        })?;
        Ok(Self { inner: cam })
    }

    pub fn capture(&self) -> std::io::Result<Frame> {
        self.inner.capture()
    }
}