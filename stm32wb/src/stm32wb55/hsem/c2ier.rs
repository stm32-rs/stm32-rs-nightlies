#[doc = "Register `C2IER` reader"]
pub type R = crate::R<C2IERrs>;
#[doc = "Register `C2IER` writer"]
pub type W = crate::W<C2IERrs>;
#[doc = "Field `ISEm` reader - CPU(2) semaphore m enable bit."]
pub type ISEM_R = crate::FieldReader<u32>;
#[doc = "Field `ISEm` writer - CPU(2) semaphore m enable bit."]
pub type ISEM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU(2) semaphore m enable bit."]
    #[inline(always)]
    pub fn isem(&self) -> ISEM_R {
        ISEM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU(2) semaphore m enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn isem(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 0)
    }
}
#[doc = "HSEM Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2IERrs;
impl crate::RegisterSpec for C2IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ier::R`](R) reader structure"]
impl crate::Readable for C2IERrs {}
#[doc = "`write(|w| ..)` method takes [`c2ier::W`](W) writer structure"]
impl crate::Writable for C2IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2IER to value 0"]
impl crate::Resettable for C2IERrs {
    const RESET_VALUE: u32 = 0;
}
