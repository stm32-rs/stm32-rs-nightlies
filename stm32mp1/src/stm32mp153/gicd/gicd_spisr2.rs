#[doc = "Register `GICD_SPISR2` reader"]
pub type R = crate::R<GICD_SPISR2rs>;
#[doc = "Field `SPISR2` reader - SPISR2"]
pub type SPISR2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR2"]
    #[inline(always)]
    pub fn spisr2(&self) -> SPISR2_R {
        SPISR2_R::new(self.bits)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SPISR2rs;
impl crate::RegisterSpec for GICD_SPISR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spisr2::R`](R) reader structure"]
impl crate::Readable for GICD_SPISR2rs {}
#[doc = "`reset()` method sets GICD_SPISR2 to value 0"]
impl crate::Resettable for GICD_SPISR2rs {
    const RESET_VALUE: u32 = 0;
}
