use std::{
    borrow::BorrowMut,
    io::{self, stdout, Read, Write},
    time::Instant,
};

use windows_capture::{
    capture::GraphicsCaptureApiHandler,
    encoder::{VideoEncoder, VideoEncoderQuality, VideoEncoderType},
    frame::{Frame, ImageFormat},
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};

// This struct will be used to handle the capture events.
struct Capture {
    start: Instant,
    count: u32,
    stdin: std::process::ChildStdin,
    stdout: std::process::ChildStdout
}

impl GraphicsCaptureApiHandler for Capture {
    // The type of flags used to get the values from the settings.
    type Flags = Monitor;

    // The type of error that can occur during capture, the error will be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create the struct. The flags can be passed from settings.
    fn new(message: Self::Flags) -> Result<Self, Self::Error> {
        println!("{:?}", message);
        let h = message.height().unwrap();
        let w = message.width().unwrap();
        let mut child = std::process::Command::new("ffmpeg")
            .arg("-f")
            .arg("rawvideo")
            .arg("-pix_fmt")
            .arg("rgba")
            .arg("-s")
            .arg(format!("{}x{}", w, h))
            .arg("-i")
            .arg("-")
            .arg("-f")
            .arg("mpegts")
            .arg("-b:v")
            .arg("15000k")
            .arg("-fflags")
            .arg("nobuffer")
            .arg("pipe:1")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let stdin = child.stdin.unwrap();
        let stdout = child.stdout.unwrap();
        Ok(Self {
            start: Instant::now(),
            count: 0,
            stdin,
            stdout
        })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        self.count += 1;
        let mut _data = frame.buffer()?;

        let d = _data.as_raw_nopadding_buffer().unwrap();
        self.stdin.write_all(d).unwrap();
        self.stdin.flush().unwrap();

        if self.start.elapsed().as_secs() == 1000 {
            capture_control.stop();
            println!("{}", self.count);
        }

        Ok(())
    }

    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture Session Closed");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> () {
        tracing_subscriber::fmt::init();
        ()
    }
    #[test]
    fn list_devices() {
        setup();
        let mut manager = crate::modules::screen_capture::win::DXGIManager::new(10).unwrap();
        let (w, h) = manager.geometry();
        let mut count = 0;

        let primary_monitor = Monitor::from_index(2).expect("There is no primary monitor");
        let settings = Settings::new(
            primary_monitor,
            CursorCaptureSettings::Default,
            DrawBorderSettings::Default,
            ColorFormat::Rgba8,
            primary_monitor,
        )
        .unwrap();

        Capture::start(settings).expect("Screen Capture Failed");
    }

}
