///Register `AHB3LPENR` reader
pub type R = crate::R<AHB3LPENRrs>;
///Register `AHB3LPENR` writer
pub type W = crate::W<AHB3LPENRrs>;
///Field `RNGLPEN` reader - RNG sleep enable
pub type RNGLPEN_R = crate::BitReader;
///Field `RNGLPEN` writer - RNG sleep enable
pub type RNGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHLPEN` reader - HASH sleep enable
pub type HASHLPEN_R = crate::BitReader;
///Field `HASHLPEN` writer - HASH sleep enable
pub type HASHLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPLPEN` reader - CRYP sleep enable
pub type CRYPLPEN_R = crate::BitReader;
///Field `CRYPLPEN` writer - CRYP sleep enable
pub type CRYPLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESLPEN` reader - SAES sleep enable
pub type SAESLPEN_R = crate::BitReader;
///Field `SAESLPEN` writer - SAES sleep enable
pub type SAESLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKALPEN` reader - PKA sleep enable
pub type PKALPEN_R = crate::BitReader;
///Field `PKALPEN` writer - PKA sleep enable
pub type PKALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIFSCLPEN` reader - RIFSC sleep enable
pub type RIFSCLPEN_R = crate::BitReader;
///Field `RIFSCLPEN` writer - RIFSC sleep enable
pub type RIFSCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IACLPEN` reader - IAC sleep enable
pub type IACLPEN_R = crate::BitReader;
///Field `IACLPEN` writer - IAC sleep enable
pub type IACLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RISAFLPEN` reader - RISAF sleep enable
pub type RISAFLPEN_R = crate::BitReader;
///Field `RISAFLPEN` writer - RISAF sleep enable
pub type RISAFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RNG sleep enable
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HASH sleep enable
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CRYP sleep enable
    #[inline(always)]
    pub fn cryplpen(&self) -> CRYPLPEN_R {
        CRYPLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SAES sleep enable
    #[inline(always)]
    pub fn saeslpen(&self) -> SAESLPEN_R {
        SAESLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PKA sleep enable
    #[inline(always)]
    pub fn pkalpen(&self) -> PKALPEN_R {
        PKALPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RIFSC sleep enable
    #[inline(always)]
    pub fn rifsclpen(&self) -> RIFSCLPEN_R {
        RIFSCLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IAC sleep enable
    #[inline(always)]
    pub fn iaclpen(&self) -> IACLPEN_R {
        IACLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - RISAF sleep enable
    #[inline(always)]
    pub fn risaflpen(&self) -> RISAFLPEN_R {
        RISAFLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3LPENR")
            .field("rnglpen", &self.rnglpen())
            .field("hashlpen", &self.hashlpen())
            .field("cryplpen", &self.cryplpen())
            .field("saeslpen", &self.saeslpen())
            .field("pkalpen", &self.pkalpen())
            .field("rifsclpen", &self.rifsclpen())
            .field("iaclpen", &self.iaclpen())
            .field("risaflpen", &self.risaflpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - RNG sleep enable
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<'_, AHB3LPENRrs> {
        RNGLPEN_W::new(self, 0)
    }
    ///Bit 1 - HASH sleep enable
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<'_, AHB3LPENRrs> {
        HASHLPEN_W::new(self, 1)
    }
    ///Bit 2 - CRYP sleep enable
    #[inline(always)]
    pub fn cryplpen(&mut self) -> CRYPLPEN_W<'_, AHB3LPENRrs> {
        CRYPLPEN_W::new(self, 2)
    }
    ///Bit 4 - SAES sleep enable
    #[inline(always)]
    pub fn saeslpen(&mut self) -> SAESLPEN_W<'_, AHB3LPENRrs> {
        SAESLPEN_W::new(self, 4)
    }
    ///Bit 8 - PKA sleep enable
    #[inline(always)]
    pub fn pkalpen(&mut self) -> PKALPEN_W<'_, AHB3LPENRrs> {
        PKALPEN_W::new(self, 8)
    }
    ///Bit 9 - RIFSC sleep enable
    #[inline(always)]
    pub fn rifsclpen(&mut self) -> RIFSCLPEN_W<'_, AHB3LPENRrs> {
        RIFSCLPEN_W::new(self, 9)
    }
    ///Bit 10 - IAC sleep enable
    #[inline(always)]
    pub fn iaclpen(&mut self) -> IACLPEN_W<'_, AHB3LPENRrs> {
        IACLPEN_W::new(self, 10)
    }
    ///Bit 14 - RISAF sleep enable
    #[inline(always)]
    pub fn risaflpen(&mut self) -> RISAFLPEN_W<'_, AHB3LPENRrs> {
        RISAFLPEN_W::new(self, 14)
    }
}
/**RCC AHB3 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:AHB3LPENR)*/
pub struct AHB3LPENRrs;
impl crate::RegisterSpec for AHB3LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3lpenr::R`](R) reader structure
impl crate::Readable for AHB3LPENRrs {}
///`write(|w| ..)` method takes [`ahb3lpenr::W`](W) writer structure
impl crate::Writable for AHB3LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3LPENR to value 0x0400
impl crate::Resettable for AHB3LPENRrs {
    const RESET_VALUE: u32 = 0x0400;
}
