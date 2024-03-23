#[doc = "Register `FDCAN_TURNA` reader"]
pub type R = crate::R<FDCAN_TURNArs>;
#[doc = "Field `NAV` reader - Numerator Actual Value"]
pub type NAV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Numerator Actual Value"]
    #[inline(always)]
    pub fn nav(&self) -> NAV_R {
        NAV_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "FDCAN TUR Numerator Actual Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_turna::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TURNArs;
impl crate::RegisterSpec for FDCAN_TURNArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_turna::R`](R) reader structure"]
impl crate::Readable for FDCAN_TURNArs {}
#[doc = "`reset()` method sets FDCAN_TURNA to value 0"]
impl crate::Resettable for FDCAN_TURNArs {
    const RESET_VALUE: u32 = 0;
}
