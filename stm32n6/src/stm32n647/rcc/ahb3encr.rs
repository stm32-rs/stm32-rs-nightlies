///Register `AHB3ENCR` writer
pub type W = crate::W<AHB3ENCRrs>;
///Field `RNGENC` writer - RNG enable
pub type RNGENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHENC` writer - HASH enable
pub type HASHENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPENC` writer - CRYP enable
pub type CRYPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESENC` writer - SAES enable
pub type SAESENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAENC` writer - PKA enable
pub type PKAENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIFSCENC` writer - RIFSC enable
pub type RIFSCENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IACENC` writer - IAC enable
pub type IACENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RISAFENC` writer - RISAF enable
pub type RISAFENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB3ENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - RNG enable
    #[inline(always)]
    pub fn rngenc(&mut self) -> RNGENC_W<'_, AHB3ENCRrs> {
        RNGENC_W::new(self, 0)
    }
    ///Bit 1 - HASH enable
    #[inline(always)]
    pub fn hashenc(&mut self) -> HASHENC_W<'_, AHB3ENCRrs> {
        HASHENC_W::new(self, 1)
    }
    ///Bit 2 - CRYP enable
    #[inline(always)]
    pub fn crypenc(&mut self) -> CRYPENC_W<'_, AHB3ENCRrs> {
        CRYPENC_W::new(self, 2)
    }
    ///Bit 4 - SAES enable
    #[inline(always)]
    pub fn saesenc(&mut self) -> SAESENC_W<'_, AHB3ENCRrs> {
        SAESENC_W::new(self, 4)
    }
    ///Bit 8 - PKA enable
    #[inline(always)]
    pub fn pkaenc(&mut self) -> PKAENC_W<'_, AHB3ENCRrs> {
        PKAENC_W::new(self, 8)
    }
    ///Bit 9 - RIFSC enable
    #[inline(always)]
    pub fn rifscenc(&mut self) -> RIFSCENC_W<'_, AHB3ENCRrs> {
        RIFSCENC_W::new(self, 9)
    }
    ///Bit 10 - IAC enable
    #[inline(always)]
    pub fn iacenc(&mut self) -> IACENC_W<'_, AHB3ENCRrs> {
        IACENC_W::new(self, 10)
    }
    ///Bit 14 - RISAF enable
    #[inline(always)]
    pub fn risafenc(&mut self) -> RISAFENC_W<'_, AHB3ENCRrs> {
        RISAFENC_W::new(self, 14)
    }
}
/**RCC AHB3 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3encr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3ENCR)*/
pub struct AHB3ENCRrs;
impl crate::RegisterSpec for AHB3ENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb3encr::W`](W) writer structure
impl crate::Writable for AHB3ENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3ENCR to value 0
impl crate::Resettable for AHB3ENCRrs {}
