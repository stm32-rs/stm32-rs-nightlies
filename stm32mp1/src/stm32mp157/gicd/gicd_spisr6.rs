#[doc = "Register `GICD_SPISR6` reader"]
pub type R = crate::R<GICD_SPISR6rs>;
#[doc = "Field `SPISR6` reader - SPISR6"]
pub type SPISR6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR6"]
    #[inline(always)]
    pub fn spisr6(&self) -> SPISR6_R {
        SPISR6_R::new(self.bits)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SPISR6rs;
impl crate::RegisterSpec for GICD_SPISR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spisr6::R`](R) reader structure"]
impl crate::Readable for GICD_SPISR6rs {}
#[doc = "`reset()` method sets GICD_SPISR6 to value 0"]
impl crate::Resettable for GICD_SPISR6rs {
    const RESET_VALUE: u32 = 0;
}
