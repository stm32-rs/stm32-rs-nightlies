#[doc = "Register `GICH_EISR0` reader"]
pub type R = crate::R<GICH_EISR0rs>;
#[doc = "Field `EISR0` reader - EISR0"]
pub type EISR0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - EISR0"]
    #[inline(always)]
    pub fn eisr0(&self) -> EISR0_R {
        EISR0_R::new(self.bits)
    }
}
#[doc = "GICH end of interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_eisr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICH_EISR0rs;
impl crate::RegisterSpec for GICH_EISR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gich_eisr0::R`](R) reader structure"]
impl crate::Readable for GICH_EISR0rs {}
#[doc = "`reset()` method sets GICH_EISR0 to value 0"]
impl crate::Resettable for GICH_EISR0rs {
    const RESET_VALUE: u32 = 0;
}
