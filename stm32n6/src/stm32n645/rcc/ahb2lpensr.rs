///Register `AHB2LPENSR` writer
pub type W = crate::W<AHB2LPENSRrs>;
///Field `RAMCFGLPENS` writer - RAMCFG sleep enable
pub type RAMCFGLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1LPENS` writer - MDF1 sleep enable
pub type MDF1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1LPENS` writer - ADF1 sleep enable
pub type ADF1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB2LPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 12 - RAMCFG sleep enable
    #[inline(always)]
    pub fn ramcfglpens(&mut self) -> RAMCFGLPENS_W<'_, AHB2LPENSRrs> {
        RAMCFGLPENS_W::new(self, 12)
    }
    ///Bit 16 - MDF1 sleep enable
    #[inline(always)]
    pub fn mdf1lpens(&mut self) -> MDF1LPENS_W<'_, AHB2LPENSRrs> {
        MDF1LPENS_W::new(self, 16)
    }
    ///Bit 17 - ADF1 sleep enable
    #[inline(always)]
    pub fn adf1lpens(&mut self) -> ADF1LPENS_W<'_, AHB2LPENSRrs> {
        ADF1LPENS_W::new(self, 17)
    }
}
/**RCC AHB2 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:AHB2LPENSR)*/
pub struct AHB2LPENSRrs;
impl crate::RegisterSpec for AHB2LPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb2lpensr::W`](W) writer structure
impl crate::Writable for AHB2LPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2LPENSR to value 0
impl crate::Resettable for AHB2LPENSRrs {}
