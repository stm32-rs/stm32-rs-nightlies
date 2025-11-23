///Register `APB1HLPENSR` writer
pub type W = crate::W<APB1HLPENSRrs>;
///Field `MDIOSLPENS` writer - MDIOS sleep enable
pub type MDIOSLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANLPENS` writer - FDCAN sleep enable
pub type FDCANLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1LPENS` writer - UCPD1 sleep enable
pub type UCPD1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1HLPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 5 - MDIOS sleep enable
    #[inline(always)]
    pub fn mdioslpens(&mut self) -> MDIOSLPENS_W<'_, APB1HLPENSRrs> {
        MDIOSLPENS_W::new(self, 5)
    }
    ///Bit 8 - FDCAN sleep enable
    #[inline(always)]
    pub fn fdcanlpens(&mut self) -> FDCANLPENS_W<'_, APB1HLPENSRrs> {
        FDCANLPENS_W::new(self, 8)
    }
    ///Bit 18 - UCPD1 sleep enable
    #[inline(always)]
    pub fn ucpd1lpens(&mut self) -> UCPD1LPENS_W<'_, APB1HLPENSRrs> {
        UCPD1LPENS_W::new(self, 18)
    }
}
/**RCC APB1H Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:APB1HLPENSR)*/
pub struct APB1HLPENSRrs;
impl crate::RegisterSpec for APB1HLPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1hlpensr::W`](W) writer structure
impl crate::Writable for APB1HLPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HLPENSR to value 0
impl crate::Resettable for APB1HLPENSRrs {}
