#[doc = "Register `C1IER` reader"]
pub type R = crate::R<C1IERrs>;
#[doc = "Register `C1IER` writer"]
pub type W = crate::W<C1IERrs>;
#[doc = "Field `ISEm` reader - CPU(n) semaphore m enable bit"]
pub type ISEM_R = crate::FieldReader<u32>;
#[doc = "Field `ISEm` writer - CPU(n) semaphore m enable bit"]
pub type ISEM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU(n) semaphore m enable bit"]
    #[inline(always)]
    pub fn isem(&self) -> ISEM_R {
        ISEM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU(n) semaphore m enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem(&mut self) -> ISEM_W<C1IERrs> {
        ISEM_W::new(self, 0)
    }
}
#[doc = "HSEM Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1IERrs;
impl crate::RegisterSpec for C1IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1ier::R`](R) reader structure"]
impl crate::Readable for C1IERrs {}
#[doc = "`write(|w| ..)` method takes [`c1ier::W`](W) writer structure"]
impl crate::Writable for C1IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1IER to value 0"]
impl crate::Resettable for C1IERrs {
    const RESET_VALUE: u32 = 0;
}
