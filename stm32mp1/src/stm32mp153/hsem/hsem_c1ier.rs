#[doc = "Register `HSEM_C1IER` reader"]
pub type R = crate::R<HSEM_C1IERrs>;
#[doc = "Register `HSEM_C1IER` writer"]
pub type W = crate::W<HSEM_C1IERrs>;
#[doc = "Field `ISE` reader - ISE"]
pub type ISE_R = crate::FieldReader<u32>;
#[doc = "Field `ISE` writer - ISE"]
pub type ISE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISE"]
    #[inline(always)]
    pub fn ise(&self) -> ISE_R {
        ISE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISE"]
    #[inline(always)]
    #[must_use]
    pub fn ise(&mut self) -> ISE_W<HSEM_C1IERrs> {
        ISE_W::new(self, 0)
    }
}
#[doc = "HSEM i1terrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_c1ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_C1IERrs;
impl crate::RegisterSpec for HSEM_C1IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c1ier::R`](R) reader structure"]
impl crate::Readable for HSEM_C1IERrs {}
#[doc = "`write(|w| ..)` method takes [`hsem_c1ier::W`](W) writer structure"]
impl crate::Writable for HSEM_C1IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSEM_C1IER to value 0"]
impl crate::Resettable for HSEM_C1IERrs {
    const RESET_VALUE: u32 = 0;
}
