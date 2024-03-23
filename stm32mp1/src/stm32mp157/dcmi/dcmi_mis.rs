#[doc = "Register `DCMI_MIS` reader"]
pub type R = crate::R<DCMI_MISrs>;
#[doc = "Field `FRAME_MIS` reader - FRAME_MIS"]
pub type FRAME_MIS_R = crate::BitReader;
#[doc = "Field `OVR_MIS` reader - OVR_MIS"]
pub type OVR_MIS_R = crate::BitReader;
#[doc = "Field `ERR_MIS` reader - ERR_MIS"]
pub type ERR_MIS_R = crate::BitReader;
#[doc = "Field `VSYNC_MIS` reader - VSYNC_MIS"]
pub type VSYNC_MIS_R = crate::BitReader;
#[doc = "Field `LINE_MIS` reader - LINE_MIS"]
pub type LINE_MIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FRAME_MIS"]
    #[inline(always)]
    pub fn frame_mis(&self) -> FRAME_MIS_R {
        FRAME_MIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OVR_MIS"]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERR_MIS"]
    #[inline(always)]
    pub fn err_mis(&self) -> ERR_MIS_R {
        ERR_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC_MIS"]
    #[inline(always)]
    pub fn vsync_mis(&self) -> VSYNC_MIS_R {
        VSYNC_MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LINE_MIS"]
    #[inline(always)]
    pub fn line_mis(&self) -> LINE_MIS_R {
        LINE_MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCMI_MISrs;
impl crate::RegisterSpec for DCMI_MISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmi_mis::R`](R) reader structure"]
impl crate::Readable for DCMI_MISrs {}
#[doc = "`reset()` method sets DCMI_MIS to value 0"]
impl crate::Resettable for DCMI_MISrs {
    const RESET_VALUE: u32 = 0;
}
