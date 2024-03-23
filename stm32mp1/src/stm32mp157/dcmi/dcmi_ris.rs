#[doc = "Register `DCMI_RIS` reader"]
pub type R = crate::R<DCMI_RISrs>;
#[doc = "Field `FRAME_RIS` reader - FRAME_RIS"]
pub type FRAME_RIS_R = crate::BitReader;
#[doc = "Field `OVR_RIS` reader - OVR_RIS"]
pub type OVR_RIS_R = crate::BitReader;
#[doc = "Field `ERR_RIS` reader - ERR_RIS"]
pub type ERR_RIS_R = crate::BitReader;
#[doc = "Field `VSYNC_RIS` reader - VSYNC_RIS"]
pub type VSYNC_RIS_R = crate::BitReader;
#[doc = "Field `LINE_RIS` reader - LINE_RIS"]
pub type LINE_RIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FRAME_RIS"]
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OVR_RIS"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERR_RIS"]
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC_RIS"]
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LINE_RIS"]
    #[inline(always)]
    pub fn line_ris(&self) -> LINE_RIS_R {
        LINE_RIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCMI_RISrs;
impl crate::RegisterSpec for DCMI_RISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmi_ris::R`](R) reader structure"]
impl crate::Readable for DCMI_RISrs {}
#[doc = "`reset()` method sets DCMI_RIS to value 0"]
impl crate::Resettable for DCMI_RISrs {
    const RESET_VALUE: u32 = 0;
}
