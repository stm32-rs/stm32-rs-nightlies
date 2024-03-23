#[doc = "Register `HSEM_C1MISR` reader"]
pub type R = crate::R<HSEM_C1MISRrs>;
#[doc = "Field `MISF` reader - MISF"]
pub type MISF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MISF"]
    #[inline(always)]
    pub fn misf(&self) -> MISF_R {
        MISF_R::new(self.bits)
    }
}
#[doc = "HSEM i1terrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_C1MISRrs;
impl crate::RegisterSpec for HSEM_C1MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c1misr::R`](R) reader structure"]
impl crate::Readable for HSEM_C1MISRrs {}
#[doc = "`reset()` method sets HSEM_C1MISR to value 0"]
impl crate::Resettable for HSEM_C1MISRrs {
    const RESET_VALUE: u32 = 0;
}
