#[doc = "Register `DMAR` reader"]
pub type R = crate::R<DMARrs>;
#[doc = "Register `DMAR` writer"]
pub type W = crate::W<DMARrs>;
#[doc = "Field `DMAB` reader - DMA register for burst accesses"]
pub type DMAB_R = crate::FieldReader<u32>;
#[doc = "Field `DMAB` writer - DMA register for burst accesses"]
pub type DMAB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA register for burst accesses"]
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DMAB_W<DMARrs> {
        DMAB_W::new(self, 0)
    }
}
#[doc = "DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARrs;
impl crate::RegisterSpec for DMARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmar::R`](R) reader structure"]
impl crate::Readable for DMARrs {}
#[doc = "`write(|w| ..)` method takes [`dmar::W`](W) writer structure"]
impl crate::Writable for DMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAR to value 0"]
impl crate::Resettable for DMARrs {
    const RESET_VALUE: u32 = 0;
}
