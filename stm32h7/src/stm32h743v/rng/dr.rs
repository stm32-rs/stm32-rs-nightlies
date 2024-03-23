#[doc = "Register `DR` reader"]
pub type R = crate::R<DRrs>;
#[doc = "Field `RNDATA` reader - Random data 32-bit random data which are valid when DRDY=1."]
pub type RNDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Random data 32-bit random data which are valid when DRDY=1."]
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new(self.bits)
    }
}
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DRrs {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0;
}
