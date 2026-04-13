///Register `AHB2RSTSR` writer
pub type W = crate::W<AHB2RSTSRrs>;
///Field `RAMCFGRSTS` writer - RAMCFG reset
pub type RAMCFGRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1RSTS` writer - MDF1 reset
pub type MDF1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1RSTS` writer - ADF1 reset
pub type ADF1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB2RSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 12 - RAMCFG reset
    #[inline(always)]
    pub fn ramcfgrsts(&mut self) -> RAMCFGRSTS_W<'_, AHB2RSTSRrs> {
        RAMCFGRSTS_W::new(self, 12)
    }
    ///Bit 16 - MDF1 reset
    #[inline(always)]
    pub fn mdf1rsts(&mut self) -> MDF1RSTS_W<'_, AHB2RSTSRrs> {
        MDF1RSTS_W::new(self, 16)
    }
    ///Bit 17 - ADF1 reset
    #[inline(always)]
    pub fn adf1rsts(&mut self) -> ADF1RSTS_W<'_, AHB2RSTSRrs> {
        ADF1RSTS_W::new(self, 17)
    }
}
/**RCC AHB2 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:AHB2RSTSR)*/
pub struct AHB2RSTSRrs;
impl crate::RegisterSpec for AHB2RSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb2rstsr::W`](W) writer structure
impl crate::Writable for AHB2RSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTSR to value 0
impl crate::Resettable for AHB2RSTSRrs {}
