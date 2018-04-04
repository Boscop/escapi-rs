use std::os::raw::*;

#[repr(C)]
pub struct SimpleCapParams {
	/* Target buffer.
	 * Must be at least width * height * sizeof(int) of size!
	 */
	pub buf: *mut c_uint,
	/* Buffer width */
	pub width: c_uint,
	/* Buffer height */
	pub height: c_uint,

	/* Minimum framerate */
	pub fps: c_float,
}

#[link(name = "escapi", kind = "dylib")]
extern "C" {
	/* return the number of capture devices found */
	pub fn countCaptureDevices() -> c_uint;

	/* initCapture tries to open the video capture device.
	 * Returns 0 on failure, 1 on success.
	 * Note: Capture parameter values must not change while capture device
	 *       is in use (i.e. between initCapture and deinitCapture).
	 *       Do *not* free the target buffer, or change its pointer!
	 */
	pub fn initCapture(deviceno: c_uint, aParams: *const SimpleCapParams) -> c_int;

	/* deinitCapture closes the video capture device. */
	pub fn deinitCapture(deviceno: c_uint);

	/* doCapture requests video frame to be captured. */
	pub fn doCapture(deviceno: c_uint);

	/* isCaptureDone returns 1 when the requested frame has been captured.*/
	pub fn isCaptureDone(deviceno: c_uint) -> c_int;

	/* Get the user-friendly name of a capture device. */
	pub fn getCaptureDeviceName(deviceno: c_uint, namebuffer: *mut c_char, bufferlength: c_uint);

	pub fn ESCAPIVersion() -> c_uint;
	pub fn getCapturePropertyValue(_: c_uint, _: c_int) -> c_float;
	pub fn getCapturePropertyAuto(_: c_uint, _: c_int) -> c_int;
	pub fn setCaptureProperty(_: c_uint, _: c_int, _: c_float, _: c_int);
	pub fn getCaptureErrorLine(_: c_uint) -> c_int;
	pub fn getCaptureErrorCode(_: c_uint) -> c_int;
}

/* escapi/enumprops/main.cpp
pub enum CaptureProperties {
	Brightness,
	Contrast,
	Hue,
	Saturation,
	Sharpness,
	Gamma,
	ColorEnable,
	WhiteBalance,
	BacklightCompensation,
	Gain,
	Pan,
	Tilt,
	Roll,
	Zoom,
	Exposure,
	Iris,
	Focus,
	PropMax,
};
*/