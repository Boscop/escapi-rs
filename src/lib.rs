//! Rust bindings for ESCAPI (Extremely Simple Capture API)

use std::error::Error;

extern crate libc;
mod ffi;
use ffi::*;

pub struct Capture {
	params: SimpleCapParams,
	deviceno: u32,
}

impl Capture {
	/// return the number of capture devices found
	pub fn count_devices() -> u32 {
		ensure_initialized();
		unsafe { countCaptureDevices() }
	}
	/// get the user-friendly name of a capture device
	pub fn get_device_name(deviceno: u32) -> &'static str {
		ensure_initialized();
		unsafe {
			getCaptureDeviceName(deviceno, CAPTURE_DEVICE_NAME_BUF.as_mut_ptr(), CAPTURE_DEVICE_NAME_LEN as u32);
			::std::ffi::CStr::from_ptr(CAPTURE_DEVICE_NAME_BUF.as_ptr()).to_str().unwrap()
		}
	}
	/// creates a new Capture with the given size but doesn't start capturing yet
	pub fn new(deviceno: u32, width: usize, height: usize, buf: &mut [u32]) -> Capture {
		assert_eq!(buf.len(), width * height);
		ensure_initialized();
		Capture {
			params: SimpleCapParams {buf: buf.as_mut_ptr(), width: width as u32, height: height as u32},
			deviceno: deviceno
		}
	}
	/// starts capturing
	pub fn start(&mut self) -> Result<(), Box<Error>> {
		if unsafe { initCapture(self.deviceno, &self.params) } == 0 {
			return try!(Err("failed to start camera capture"));
		}
		Ok(())
	}
	/// stop capturing
	pub fn stop(&mut self) {
		unsafe { deinitCapture(self.deviceno); }
	}

	/// requests video frame to be captured
	pub fn do_capture(&mut self) {
		unsafe { doCapture(self.deviceno); }
	}

	/// returns true when the requested frame has been captured
	pub fn is_capture_done(&mut self) -> bool {
		unsafe { isCaptureDone(self.deviceno) != 0 }
	}

	/// returns size that the Capture was created with
	pub fn get_size(&self) -> (u32, u32) {
		(self.params.width, self.params.height)
	}

	/// get the captured frame in the buffer
	pub fn get_buf(&mut self) -> &u32 {
		unsafe { &*self.params.buf }
	}

	pub fn get_version() -> u32 {
		unsafe { ESCAPIDLLVersion() }
	}
}

fn ensure_initialized() {
	unsafe {
		if !INITIALIZED {
			/* Initialize COM.. */
			initCOM();
			INITIALIZED = true;
		}
	}
}

const CAPTURE_DEVICE_NAME_LEN: usize = 1024;
static mut CAPTURE_DEVICE_NAME_BUF: [i8; CAPTURE_DEVICE_NAME_LEN] = [0; CAPTURE_DEVICE_NAME_LEN];
static mut INITIALIZED: bool = false;
