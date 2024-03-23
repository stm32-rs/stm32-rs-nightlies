#[doc = "Register `STGENR_PIDR7` reader"]
pub type R = crate::R<STGENR_PIDR7rs>;
#[doc = "Field `PIDR7` reader - PIDR7"]
pub type PIDR7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR7"]
    #[inline(always)]
    pub fn pidr7(&self) -> PIDR7_R {
        PIDR7_R::new(self.bits)
    }
}
#[doc = "STGENR peripheral ID7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENR_PIDR7rs;
impl crate::RegisterSpec for STGENR_PIDR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenr_pidr7::R`](R) reader structure"]
impl crate::Readable for STGENR_PIDR7rs {}
#[doc = "`reset()` method sets STGENR_PIDR7 to value 0"]
impl crate::Resettable for STGENR_PIDR7rs {
    const RESET_VALUE: u32 = 0;
}
