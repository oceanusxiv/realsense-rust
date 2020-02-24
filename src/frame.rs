use crate::{
    base::Resolution,
    error::{ErrorChecker, Result as RsResult},
    kind::{Extension, FrameMetaDataValue, TimestampDomain},
    pose_data::PoseData,
    sensor::{marker as sensor_marker, Sensor},
    stream_profile::StreamProfile,
};
use nalgebra::{base::SliceStorage, Vector, U1, U3};
use num_traits::FromPrimitive;
use std::{
    iter::FusedIterator, marker::PhantomData, mem::MaybeUninit, os::raw::c_int, path::Path,
    ptr::NonNull,
};

type MotionData<'a> = Vector<f32, U3, SliceStorage<'a, f32, U3, U1, U1, U3>>;

pub mod marker {
    use super::*;

    pub trait FrameKind {}

    pub trait NonAnyFrameKind
    where
        Self: FrameKind,
    {
        const TYPE: Extension;
    }

    #[derive(Debug)]
    pub struct Composite;

    impl FrameKind for Composite {}
    impl NonAnyFrameKind for Composite {
        const TYPE: Extension = Extension::CompositeFrame;
    }

    #[derive(Debug)]
    pub struct Any;

    impl FrameKind for Any {}

    #[derive(Debug)]
    pub struct Video;

    impl FrameKind for Video {}
    impl NonAnyFrameKind for Video {
        const TYPE: Extension = Extension::VideoFrame;
    }

    #[derive(Debug)]
    pub struct Motion;

    impl FrameKind for Motion {}
    impl NonAnyFrameKind for Motion {
        const TYPE: Extension = Extension::MotionFrame;
    }

    #[derive(Debug)]
    pub struct Depth;

    impl FrameKind for Depth {}
    impl NonAnyFrameKind for Depth {
        const TYPE: Extension = Extension::DepthFrame;
    }

    #[derive(Debug)]
    pub struct Disparity;

    impl FrameKind for Disparity {}
    impl NonAnyFrameKind for Disparity {
        const TYPE: Extension = Extension::DisparityFrame;
    }

    #[derive(Debug)]
    pub struct Pose;

    impl FrameKind for Pose {}
    impl NonAnyFrameKind for Pose {
        const TYPE: Extension = Extension::PoseFrame;
    }

    #[derive(Debug)]
    pub struct Points;

    impl FrameKind for Points {}
    impl NonAnyFrameKind for Points {
        const TYPE: Extension = Extension::Points;
    }
}

#[derive(Debug)]
pub struct Frame<Kind>
where
    Kind: marker::FrameKind,
{
    pub(crate) ptr: NonNull<realsense_sys::rs2_frame>,
    _phantom: PhantomData<Kind>,
}

impl<Kind> Frame<Kind>
where
    Kind: marker::FrameKind,
{
    pub fn metadata(&self, kind: FrameMetaDataValue) -> RsResult<u64> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_get_frame_metadata(
                self.ptr.as_ptr(),
                kind as realsense_sys::rs2_frame_metadata_value,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Ok(val as u64)
        }
    }

    pub fn number(&self) -> RsResult<u64> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val =
                realsense_sys::rs2_get_frame_number(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            Ok(val as u64)
        }
    }

    pub fn data_size(&self) -> RsResult<usize> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val =
                realsense_sys::rs2_get_frame_data_size(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            Ok(val as usize)
        }
    }

    pub fn timestamp(&self) -> RsResult<f64> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val =
                realsense_sys::rs2_get_frame_timestamp(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            Ok(val as f64)
        }
    }

    pub fn timestamp_domain(&self) -> RsResult<TimestampDomain> {
        let val = unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_get_frame_timestamp_domain(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            val
        };
        let domain = TimestampDomain::from_u32(val).unwrap();
        Ok(domain)
    }

    pub fn data(&self) -> RsResult<&[u8]> {
        let size = self.data_size()?;
        let slice = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = realsense_sys::rs2_get_frame_data(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            std::slice::from_raw_parts::<u8>(ptr.cast::<u8>(), size)
        };
        Ok(slice)
    }

    pub fn sensor(&self) -> RsResult<Sensor<sensor_marker::Any>> {
        let sensor = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr =
                realsense_sys::rs2_get_frame_sensor(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            Sensor::from_ptr(NonNull::new(ptr).unwrap())
        };
        Ok(sensor)
    }

    pub fn stream_profile(&self) -> RsResult<StreamProfile> {
        let profile = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = realsense_sys::rs2_get_frame_stream_profile(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            StreamProfile::from_parts(NonNull::new(ptr as *mut _).unwrap(), false)
        };
        Ok(profile)
    }

    pub(crate) unsafe fn take(mut self) -> NonNull<realsense_sys::rs2_frame> {
        let ptr = std::mem::replace(&mut self.ptr, MaybeUninit::uninit().assume_init());
        std::mem::forget(self);
        ptr
    }

    pub(crate) fn from_ptr(ptr: NonNull<realsense_sys::rs2_frame>) -> Self {
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }
}

impl Frame<marker::Any> {
    pub fn try_extend_to<Kind>(self) -> RsResult<Result<Frame<Kind>, Self>>
    where
        Kind: marker::NonAnyFrameKind,
    {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_is_frame_extendable_to(
                self.ptr.as_ptr(),
                Kind::TYPE as realsense_sys::rs2_extension,
                checker.inner_mut_ptr(),
            );
            checker.check()?;

            let is_extendable = val != 0;
            if is_extendable {
                let ptr = self.take();
                let frame = Frame {
                    ptr,
                    _phantom: PhantomData,
                };
                Ok(Ok(frame))
            } else {
                Ok(Err(self))
            }
        }
    }
}

impl Frame<marker::Composite> {
    pub fn len(&self) -> RsResult<usize> {
        let len = unsafe {
            let mut checker = ErrorChecker::new();
            let len = realsense_sys::rs2_embedded_frames_count(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            len as usize
        };
        Ok(len)
    }

    pub fn get(&self, index: usize) -> RsResult<Option<Frame<marker::Any>>> {
        let len = self.len()?;
        if index >= len {
            return Ok(None);
        }

        let frame = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = realsense_sys::rs2_extract_frame(
                self.ptr.as_ptr(),
                index as c_int,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Frame::from_ptr(NonNull::new(ptr).unwrap())
        };
        Ok(Some(frame))
    }

    pub fn try_into_iter(self) -> RsResult<CompositeFrameIntoIter> {
        let len = self.len()?;
        let ptr = unsafe { self.take() };
        let iter = CompositeFrameIntoIter {
            index: 0,
            len,
            ptr,
            fused: len == 0,
        };
        Ok(iter)
    }
}

impl Frame<marker::Depth> {
    pub fn get_distance(&self, x: usize, y: usize) -> RsResult<f32> {
        let distance = unsafe {
            let mut checker = ErrorChecker::new();
            let distance = realsense_sys::rs2_depth_frame_get_distance(
                self.ptr.as_ptr(),
                x as c_int,
                y as c_int,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            distance
        };
        Ok(distance)
    }
}

impl Frame<marker::Video> {
    pub fn resolution(&self) -> RsResult<Resolution> {
        let width = self.width()?;
        let height = self.width()?;
        let resolution = Resolution { width, height };
        Ok(resolution)
    }

    pub fn width(&self) -> RsResult<usize> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val =
                realsense_sys::rs2_get_frame_width(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            Ok(val as usize)
        }
    }

    pub fn height(&self) -> RsResult<usize> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val =
                realsense_sys::rs2_get_frame_height(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            Ok(val as usize)
        }
    }

    pub fn stride_in_bytes(&self) -> RsResult<usize> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_get_frame_stride_in_bytes(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Ok(val as usize)
        }
    }

    pub fn bits_per_pixel(&self) -> RsResult<usize> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_get_frame_bits_per_pixel(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Ok(val as usize)
        }
    }
}

impl Frame<marker::Pose> {
    pub fn pose(&self) -> RsResult<PoseData> {
        let pose_data = unsafe {
            let mut checker = ErrorChecker::new();
            let mut pose_data = MaybeUninit::uninit();
            realsense_sys::rs2_pose_frame_get_pose_data(
                self.ptr.as_ptr(),
                pose_data.as_mut_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            pose_data.assume_init()
        };

        let pose = PoseData(pose_data);
        Ok(pose)
    }
}

impl Frame<marker::Disparity> {
    pub fn get_baseline(&self) -> RsResult<f32> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let baseline = realsense_sys::rs2_depth_stereo_frame_get_baseline(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Ok(baseline)
        }
    }
}

impl Frame<marker::Points> {
    pub fn vertices<'a>(&'a self) -> RsResult<&'a [realsense_sys::rs2_vertex]> {
        let n_points = self.points_count()?;
        unsafe {
            let mut checker = ErrorChecker::new();
            let ptr =
                realsense_sys::rs2_get_frame_vertices(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            let slice = std::slice::from_raw_parts::<realsense_sys::rs2_vertex>(ptr, n_points);
            Ok(slice)
        }
    }

    pub fn texture_coordinates<'a>(&'a self) -> RsResult<&'a [realsense_sys::rs2_pixel]> {
        unsafe {
            let n_points = self.points_count()?;
            let mut checker = ErrorChecker::new();
            let ptr = realsense_sys::rs2_get_frame_texture_coordinates(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            let slice = std::slice::from_raw_parts::<realsense_sys::rs2_pixel>(ptr, n_points);
            Ok(slice)
        }
    }

    pub fn points_count(&self) -> RsResult<usize> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_get_frame_points_count(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Ok(val as usize)
        }
    }
}

impl Frame<marker::Motion> {
    pub fn get_motion_data(&self) -> RsResult<MotionData> {
        let data = unsafe {
            let data: &[f32] = std::mem::transmute(self.data()?);
            let storage = SliceStorage::from_raw_parts(data.as_ptr(), (U3, U1), (U1, U3));
            MotionData::from_data(storage)
        };
        Ok(data)
    }
}

impl IntoIterator for Frame<marker::Composite> {
    type Item = RsResult<Frame<marker::Any>>;
    type IntoIter = CompositeFrameIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.try_into_iter().unwrap()
    }
}

impl<Kind> Drop for Frame<Kind>
where
    Kind: marker::FrameKind,
{
    fn drop(&mut self) {
        unsafe {
            realsense_sys::rs2_release_frame(self.ptr.as_ptr());
        }
    }
}

unsafe impl<Kind> Send for Frame<Kind> where Kind: marker::FrameKind {}

#[derive(Debug)]
pub struct CompositeFrameIntoIter {
    len: usize,
    index: usize,
    ptr: NonNull<realsense_sys::rs2_frame>,
    fused: bool,
}

impl Iterator for CompositeFrameIntoIter {
    type Item = RsResult<Frame<marker::Any>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fused {
            return None;
        }

        let ptr = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = realsense_sys::rs2_extract_frame(
                self.ptr.as_ptr(),
                self.index as c_int,
                checker.inner_mut_ptr(),
            );
            match checker.check() {
                Ok(()) => ptr,
                Err(err) => {
                    self.fused = true;
                    return Some(Err(err));
                }
            }
        };

        self.index += 1;
        if self.index >= self.len {
            self.fused = true;
        }

        let frame = Frame::from_ptr(NonNull::new(ptr).unwrap());
        Some(Ok(frame))
    }
}

impl FusedIterator for CompositeFrameIntoIter {}

impl Drop for CompositeFrameIntoIter {
    fn drop(&mut self) {
        unsafe {
            realsense_sys::rs2_release_frame(self.ptr.as_ptr());
        }
    }
}
