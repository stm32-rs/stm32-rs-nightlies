#[doc = "Register `GICD_SPISR5` reader"]
pub type R = crate::R<GICD_SPISR5rs>;
#[doc = "Field `SPISR5` reader - SPISR5"]
pub type SPISR5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR5"]
    #[inline(always)]
    pub fn spisr5(&self) -> SPISR5_R {
        SPISR5_R::new(self.bits)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SPISR5rs;
impl crate::RegisterSpec for GICD_SPISR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spisr5::R`](R) reader structure"]
impl crate::Readable for GICD_SPISR5rs {}
#[doc = "`reset()` method sets GICD_SPISR5 to value 0"]
impl crate::Resettable for GICD_SPISR5rs {
    const RESET_VALUE: u32 = 0;
}
