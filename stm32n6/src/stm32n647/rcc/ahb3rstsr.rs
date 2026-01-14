///Register `AHB3RSTSR` writer
pub type W = crate::W<AHB3RSTSRrs>;
///Field `RNGRSTS` writer - RNG reset
pub type RNGRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHRSTS` writer - HASH reset
pub type HASHRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPRSTS` writer - CRYP reset
pub type CRYPRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESRSTS` writer - SAES reset
pub type SAESRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKARSTS` writer - PKA reset
pub type PKARSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IACRSTS` writer - IAC reset
pub type IACRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB3RSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - RNG reset
    #[inline(always)]
    pub fn rngrsts(&mut self) -> RNGRSTS_W<'_, AHB3RSTSRrs> {
        RNGRSTS_W::new(self, 0)
    }
    ///Bit 1 - HASH reset
    #[inline(always)]
    pub fn hashrsts(&mut self) -> HASHRSTS_W<'_, AHB3RSTSRrs> {
        HASHRSTS_W::new(self, 1)
    }
    ///Bit 2 - CRYP reset
    #[inline(always)]
    pub fn cryprsts(&mut self) -> CRYPRSTS_W<'_, AHB3RSTSRrs> {
        CRYPRSTS_W::new(self, 2)
    }
    ///Bit 4 - SAES reset
    #[inline(always)]
    pub fn saesrsts(&mut self) -> SAESRSTS_W<'_, AHB3RSTSRrs> {
        SAESRSTS_W::new(self, 4)
    }
    ///Bit 8 - PKA reset
    #[inline(always)]
    pub fn pkarsts(&mut self) -> PKARSTS_W<'_, AHB3RSTSRrs> {
        PKARSTS_W::new(self, 8)
    }
    ///Bit 10 - IAC reset
    #[inline(always)]
    pub fn iacrsts(&mut self) -> IACRSTS_W<'_, AHB3RSTSRrs> {
        IACRSTS_W::new(self, 10)
    }
}
/**RCC AHB3 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3RSTSR)*/
pub struct AHB3RSTSRrs;
impl crate::RegisterSpec for AHB3RSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb3rstsr::W`](W) writer structure
impl crate::Writable for AHB3RSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTSR to value 0
impl crate::Resettable for AHB3RSTSRrs {}
