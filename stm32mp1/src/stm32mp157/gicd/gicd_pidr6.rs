#[doc = "Register `GICD_PIDR6` reader"]
pub type R = crate::R<GICD_PIDR6rs>;
#[doc = "Field `PIDR6` reader - PIDR6"]
pub type PIDR6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR6"]
    #[inline(always)]
    pub fn pidr6(&self) -> PIDR6_R {
        PIDR6_R::new(self.bits)
    }
}
#[doc = "GICD peripheral ID5 to ID7 register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR6rs;
impl crate::RegisterSpec for GICD_PIDR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr6::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR6rs {}
#[doc = "`reset()` method sets GICD_PIDR6 to value 0"]
impl crate::Resettable for GICD_PIDR6rs {
    const RESET_VALUE: u32 = 0;
}
