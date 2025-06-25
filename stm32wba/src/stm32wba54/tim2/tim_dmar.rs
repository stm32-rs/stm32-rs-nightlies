///Register `TIM_DMAR` reader
pub type R = crate::R<TIM_DMARrs>;
///Register `TIM_DMAR` writer
pub type W = crate::W<TIM_DMARrs>;
///Field `DMAB` reader - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).
pub type DMAB_R = crate::FieldReader<u32>;
///Field `DMAB` writer - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).
pub type DMAB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM_DMAR")
            .field("dmab", &self.dmab())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W<TIM_DMARrs> {
        DMAB_W::new(self, 0)
    }
}
/**TIM DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim_dmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TIM2:TIM_DMAR)*/
pub struct TIM_DMARrs;
impl crate::RegisterSpec for TIM_DMARrs {
    type Ux = u32;
}
///`read()` method returns [`tim_dmar::R`](R) reader structure
impl crate::Readable for TIM_DMARrs {}
///`write(|w| ..)` method takes [`tim_dmar::W`](W) writer structure
impl crate::Writable for TIM_DMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM_DMAR to value 0
impl crate::Resettable for TIM_DMARrs {}
