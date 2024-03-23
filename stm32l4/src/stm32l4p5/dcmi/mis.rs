#[doc = "Register `MIS` reader"]
pub type R = crate::R<MISrs>;
#[doc = "Capture complete masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_MIS {
    #[doc = "0: No interrupt is generated after a complete capture"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated at the end of each received frame/crop window (in crop mode) and the FRAME_IE bit is set in DCMI_IER"]
    Enabled = 1,
}
impl From<FRAME_MIS> for bool {
    #[inline(always)]
    fn from(variant: FRAME_MIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME_MIS` reader - Capture complete masked interrupt status"]
pub type FRAME_MIS_R = crate::BitReader<FRAME_MIS>;
impl FRAME_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRAME_MIS {
        match self.bits {
            false => FRAME_MIS::Disabled,
            true => FRAME_MIS::Enabled,
        }
    }
    #[doc = "No interrupt is generated after a complete capture"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAME_MIS::Disabled
    }
    #[doc = "An interrupt is generated at the end of each received frame/crop window (in crop mode) and the FRAME_IE bit is set in DCMI_IER"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAME_MIS::Enabled
    }
}
#[doc = "Overrun masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_MIS {
    #[doc = "0: No interrupt is generated on overrun"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received and the OVR_IE bit is set in DCMI_IER"]
    Enabled = 1,
}
impl From<OVR_MIS> for bool {
    #[inline(always)]
    fn from(variant: OVR_MIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_MIS` reader - Overrun masked interrupt status"]
pub type OVR_MIS_R = crate::BitReader<OVR_MIS>;
impl OVR_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_MIS {
        match self.bits {
            false => OVR_MIS::Disabled,
            true => OVR_MIS::Enabled,
        }
    }
    #[doc = "No interrupt is generated on overrun"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_MIS::Disabled
    }
    #[doc = "An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received and the OVR_IE bit is set in DCMI_IER"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_MIS::Enabled
    }
}
#[doc = "Synchronization error masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_MIS {
    #[doc = "0: No interrupt is generated on a synchronization error"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if the embedded synchronization codes are not received in the correct order and the ERR_IE bit in DCMI_IER is set"]
    Enabled = 1,
}
impl From<ERR_MIS> for bool {
    #[inline(always)]
    fn from(variant: ERR_MIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_MIS` reader - Synchronization error masked interrupt status"]
pub type ERR_MIS_R = crate::BitReader<ERR_MIS>;
impl ERR_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERR_MIS {
        match self.bits {
            false => ERR_MIS::Disabled,
            true => ERR_MIS::Enabled,
        }
    }
    #[doc = "No interrupt is generated on a synchronization error"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERR_MIS::Disabled
    }
    #[doc = "An interrupt is generated if the embedded synchronization codes are not received in the correct order and the ERR_IE bit in DCMI_IER is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERR_MIS::Enabled
    }
}
#[doc = "VSYNC masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_MIS {
    #[doc = "0: No interrupt is generated on DCMI_VSYNC transitions"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state and the VSYNC_IE bit is set in DCMI_IER"]
    Enabled = 1,
}
impl From<VSYNC_MIS> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_MIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNC_MIS` reader - VSYNC masked interrupt status"]
pub type VSYNC_MIS_R = crate::BitReader<VSYNC_MIS>;
impl VSYNC_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VSYNC_MIS {
        match self.bits {
            false => VSYNC_MIS::Disabled,
            true => VSYNC_MIS::Enabled,
        }
    }
    #[doc = "No interrupt is generated on DCMI_VSYNC transitions"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSYNC_MIS::Disabled
    }
    #[doc = "An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state and the VSYNC_IE bit is set in DCMI_IER"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSYNC_MIS::Enabled
    }
}
#[doc = "Line masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_MIS {
    #[doc = "0: No interrupt generation when the line is received"]
    Disabled = 0,
    #[doc = "1: An Interrupt is generated when a line has been completely received and the LINE_IE bit is set in DCMI_IER"]
    Enabled = 1,
}
impl From<LINE_MIS> for bool {
    #[inline(always)]
    fn from(variant: LINE_MIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE_MIS` reader - Line masked interrupt status"]
pub type LINE_MIS_R = crate::BitReader<LINE_MIS>;
impl LINE_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINE_MIS {
        match self.bits {
            false => LINE_MIS::Disabled,
            true => LINE_MIS::Enabled,
        }
    }
    #[doc = "No interrupt generation when the line is received"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINE_MIS::Disabled
    }
    #[doc = "An Interrupt is generated when a line has been completely received and the LINE_IE bit is set in DCMI_IER"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINE_MIS::Enabled
    }
}
impl R {
    #[doc = "Bit 0 - Capture complete masked interrupt status"]
    #[inline(always)]
    pub fn frame_mis(&self) -> FRAME_MIS_R {
        FRAME_MIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun masked interrupt status"]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error masked interrupt status"]
    #[inline(always)]
    pub fn err_mis(&self) -> ERR_MIS_R {
        ERR_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC masked interrupt status"]
    #[inline(always)]
    pub fn vsync_mis(&self) -> VSYNC_MIS_R {
        VSYNC_MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line masked interrupt status"]
    #[inline(always)]
    pub fn line_mis(&self) -> LINE_MIS_R {
        LINE_MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISrs;
impl crate::RegisterSpec for MISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MISrs {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MISrs {
    const RESET_VALUE: u32 = 0;
}
