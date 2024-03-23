#[doc = "Register `C2ISR` reader"]
pub type R = crate::R<C2ISRrs>;
#[doc = "Field `ISFm` reader - CPU(2) semaphore m status bit before enable (mask)."]
pub type ISFM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CPU(2) semaphore m status bit before enable (mask)."]
    #[inline(always)]
    pub fn isfm(&self) -> ISFM_R {
        ISFM_R::new(self.bits)
    }
}
#[doc = "HSEM Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2ISRrs;
impl crate::RegisterSpec for C2ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2isr::R`](R) reader structure"]
impl crate::Readable for C2ISRrs {}
#[doc = "`reset()` method sets C2ISR to value 0"]
impl crate::Resettable for C2ISRrs {
    const RESET_VALUE: u32 = 0;
}
