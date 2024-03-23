#[doc = "Register `RIS` reader"]
pub type R = crate::R<RISrs>;
#[doc = "Capture complete raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_RIS {
    #[doc = "0: No new capture"]
    NoNewCapture = 0,
    #[doc = "1: A frame has been captured"]
    FrameCaptured = 1,
}
impl From<FRAME_RIS> for bool {
    #[inline(always)]
    fn from(variant: FRAME_RIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME_RIS` reader - Capture complete raw interrupt status"]
pub type FRAME_RIS_R = crate::BitReader<FRAME_RIS>;
impl FRAME_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRAME_RIS {
        match self.bits {
            false => FRAME_RIS::NoNewCapture,
            true => FRAME_RIS::FrameCaptured,
        }
    }
    #[doc = "No new capture"]
    #[inline(always)]
    pub fn is_no_new_capture(&self) -> bool {
        *self == FRAME_RIS::NoNewCapture
    }
    #[doc = "A frame has been captured"]
    #[inline(always)]
    pub fn is_frame_captured(&self) -> bool {
        *self == FRAME_RIS::FrameCaptured
    }
}
#[doc = "Overrun raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_RIS {
    #[doc = "0: No data buffer overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: A data buffer overrun occurred and the data FIFO is corrupted. The bit is cleared by setting the OVR_ISC bit of the DCMI_ICR register"]
    OverrunOccured = 1,
}
impl From<OVR_RIS> for bool {
    #[inline(always)]
    fn from(variant: OVR_RIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_RIS` reader - Overrun raw interrupt status"]
pub type OVR_RIS_R = crate::BitReader<OVR_RIS>;
impl OVR_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_RIS {
        match self.bits {
            false => OVR_RIS::NoOverrun,
            true => OVR_RIS::OverrunOccured,
        }
    }
    #[doc = "No data buffer overrun occurred"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_RIS::NoOverrun
    }
    #[doc = "A data buffer overrun occurred and the data FIFO is corrupted. The bit is cleared by setting the OVR_ISC bit of the DCMI_ICR register"]
    #[inline(always)]
    pub fn is_overrun_occured(&self) -> bool {
        *self == OVR_RIS::OverrunOccured
    }
}
#[doc = "Synchronization error raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_RIS {
    #[doc = "0: No synchronization error detected"]
    NoError = 0,
    #[doc = "1: Embedded synchronization characters are not received in the correct order"]
    SynchronizationError = 1,
}
impl From<ERR_RIS> for bool {
    #[inline(always)]
    fn from(variant: ERR_RIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_RIS` reader - Synchronization error raw interrupt status"]
pub type ERR_RIS_R = crate::BitReader<ERR_RIS>;
impl ERR_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERR_RIS {
        match self.bits {
            false => ERR_RIS::NoError,
            true => ERR_RIS::SynchronizationError,
        }
    }
    #[doc = "No synchronization error detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERR_RIS::NoError
    }
    #[doc = "Embedded synchronization characters are not received in the correct order"]
    #[inline(always)]
    pub fn is_synchronization_error(&self) -> bool {
        *self == ERR_RIS::SynchronizationError
    }
}
#[doc = "VSYNC raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_RIS {
    #[doc = "0: Interrupt cleared"]
    Cleared = 0,
    #[doc = "1: Interrupt set"]
    Set = 1,
}
impl From<VSYNC_RIS> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_RIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNC_RIS` reader - VSYNC raw interrupt status"]
pub type VSYNC_RIS_R = crate::BitReader<VSYNC_RIS>;
impl VSYNC_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VSYNC_RIS {
        match self.bits {
            false => VSYNC_RIS::Cleared,
            true => VSYNC_RIS::Set,
        }
    }
    #[doc = "Interrupt cleared"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == VSYNC_RIS::Cleared
    }
    #[doc = "Interrupt set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == VSYNC_RIS::Set
    }
}
#[doc = "Line raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_RIS {
    #[doc = "0: Interrupt cleared"]
    Cleared = 0,
    #[doc = "1: Interrupt set"]
    Set = 1,
}
impl From<LINE_RIS> for bool {
    #[inline(always)]
    fn from(variant: LINE_RIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE_RIS` reader - Line raw interrupt status"]
pub type LINE_RIS_R = crate::BitReader<LINE_RIS>;
impl LINE_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINE_RIS {
        match self.bits {
            false => LINE_RIS::Cleared,
            true => LINE_RIS::Set,
        }
    }
    #[doc = "Interrupt cleared"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == LINE_RIS::Cleared
    }
    #[doc = "Interrupt set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == LINE_RIS::Set
    }
}
impl R {
    #[doc = "Bit 0 - Capture complete raw interrupt status"]
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun raw interrupt status"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error raw interrupt status"]
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC raw interrupt status"]
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line raw interrupt status"]
    #[inline(always)]
    pub fn line_ris(&self) -> LINE_RIS_R {
        LINE_RIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISrs;
impl crate::RegisterSpec for RISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RISrs {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RISrs {
    const RESET_VALUE: u32 = 0;
}
