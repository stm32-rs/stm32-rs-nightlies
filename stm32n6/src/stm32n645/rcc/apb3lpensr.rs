///Register `APB3LPENSR` writer
pub type W = crate::W<APB3LPENSRrs>;
///Field `DFTLPENS` writer - DFT sleep enable
pub type DFTLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB3LPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - DFT sleep enable
    #[inline(always)]
    pub fn dftlpens(&mut self) -> DFTLPENS_W<'_, APB3LPENSRrs> {
        DFTLPENS_W::new(self, 2)
    }
}
/**RCC APB3 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB3LPENSR)*/
pub struct APB3LPENSRrs;
impl crate::RegisterSpec for APB3LPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb3lpensr::W`](W) writer structure
impl crate::Writable for APB3LPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3LPENSR to value 0
impl crate::Resettable for APB3LPENSRrs {}
