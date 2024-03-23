#[doc = "Register `GICD_SPISR7` reader"]
pub type R = crate::R<GICD_SPISR7rs>;
#[doc = "Field `SPISR7` reader - SPISR7"]
pub type SPISR7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR7"]
    #[inline(always)]
    pub fn spisr7(&self) -> SPISR7_R {
        SPISR7_R::new(self.bits)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SPISR7rs;
impl crate::RegisterSpec for GICD_SPISR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spisr7::R`](R) reader structure"]
impl crate::Readable for GICD_SPISR7rs {}
#[doc = "`reset()` method sets GICD_SPISR7 to value 0"]
impl crate::Resettable for GICD_SPISR7rs {
    const RESET_VALUE: u32 = 0;
}
