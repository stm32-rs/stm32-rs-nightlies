///Register `AHB3ENSR` writer
pub type W = crate::W<AHB3ENSRrs>;
///Field `RNGENS` writer - RNG enable
pub type RNGENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHENS` writer - HASH enable
pub type HASHENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPENS` writer - CRYP enable
pub type CRYPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESENS` writer - SAES enable
pub type SAESENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAENS` writer - PKA enable
pub type PKAENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIFSCENS` writer - RIFSC enable
pub type RIFSCENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IACENS` writer - IAC enable
pub type IACENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RISAFENS` writer - RISAF enable
pub type RISAFENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB3ENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - RNG enable
    #[inline(always)]
    pub fn rngens(&mut self) -> RNGENS_W<'_, AHB3ENSRrs> {
        RNGENS_W::new(self, 0)
    }
    ///Bit 1 - HASH enable
    #[inline(always)]
    pub fn hashens(&mut self) -> HASHENS_W<'_, AHB3ENSRrs> {
        HASHENS_W::new(self, 1)
    }
    ///Bit 2 - CRYP enable
    #[inline(always)]
    pub fn crypens(&mut self) -> CRYPENS_W<'_, AHB3ENSRrs> {
        CRYPENS_W::new(self, 2)
    }
    ///Bit 4 - SAES enable
    #[inline(always)]
    pub fn saesens(&mut self) -> SAESENS_W<'_, AHB3ENSRrs> {
        SAESENS_W::new(self, 4)
    }
    ///Bit 8 - PKA enable
    #[inline(always)]
    pub fn pkaens(&mut self) -> PKAENS_W<'_, AHB3ENSRrs> {
        PKAENS_W::new(self, 8)
    }
    ///Bit 9 - RIFSC enable
    #[inline(always)]
    pub fn rifscens(&mut self) -> RIFSCENS_W<'_, AHB3ENSRrs> {
        RIFSCENS_W::new(self, 9)
    }
    ///Bit 10 - IAC enable
    #[inline(always)]
    pub fn iacens(&mut self) -> IACENS_W<'_, AHB3ENSRrs> {
        IACENS_W::new(self, 10)
    }
    ///Bit 14 - RISAF enable
    #[inline(always)]
    pub fn risafens(&mut self) -> RISAFENS_W<'_, AHB3ENSRrs> {
        RISAFENS_W::new(self, 14)
    }
}
/**RCC AHB3 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3ensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3ENSR)*/
pub struct AHB3ENSRrs;
impl crate::RegisterSpec for AHB3ENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb3ensr::W`](W) writer structure
impl crate::Writable for AHB3ENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3ENSR to value 0
impl crate::Resettable for AHB3ENSRrs {}
