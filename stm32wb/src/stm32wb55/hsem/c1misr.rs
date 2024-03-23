#[doc = "Register `C1MISR` reader"]
pub type R = crate::R<C1MISRrs>;
#[doc = "Field `MISFm` reader - masked CPU(n) semaphore m status bit after enable (mask)."]
pub type MISFM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - masked CPU(n) semaphore m status bit after enable (mask)."]
    #[inline(always)]
    pub fn misfm(&self) -> MISFM_R {
        MISFM_R::new(self.bits)
    }
}
#[doc = "HSEM Masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1MISRrs;
impl crate::RegisterSpec for C1MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1misr::R`](R) reader structure"]
impl crate::Readable for C1MISRrs {}
#[doc = "`reset()` method sets C1MISR to value 0"]
impl crate::Resettable for C1MISRrs {
    const RESET_VALUE: u32 = 0;
}
