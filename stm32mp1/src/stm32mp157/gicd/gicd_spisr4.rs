#[doc = "Register `GICD_SPISR4` reader"]
pub type R = crate::R<GICD_SPISR4rs>;
#[doc = "Field `SPISR4` reader - SPISR4"]
pub type SPISR4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR4"]
    #[inline(always)]
    pub fn spisr4(&self) -> SPISR4_R {
        SPISR4_R::new(self.bits)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SPISR4rs;
impl crate::RegisterSpec for GICD_SPISR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spisr4::R`](R) reader structure"]
impl crate::Readable for GICD_SPISR4rs {}
#[doc = "`reset()` method sets GICD_SPISR4 to value 0"]
impl crate::Resettable for GICD_SPISR4rs {
    const RESET_VALUE: u32 = 0;
}
