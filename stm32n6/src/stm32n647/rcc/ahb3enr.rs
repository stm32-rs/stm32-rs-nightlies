///Register `AHB3ENR` reader
pub type R = crate::R<AHB3ENRrs>;
///Register `AHB3ENR` writer
pub type W = crate::W<AHB3ENRrs>;
///Field `RNGEN` reader - RNG enable
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - RNG enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHEN` reader - HASH enable
pub type HASHEN_R = crate::BitReader;
///Field `HASHEN` writer - HASH enable
pub type HASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPEN` reader - CRYP enable
pub type CRYPEN_R = crate::BitReader;
///Field `CRYPEN` writer - CRYP enable
pub type CRYPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESEN` reader - SAES enable
pub type SAESEN_R = crate::BitReader;
///Field `SAESEN` writer - SAES enable
pub type SAESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAEN` reader - PKA enable
pub type PKAEN_R = crate::BitReader;
///Field `PKAEN` writer - PKA enable
pub type PKAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIFSCEN` reader - RIFSC enable
pub type RIFSCEN_R = crate::BitReader;
///Field `RIFSCEN` writer - RIFSC enable
pub type RIFSCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IACEN` reader - IAC enable
pub type IACEN_R = crate::BitReader;
///Field `IACEN` writer - IAC enable
pub type IACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RISAFEN` reader - RISAF enable
pub type RISAFEN_R = crate::BitReader;
///Field `RISAFEN` writer - RISAF enable
pub type RISAFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RNG enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HASH enable
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CRYP enable
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SAES enable
    #[inline(always)]
    pub fn saesen(&self) -> SAESEN_R {
        SAESEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PKA enable
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RIFSC enable
    #[inline(always)]
    pub fn rifscen(&self) -> RIFSCEN_R {
        RIFSCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IAC enable
    #[inline(always)]
    pub fn iacen(&self) -> IACEN_R {
        IACEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - RISAF enable
    #[inline(always)]
    pub fn risafen(&self) -> RISAFEN_R {
        RISAFEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3ENR")
            .field("rngen", &self.rngen())
            .field("hashen", &self.hashen())
            .field("crypen", &self.crypen())
            .field("saesen", &self.saesen())
            .field("pkaen", &self.pkaen())
            .field("rifscen", &self.rifscen())
            .field("iacen", &self.iacen())
            .field("risafen", &self.risafen())
            .finish()
    }
}
impl W {
    ///Bit 0 - RNG enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB3ENRrs> {
        RNGEN_W::new(self, 0)
    }
    ///Bit 1 - HASH enable
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<'_, AHB3ENRrs> {
        HASHEN_W::new(self, 1)
    }
    ///Bit 2 - CRYP enable
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W<'_, AHB3ENRrs> {
        CRYPEN_W::new(self, 2)
    }
    ///Bit 4 - SAES enable
    #[inline(always)]
    pub fn saesen(&mut self) -> SAESEN_W<'_, AHB3ENRrs> {
        SAESEN_W::new(self, 4)
    }
    ///Bit 8 - PKA enable
    #[inline(always)]
    pub fn pkaen(&mut self) -> PKAEN_W<'_, AHB3ENRrs> {
        PKAEN_W::new(self, 8)
    }
    ///Bit 9 - RIFSC enable
    #[inline(always)]
    pub fn rifscen(&mut self) -> RIFSCEN_W<'_, AHB3ENRrs> {
        RIFSCEN_W::new(self, 9)
    }
    ///Bit 10 - IAC enable
    #[inline(always)]
    pub fn iacen(&mut self) -> IACEN_W<'_, AHB3ENRrs> {
        IACEN_W::new(self, 10)
    }
    ///Bit 14 - RISAF enable
    #[inline(always)]
    pub fn risafen(&mut self) -> RISAFEN_W<'_, AHB3ENRrs> {
        RISAFEN_W::new(self, 14)
    }
}
/**RCC AHB3 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3ENR)*/
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3enr::R`](R) reader structure
impl crate::Readable for AHB3ENRrs {}
///`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3ENR to value 0x4600
impl crate::Resettable for AHB3ENRrs {
    const RESET_VALUE: u32 = 0x4600;
}
