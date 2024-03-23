#[doc = "Register `GICD_SPISR3` reader"]
pub type R = crate::R<GICD_SPISR3rs>;
#[doc = "Field `SPISR3` reader - SPISR3"]
pub type SPISR3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR3"]
    #[inline(always)]
    pub fn spisr3(&self) -> SPISR3_R {
        SPISR3_R::new(self.bits)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SPISR3rs;
impl crate::RegisterSpec for GICD_SPISR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spisr3::R`](R) reader structure"]
impl crate::Readable for GICD_SPISR3rs {}
#[doc = "`reset()` method sets GICD_SPISR3 to value 0"]
impl crate::Resettable for GICD_SPISR3rs {
    const RESET_VALUE: u32 = 0;
}
