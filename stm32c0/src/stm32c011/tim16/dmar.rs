#[doc = "Register `DMAR` reader"]
pub type R = crate::R<DMARrs>;
#[doc = "Register `DMAR` writer"]
pub type W = crate::W<DMARrs>;
#[doc = "Field `DMAB` reader - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
pub type DMAB_R = crate::FieldReader<u16>;
#[doc = "Field `DMAB` writer - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
pub type DMAB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DMAB_W<DMARrs> {
        DMAB_W::new(self, 0)
    }
}
#[doc = "TIM16 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARrs;
impl crate::RegisterSpec for DMARrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmar::R`](R) reader structure"]
impl crate::Readable for DMARrs {}
#[doc = "`write(|w| ..)` method takes [`dmar::W`](W) writer structure"]
impl crate::Writable for DMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DMAR to value 0"]
impl crate::Resettable for DMARrs {
    const RESET_VALUE: u16 = 0;
}
