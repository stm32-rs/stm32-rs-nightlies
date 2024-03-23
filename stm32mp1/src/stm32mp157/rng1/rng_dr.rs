#[doc = "Register `RNG_DR` reader"]
pub type R = crate::R<RNG_DRrs>;
#[doc = "Field `RNDATA` reader - RNDATA"]
pub type RNDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RNDATA"]
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new(self.bits)
    }
}
#[doc = "The RNG_DR register is a read-only register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_DRrs;
impl crate::RegisterSpec for RNG_DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_dr::R`](R) reader structure"]
impl crate::Readable for RNG_DRrs {}
#[doc = "`reset()` method sets RNG_DR to value 0"]
impl crate::Resettable for RNG_DRrs {
    const RESET_VALUE: u32 = 0;
}
