#[doc = "Register `STGENR_PIDR5` reader"]
pub type R = crate::R<STGENR_PIDR5rs>;
#[doc = "Field `PIDR5` reader - PIDR5"]
pub type PIDR5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR5"]
    #[inline(always)]
    pub fn pidr5(&self) -> PIDR5_R {
        PIDR5_R::new(self.bits)
    }
}
#[doc = "STGENR peripheral ID5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENR_PIDR5rs;
impl crate::RegisterSpec for STGENR_PIDR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenr_pidr5::R`](R) reader structure"]
impl crate::Readable for STGENR_PIDR5rs {}
#[doc = "`reset()` method sets STGENR_PIDR5 to value 0"]
impl crate::Resettable for STGENR_PIDR5rs {
    const RESET_VALUE: u32 = 0;
}
