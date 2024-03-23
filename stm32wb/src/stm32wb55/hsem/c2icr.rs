#[doc = "Register `C2ICR` reader"]
pub type R = crate::R<C2ICRrs>;
#[doc = "Register `C2ICR` writer"]
pub type W = crate::W<C2ICRrs>;
#[doc = "Field `ISCm` reader - CPU(2) semaphore m clear bit"]
pub type ISCM_R = crate::FieldReader<u32>;
#[doc = "Field `ISCm` writer - CPU(2) semaphore m clear bit"]
pub type ISCM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU(2) semaphore m clear bit"]
    #[inline(always)]
    pub fn iscm(&self) -> ISCM_R {
        ISCM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU(2) semaphore m clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn iscm(&mut self) -> ISCM_W<C2ICRrs> {
        ISCM_W::new(self, 0)
    }
}
#[doc = "HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2ICRrs;
impl crate::RegisterSpec for C2ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2icr::R`](R) reader structure"]
impl crate::Readable for C2ICRrs {}
#[doc = "`write(|w| ..)` method takes [`c2icr::W`](W) writer structure"]
impl crate::Writable for C2ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2ICR to value 0"]
impl crate::Resettable for C2ICRrs {
    const RESET_VALUE: u32 = 0;
}
