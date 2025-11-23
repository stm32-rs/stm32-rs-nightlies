///Register `APB4HLPENSR` writer
pub type W = crate::W<APB4HLPENSRrs>;
///Field `SYSCFGLPENS` writer - SYSCFG sleep enable
pub type SYSCFGLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSECLPENS` writer - BSEC sleep enable
pub type BSECLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSLPENS` writer - DTS sleep enable
pub type DTSLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPERFMLPENS` writer - BUSPERFM sleep enable
pub type BUSPERFMLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4HLPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - SYSCFG sleep enable
    #[inline(always)]
    pub fn syscfglpens(&mut self) -> SYSCFGLPENS_W<'_, APB4HLPENSRrs> {
        SYSCFGLPENS_W::new(self, 0)
    }
    ///Bit 1 - BSEC sleep enable
    #[inline(always)]
    pub fn bseclpens(&mut self) -> BSECLPENS_W<'_, APB4HLPENSRrs> {
        BSECLPENS_W::new(self, 1)
    }
    ///Bit 2 - DTS sleep enable
    #[inline(always)]
    pub fn dtslpens(&mut self) -> DTSLPENS_W<'_, APB4HLPENSRrs> {
        DTSLPENS_W::new(self, 2)
    }
    ///Bit 4 - BUSPERFM sleep enable
    #[inline(always)]
    pub fn busperfmlpens(&mut self) -> BUSPERFMLPENS_W<'_, APB4HLPENSRrs> {
        BUSPERFMLPENS_W::new(self, 4)
    }
}
/**RCC APB4H Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hlpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB4HLPENSR)*/
pub struct APB4HLPENSRrs;
impl crate::RegisterSpec for APB4HLPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4hlpensr::W`](W) writer structure
impl crate::Writable for APB4HLPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4HLPENSR to value 0
impl crate::Resettable for APB4HLPENSRrs {}
