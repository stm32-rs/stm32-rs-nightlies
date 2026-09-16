///Register `AHB3LPENSR` writer
pub type W = crate::W<AHB3LPENSRrs>;
///Field `RNGLPENS` writer - RNG sleep enable
pub type RNGLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHLPENS` writer - HASH sleep enable
pub type HASHLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPLPENS` writer - CRYP sleep enable
pub type CRYPLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESLPENS` writer - SAES sleep enable
pub type SAESLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKALPENS` writer - PKA sleep enable
pub type PKALPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIFSCLPENS` writer - RIFSC sleep enable
pub type RIFSCLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IACLPENS` writer - IAC sleep enable
pub type IACLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RISAFLPENS` writer - RISAF sleep enable
pub type RISAFLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB3LPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - RNG sleep enable
    #[inline(always)]
    pub fn rnglpens(&mut self) -> RNGLPENS_W<'_, AHB3LPENSRrs> {
        RNGLPENS_W::new(self, 0)
    }
    ///Bit 1 - HASH sleep enable
    #[inline(always)]
    pub fn hashlpens(&mut self) -> HASHLPENS_W<'_, AHB3LPENSRrs> {
        HASHLPENS_W::new(self, 1)
    }
    ///Bit 2 - CRYP sleep enable
    #[inline(always)]
    pub fn cryplpens(&mut self) -> CRYPLPENS_W<'_, AHB3LPENSRrs> {
        CRYPLPENS_W::new(self, 2)
    }
    ///Bit 4 - SAES sleep enable
    #[inline(always)]
    pub fn saeslpens(&mut self) -> SAESLPENS_W<'_, AHB3LPENSRrs> {
        SAESLPENS_W::new(self, 4)
    }
    ///Bit 8 - PKA sleep enable
    #[inline(always)]
    pub fn pkalpens(&mut self) -> PKALPENS_W<'_, AHB3LPENSRrs> {
        PKALPENS_W::new(self, 8)
    }
    ///Bit 9 - RIFSC sleep enable
    #[inline(always)]
    pub fn rifsclpens(&mut self) -> RIFSCLPENS_W<'_, AHB3LPENSRrs> {
        RIFSCLPENS_W::new(self, 9)
    }
    ///Bit 10 - IAC sleep enable
    #[inline(always)]
    pub fn iaclpens(&mut self) -> IACLPENS_W<'_, AHB3LPENSRrs> {
        IACLPENS_W::new(self, 10)
    }
    ///Bit 14 - RISAF sleep enable
    #[inline(always)]
    pub fn risaflpens(&mut self) -> RISAFLPENS_W<'_, AHB3LPENSRrs> {
        RISAFLPENS_W::new(self, 14)
    }
}
/**RCC AHB3 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:AHB3LPENSR)*/
pub struct AHB3LPENSRrs;
impl crate::RegisterSpec for AHB3LPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb3lpensr::W`](W) writer structure
impl crate::Writable for AHB3LPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3LPENSR to value 0
impl crate::Resettable for AHB3LPENSRrs {}
