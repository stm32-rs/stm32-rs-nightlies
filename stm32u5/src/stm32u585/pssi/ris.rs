#[doc = "Register `RIS` reader"]
pub type R = crate::R<RISrs>;
#[doc = "Field `OVR_RIS` reader - OVR_RIS"]
pub type OVR_RIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - OVR_RIS"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PSSI raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
