///Register `TIM16_DMAR` reader
pub type R = crate::R<TIM16_DMARrs>;
///Register `TIM16_DMAR` writer
pub type W = crate::W<TIM16_DMARrs>;
///Field `DMAB` reader - DMA register for burst accesses
pub type DMAB_R = crate::FieldReader<u16>;
///Field `DMAB` writer - DMA register for burst accesses
pub type DMAB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM16_DMAR")
            .field("dmab", &self.dmab())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W<TIM16_DMARrs> {
        DMAB_W::new(self, 0)
    }
}
/**TIM16 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim16_dmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM16:TIM16_DMAR)*/
pub struct TIM16_DMARrs;
impl crate::RegisterSpec for TIM16_DMARrs {
    type Ux = u16;
}
///`read()` method returns [`tim16_dmar::R`](R) reader structure
impl crate::Readable for TIM16_DMARrs {}
///`write(|w| ..)` method takes [`tim16_dmar::W`](W) writer structure
impl crate::Writable for TIM16_DMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM16_DMAR to value 0
impl crate::Resettable for TIM16_DMARrs {
    const RESET_VALUE: u16 = 0;
}
