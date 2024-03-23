#[doc = "Register `MIS` reader"]
pub type R = crate::R<MISrs>;
#[doc = "Field `OVR_MIS` reader - Data buffer overrun/underrun masked interrupt status This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1."]
pub type OVR_MIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Data buffer overrun/underrun masked interrupt status This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1."]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PSSI masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
