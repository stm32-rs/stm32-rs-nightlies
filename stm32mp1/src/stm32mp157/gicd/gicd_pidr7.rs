#[doc = "Register `GICD_PIDR7` reader"]
pub type R = crate::R<GICD_PIDR7rs>;
#[doc = "Field `PIDR7` reader - PIDR7"]
pub type PIDR7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR7"]
    #[inline(always)]
    pub fn pidr7(&self) -> PIDR7_R {
        PIDR7_R::new(self.bits)
    }
}
#[doc = "GICD peripheral ID5 to ID7 register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR7rs;
impl crate::RegisterSpec for GICD_PIDR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr7::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR7rs {}
#[doc = "`reset()` method sets GICD_PIDR7 to value 0"]
impl crate::Resettable for GICD_PIDR7rs {
    const RESET_VALUE: u32 = 0;
}
