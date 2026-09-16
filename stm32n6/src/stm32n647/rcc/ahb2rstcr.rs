///Register `AHB2RSTCR` writer
pub type W = crate::W<AHB2RSTCRrs>;
///Field `RAMCFGRSTC` writer - RAMCFG reset
pub type RAMCFGRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1RSTC` writer - MDF1 reset
pub type MDF1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1RSTC` writer - ADF1 reset
pub type ADF1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB2RSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 12 - RAMCFG reset
    #[inline(always)]
    pub fn ramcfgrstc(&mut self) -> RAMCFGRSTC_W<'_, AHB2RSTCRrs> {
        RAMCFGRSTC_W::new(self, 12)
    }
    ///Bit 16 - MDF1 reset
    #[inline(always)]
    pub fn mdf1rstc(&mut self) -> MDF1RSTC_W<'_, AHB2RSTCRrs> {
        MDF1RSTC_W::new(self, 16)
    }
    ///Bit 17 - ADF1 reset
    #[inline(always)]
    pub fn adf1rstc(&mut self) -> ADF1RSTC_W<'_, AHB2RSTCRrs> {
        ADF1RSTC_W::new(self, 17)
    }
}
/**RCC AHB2 Reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2RSTCR)*/
pub struct AHB2RSTCRrs;
impl crate::RegisterSpec for AHB2RSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb2rstcr::W`](W) writer structure
impl crate::Writable for AHB2RSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTCR to value 0
impl crate::Resettable for AHB2RSTCRrs {}
