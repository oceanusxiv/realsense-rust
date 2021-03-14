//! Enumeration of frame-specific metadata

use num_derive::{FromPrimitive, ToPrimitive};
use realsense_sys as sys;

/// A type describing the different metadata keys used to access frame metadata.
///
/// Each key corresponds to a particular type of frame metadata. The librealsense2 C-API refers to
/// these as `rs2_frame_metadata_value`; however these are clearly keys to metadata values.
///
#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rs2FrameMetadata {
    /// A sequential index managed per-stream, counting up from the first frame at zero.
    FrameCounter = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_COUNTER as u32,
    /// Timestamp set by device clock when data is read out and transmission commences.
    ///
    /// Units are microseconds (usec)
    FrameTimestamp = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_TIMESTAMP as u32,
    /// Timestamp for the middle of the sensor's exposure during frame capture.
    ///
    /// This value is calculated by the device (not host).  Units are microseconds (usec)
    SensorTimestamp = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_SENSOR_TIMESTAMP as u32,
    /// The exposure duration used by the sensor when this frame was captured.
    ///
    /// When auto-exposure (AE) is turned on, this value is controlled by the device's firmware.
    /// Units are microseconds (usec).
    ActualExposure = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_ACTUAL_EXPOSURE as u32,
    /// The sensor's gain level during frame capture.
    ///
    /// This value is a relative integer value, and may not correspond to a physical quantity. When
    /// auto-exposure (AE) is turned on, this value is controlled by the device's firmware.
    GainLevel = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_GAIN_LEVEL as u32,
    /// Indicates if auto-exposure (AE) was turned on during frame capture.
    ///
    /// A value of zero corresponds to AE being off, otherwise it is on.
    AutoExposure = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_AUTO_EXPOSURE as u32,
    /// The white balance setting as a color temperature during frame capture.
    ///
    /// Units are Kelvin degrees.
    WhiteBalance = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_WHITE_BALANCE as u32,
    /// The timestamp at which the frame arrived on the host machine.
    ///
    /// This timestamp, unlike the others, is relative to the system clock on host.
    TimeOfArrival = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_TIME_OF_ARRIVAL as u32,
    /// Temperature of the device during frame capture.
    ///
    /// Units are Celsius degrees.
    Temperature = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_TEMPERATURE as u32,
    /// Timestamp of the uvc driver.
    ///
    /// Units are microseconds (usec)
    BackendTimestamp = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_BACKEND_TIMESTAMP as u32,
    /// The actual framerate of the stream at the point of frame capture.
    ///
    /// This may be different than the framerate returned by the stream profile data (which is the
    /// framerate you configured). This may differ as a result of frame drops in the firmware, or
    /// measurement noise (e.g. you get 29.99 FPS when you asked for 30 FPS).
    ActualFps = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_ACTUAL_FPS as u32,
    /// Relative measure of laser power during frame capture.
    ///
    /// Laser power is a relative measure between values of 0 and 360
    FrameLaserPower = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LASER_POWER as u32,
    /// The laser power mode used at time of frame capture.
    ///
    /// Zero corresponds to laser power being switched off, and one for laser power being turned
    /// on.
    ///
    /// This variant was deprecated by librealsense2, prefer using
    /// `Rs2FrameMetadata::FrameEmitterMode` instead.
    FrameLaserPowerMode =
        sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LASER_POWER_MODE as u32,
    /// Exposure priority
    ExposurePriority = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_PRIORITY as u32,
    /// Left region of interest for the auto-exposure algorithm.
    ExposureRoiLeft = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_LEFT as u32,
    /// Right region of interest for the auto-exposure algorithm.
    ExposureRoiRight = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_RIGHT as u32,
    /// Top region of interest for the auto-exposure algorithm.
    ExposureRoiTop = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_TOP as u32,
    /// Bottom region of interest for the auto-exposure algorithm.
    ExposureRoiBottom = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_BOTTOM as u32,
    /// Brightness of the color image at time of frame capture.
    Brightness = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_BRIGHTNESS as u32,
    /// Contrast of the color image at time of frame capture.
    Contrast = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_CONTRAST as u32,
    /// Saturation of the color image at time of frame capture.
    Saturation = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_SATURATION as u32,
    /// Sharpness of the color image at time of frame capture.
    Sharpness = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_SHARPNESS as u32,
    /// Indicator for whether auto-white-balance (AWB) was turned on during frame capture.
    ///
    /// Zero corresponds to automatic mode being switched off, otherwise it is on.
    AutoWhiteBalanceTemperature =
        sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_AUTO_WHITE_BALANCE_TEMPERATURE as u32,
    /// Indicator for whether backlight compensation was enabled on a color image.
    ///
    /// Zero corresponds to backlight compensation being switched off, otherwise it is on.
    BacklightCompensation =
        sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_BACKLIGHT_COMPENSATION as u32,
    /// Hue of the color image at time of frame capture.
    Hue = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_HUE as u32,
    /// Gamma of the color image at time of frame capture.
    Gamma = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_GAMMA as u32,
    /// White balance of the color image at time of frame capture.
    ManualWhiteBalance =
        sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_MANUAL_WHITE_BALANCE as u32,
    /// Power line frequency mode for anti-flickering.
    ///
    /// Values can be Off, 50Hz, 60Hz, and Auto.
    PowerLineFrequency =
        sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_POWER_LINE_FREQUENCY as u32,
    /// Indicator for whether lowlight compensation was enabled on the color image.
    ///
    /// Zero corresponds to lowlight compensation being switched off, otherwise it is on.
    LowLightCompensation =
        sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_LOW_LIGHT_COMPENSATION as u32,
    /// The frame emitter mode used at the time of frame capture.
    ///
    /// Possible values are:
    ///
    /// * 0 - all emitters disabled.
    /// * 1 - laser enabled.
    /// * 2 - auto laser enabled (opt).
    /// * 3 - LED enabled (opt).
    ///
    FrameEmitterMode = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_EMITTER_MODE as u32,
    /// Relative power of the LED emitter during frame capture.
    ///
    /// This is a relative measure between values of 0 and 360.
    FrameLedPower = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LED_POWER as u32,
    /// The number of transmitted payload bytes for the frame, not including metadata
    RawFrameSize = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_RAW_FRAME_SIZE as u32,
    /// GPIO input data
    GpioInputData = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_GPIO_INPUT_DATA as u32,
    /// Sub-preset identifier
    SequenceName = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_SEQUENCE_NAME as u32,
    /// Sub-preset sequence identifier
    SequenceIdentifier = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_SEQUENCE_ID as u32,
    /// Sub-preset sequence size
    SequenceSize = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_SEQUENCE_SIZE as u32,
    // Not included since this just tells us the total number of metadata fields
    //
    // Count = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_COUNT,
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::FromPrimitive;

    #[test]
    fn all_variants_exist() {
        for i in 0..sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_COUNT as u32 {
            assert!(
                Rs2FrameMetadata::from_u32(i).is_some(),
                "Rs2FrameMetadata variant for ordinal {} does not exist.",
                i,
            );
        }
    }
}
