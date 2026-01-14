///Register `AHB3LPENCR` writer
pub type W = crate::W<AHB3LPENCRrs>;
///Field `RNGLPENC` writer - RNG sleep enable
pub type RNGLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHLPENC` writer - HASH sleep enable
pub type HASHLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPLPENC` writer - CRYP sleep enable
pub type CRYPLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESLPENC` writer - SAES sleep enable
pub type SAESLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKALPENC` writer - PKA sleep enable
pub type PKALPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIFSCLPENC` writer - RIFSC sleep enable
pub type RIFSCLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IACLPENC` writer - IAC sleep enable
pub type IACLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RISAFLPENC` writer - RISAF sleep enable
pub type RISAFLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB3LPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - RNG sleep enable
    #[inline(always)]
    pub fn rnglpenc(&mut self) -> RNGLPENC_W<'_, AHB3LPENCRrs> {
        RNGLPENC_W::new(self, 0)
    }
    ///Bit 1 - HASH sleep enable
    #[inline(always)]
    pub fn hashlpenc(&mut self) -> HASHLPENC_W<'_, AHB3LPENCRrs> {
        HASHLPENC_W::new(self, 1)
    }
    ///Bit 2 - CRYP sleep enable
    #[inline(always)]
    pub fn cryplpenc(&mut self) -> CRYPLPENC_W<'_, AHB3LPENCRrs> {
        CRYPLPENC_W::new(self, 2)
    }
    ///Bit 4 - SAES sleep enable
    #[inline(always)]
    pub fn saeslpenc(&mut self) -> SAESLPENC_W<'_, AHB3LPENCRrs> {
        SAESLPENC_W::new(self, 4)
    }
    ///Bit 8 - PKA sleep enable
    #[inline(always)]
    pub fn pkalpenc(&mut self) -> PKALPENC_W<'_, AHB3LPENCRrs> {
        PKALPENC_W::new(self, 8)
    }
    ///Bit 9 - RIFSC sleep enable
    #[inline(always)]
    pub fn rifsclpenc(&mut self) -> RIFSCLPENC_W<'_, AHB3LPENCRrs> {
        RIFSCLPENC_W::new(self, 9)
    }
    ///Bit 10 - IAC sleep enable
    #[inline(always)]
    pub fn iaclpenc(&mut self) -> IACLPENC_W<'_, AHB3LPENCRrs> {
        IACLPENC_W::new(self, 10)
    }
    ///Bit 14 - RISAF sleep enable
    #[inline(always)]
    pub fn risaflpenc(&mut self) -> RISAFLPENC_W<'_, AHB3LPENCRrs> {
        RISAFLPENC_W::new(self, 14)
    }
}
/**RCC AHB3 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:AHB3LPENCR)*/
pub struct AHB3LPENCRrs;
impl crate::RegisterSpec for AHB3LPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb3lpencr::W`](W) writer structure
impl crate::Writable for AHB3LPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3LPENCR to value 0
impl crate::Resettable for AHB3LPENCRrs {}
