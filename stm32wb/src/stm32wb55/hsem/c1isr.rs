#[doc = "Register `C1ISR` reader"]
pub type R = crate::R<C1ISRrs>;
#[doc = "Field `ISFm` reader - CPU(n) semaphore m status bit before enable (mask)"]
pub type ISFM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CPU(n) semaphore m status bit before enable (mask)"]
    #[inline(always)]
    pub fn isfm(&self) -> ISFM_R {
        ISFM_R::new(self.bits)
    }
}
#[doc = "HSEM Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1ISRrs;
impl crate::RegisterSpec for C1ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1isr::R`](R) reader structure"]
impl crate::Readable for C1ISRrs {}
#[doc = "`reset()` method sets C1ISR to value 0"]
impl crate::Resettable for C1ISRrs {
    const RESET_VALUE: u32 = 0;
}
