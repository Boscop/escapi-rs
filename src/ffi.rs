use libc::*;

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
}

#[link(name = "escapi", kind="dylib")]
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

	/* Returns the ESCAPI DLL version. 0x200 for 2.0 */
	pub fn ESCAPIDLLVersion() -> c_uint;

	pub fn initCOM();
}
