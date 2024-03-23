#[doc = "Register `MIS` reader"]
pub type R = crate::R<MISrs>;
#[doc = "Field `FRAME_MIS` reader - Capture complete masked interrupt status This bit gives the status of the masked capture complete interrupt"]
pub type FRAME_MIS_R = crate::BitReader;
#[doc = "Field `OVR_MIS` reader - Overrun masked interrupt status This bit gives the status of the masked overflow interrupt."]
pub type OVR_MIS_R = crate::BitReader;
#[doc = "Field `ERR_MIS` reader - Synchronization error masked interrupt status This bit gives the status of the masked synchronization error interrupt. Note: This bit is available only in embedded synchronization mode."]
pub type ERR_MIS_R = crate::BitReader;
#[doc = "Field `VSYNC_MIS` reader - VSYNC masked interrupt status This bit gives the status of the masked VSYNC interrupt. The active state of the DCMI_VSYNC signal is defined by the VSPOL bit."]
pub type VSYNC_MIS_R = crate::BitReader;
#[doc = "Field `LINE_MIS` reader - Line masked interrupt status This bit gives the status of the masked line interrupt."]
pub type LINE_MIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Capture complete masked interrupt status This bit gives the status of the masked capture complete interrupt"]
    #[inline(always)]
    pub fn frame_mis(&self) -> FRAME_MIS_R {
        FRAME_MIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun masked interrupt status This bit gives the status of the masked overflow interrupt."]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error masked interrupt status This bit gives the status of the masked synchronization error interrupt. Note: This bit is available only in embedded synchronization mode."]
    #[inline(always)]
    pub fn err_mis(&self) -> ERR_MIS_R {
        ERR_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC masked interrupt status This bit gives the status of the masked VSYNC interrupt. The active state of the DCMI_VSYNC signal is defined by the VSPOL bit."]
    #[inline(always)]
    pub fn vsync_mis(&self) -> VSYNC_MIS_R {
        VSYNC_MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line masked interrupt status This bit gives the status of the masked line interrupt."]
    #[inline(always)]
    pub fn line_mis(&self) -> LINE_MIS_R {
        LINE_MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "DCMI masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
