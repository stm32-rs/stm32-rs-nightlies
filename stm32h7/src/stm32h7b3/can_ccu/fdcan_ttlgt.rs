#[doc = "Register `FDCAN_TTLGT` reader"]
pub type R = crate::R<FDCAN_TTLGTrs>;
#[doc = "Field `LT` reader - Local Time"]
pub type LT_R = crate::FieldReader<u16>;
#[doc = "Field `GT` reader - Global Time"]
pub type GT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Local Time"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Global Time"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Local and Global Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttlgt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTLGTrs;
impl crate::RegisterSpec for FDCAN_TTLGTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttlgt::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTLGTrs {}
#[doc = "`reset()` method sets FDCAN_TTLGT to value 0"]
impl crate::Resettable for FDCAN_TTLGTrs {
    const RESET_VALUE: u32 = 0;
}
