#[doc = "Register `TURNA` reader"]
pub type R = crate::R<TURNArs>;
#[doc = "Field `NAV` reader - Numerator Actual Value"]
pub type NAV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Numerator Actual Value"]
    #[inline(always)]
    pub fn nav(&self) -> NAV_R {
        NAV_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "FDCAN TUR Numerator Actual Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`turna::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TURNArs;
impl crate::RegisterSpec for TURNArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`turna::R`](R) reader structure"]
impl crate::Readable for TURNArs {}
#[doc = "`reset()` method sets TURNA to value 0"]
impl crate::Resettable for TURNArs {
    const RESET_VALUE: u32 = 0;
}
