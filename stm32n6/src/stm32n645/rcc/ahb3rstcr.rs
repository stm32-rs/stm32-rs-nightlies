///Register `AHB3RSTCR` writer
pub type W = crate::W<AHB3RSTCRrs>;
///Field `RNGRSTC` writer - RNG reset
pub type RNGRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHRSTC` writer - HASH reset
pub type HASHRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPRSTC` writer - CRYP reset
pub type CRYPRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESRSTC` writer - SAES reset
pub type SAESRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKARSTC` writer - PKA reset
pub type PKARSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IACRSTC` writer - IAC reset
pub type IACRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB3RSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - RNG reset
    #[inline(always)]
    pub fn rngrstc(&mut self) -> RNGRSTC_W<'_, AHB3RSTCRrs> {
        RNGRSTC_W::new(self, 0)
    }
    ///Bit 1 - HASH reset
    #[inline(always)]
    pub fn hashrstc(&mut self) -> HASHRSTC_W<'_, AHB3RSTCRrs> {
        HASHRSTC_W::new(self, 1)
    }
    ///Bit 2 - CRYP reset
    #[inline(always)]
    pub fn cryprstc(&mut self) -> CRYPRSTC_W<'_, AHB3RSTCRrs> {
        CRYPRSTC_W::new(self, 2)
    }
    ///Bit 4 - SAES reset
    #[inline(always)]
    pub fn saesrstc(&mut self) -> SAESRSTC_W<'_, AHB3RSTCRrs> {
        SAESRSTC_W::new(self, 4)
    }
    ///Bit 8 - PKA reset
    #[inline(always)]
    pub fn pkarstc(&mut self) -> PKARSTC_W<'_, AHB3RSTCRrs> {
        PKARSTC_W::new(self, 8)
    }
    ///Bit 10 - IAC reset
    #[inline(always)]
    pub fn iacrstc(&mut self) -> IACRSTC_W<'_, AHB3RSTCRrs> {
        IACRSTC_W::new(self, 10)
    }
}
/**RCC AHB3 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:AHB3RSTCR)*/
pub struct AHB3RSTCRrs;
impl crate::RegisterSpec for AHB3RSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb3rstcr::W`](W) writer structure
impl crate::Writable for AHB3RSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTCR to value 0
impl crate::Resettable for AHB3RSTCRrs {}
