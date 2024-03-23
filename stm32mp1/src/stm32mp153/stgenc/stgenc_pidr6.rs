#[doc = "Register `STGENC_PIDR6` reader"]
pub type R = crate::R<STGENC_PIDR6rs>;
#[doc = "Field `PIDR6` reader - PIDR6"]
pub type PIDR6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR6"]
    #[inline(always)]
    pub fn pidr6(&self) -> PIDR6_R {
        PIDR6_R::new(self.bits)
    }
}
#[doc = "STGENC peripheral ID6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenc_pidr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENC_PIDR6rs;
impl crate::RegisterSpec for STGENC_PIDR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenc_pidr6::R`](R) reader structure"]
impl crate::Readable for STGENC_PIDR6rs {}
#[doc = "`reset()` method sets STGENC_PIDR6 to value 0"]
impl crate::Resettable for STGENC_PIDR6rs {
    const RESET_VALUE: u32 = 0;
}
