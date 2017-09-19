//! Rust bindings for ESCAPI (Extremely Simple Capture API)

// use std::error::Error;

mod ffi;
use ffi::*;

pub struct Capture {
	params: SimpleCapParams,
	device_id: u32,
}

impl Capture {
	/// return the number of capture devices found
	pub fn count_devices() -> u32 {
		unsafe { countCaptureDevices() }
	}

	/// get the user-friendly name of a capture device
	pub fn get_device_name(device_id: u32) -> &'static str {
		unsafe {
			getCaptureDeviceName(device_id, CAPTURE_DEVICE_NAME_BUF.as_mut_ptr(), CAPTURE_DEVICE_NAME_LEN as u32);
			::std::ffi::CStr::from_ptr(CAPTURE_DEVICE_NAME_BUF.as_ptr()).to_str().unwrap()
		}
	}

	/// creates a new Capture with the given size but doesn't start capturing yet
	pub fn new(device_id: u32, width: u32, height: u32, buf: &mut [u32]) -> Capture {
		assert_eq!(buf.len() as u32, width * height);
		Capture {
			params: SimpleCapParams { buf: buf.as_mut_ptr(), width, height },
			device_id: device_id
		}
	}

	/// starts capturing
	pub fn start(&mut self) -> Result<(), &'static str> {
		if unsafe { initCapture(self.device_id, &self.params) } == 0 {
			return Err("failed to start camera capture");
		}
		assert_eq!(unsafe { getCaptureErrorCode(self.device_id) }, 0);
		Ok(())
	}

	/// stop capturing
	pub fn stop(&mut self) {
		unsafe { deinitCapture(self.device_id); }
	}

	/// requests video frame to be captured
	pub fn do_capture(&mut self) {
		unsafe { doCapture(self.device_id); }
	}

	/// returns true when the requested frame has been captured
	pub fn is_capture_done(&mut self) -> bool {
		unsafe { isCaptureDone(self.device_id) != 0 }
	}

	/// returns size that the Capture was created with
	pub fn get_size(&self) -> (u32, u32) {
		(self.params.width, self.params.height)
	}

	/// get the captured frame in the buffer
	pub fn get_buf(&mut self) -> &u32 {
		unsafe { &*self.params.buf }
	}

	/// get the ESCAPI version
	pub fn get_version() -> u32 {
		unsafe { ESCAPIVersion() }
	}

	/// get the captured frame in the buffer
	pub fn get_error(&mut self) -> i32 {
		unsafe { getCaptureErrorCode(self.device_id) }
	}
}

impl Drop for Capture {
	fn drop(&mut self) {
		self.stop();
	}
}

const CAPTURE_DEVICE_NAME_LEN: usize = 1024;
static mut CAPTURE_DEVICE_NAME_BUF: [i8; CAPTURE_DEVICE_NAME_LEN] = [0; CAPTURE_DEVICE_NAME_LEN];
