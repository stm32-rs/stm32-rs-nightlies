///Register `AHB3RSTR` reader
pub type R = crate::R<AHB3RSTRrs>;
///Register `AHB3RSTR` writer
pub type W = crate::W<AHB3RSTRrs>;
///Field `RNGRST` reader - RNG reset
pub type RNGRST_R = crate::BitReader;
///Field `RNGRST` writer - RNG reset
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHRST` reader - HASH reset
pub type HASHRST_R = crate::BitReader;
///Field `HASHRST` writer - HASH reset
pub type HASHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPRST` reader - CRYP reset
pub type CRYPRST_R = crate::BitReader;
///Field `CRYPRST` writer - CRYP reset
pub type CRYPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESRST` reader - SAES reset
pub type SAESRST_R = crate::BitReader;
///Field `SAESRST` writer - SAES reset
pub type SAESRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKARST` reader - PKA reset
pub type PKARST_R = crate::BitReader;
///Field `PKARST` writer - PKA reset
pub type PKARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IACRST` reader - IAC reset
pub type IACRST_R = crate::BitReader;
///Field `IACRST` writer - IAC reset
pub type IACRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RNG reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HASH reset
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CRYP reset
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SAES reset
    #[inline(always)]
    pub fn saesrst(&self) -> SAESRST_R {
        SAESRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PKA reset
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - IAC reset
    #[inline(always)]
    pub fn iacrst(&self) -> IACRST_R {
        IACRST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3RSTR")
            .field("rngrst", &self.rngrst())
            .field("hashrst", &self.hashrst())
            .field("cryprst", &self.cryprst())
            .field("saesrst", &self.saesrst())
            .field("pkarst", &self.pkarst())
            .field("iacrst", &self.iacrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - RNG reset
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB3RSTRrs> {
        RNGRST_W::new(self, 0)
    }
    ///Bit 1 - HASH reset
    #[inline(always)]
    pub fn hashrst(&mut self) -> HASHRST_W<'_, AHB3RSTRrs> {
        HASHRST_W::new(self, 1)
    }
    ///Bit 2 - CRYP reset
    #[inline(always)]
    pub fn cryprst(&mut self) -> CRYPRST_W<'_, AHB3RSTRrs> {
        CRYPRST_W::new(self, 2)
    }
    ///Bit 4 - SAES reset
    #[inline(always)]
    pub fn saesrst(&mut self) -> SAESRST_W<'_, AHB3RSTRrs> {
        SAESRST_W::new(self, 4)
    }
    ///Bit 8 - PKA reset
    #[inline(always)]
    pub fn pkarst(&mut self) -> PKARST_W<'_, AHB3RSTRrs> {
        PKARST_W::new(self, 8)
    }
    ///Bit 10 - IAC reset
    #[inline(always)]
    pub fn iacrst(&mut self) -> IACRST_W<'_, AHB3RSTRrs> {
        IACRST_W::new(self, 10)
    }
}
/**RCC AHB3 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3RSTR)*/
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3rstr::R`](R) reader structure
impl crate::Readable for AHB3RSTRrs {}
///`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTRrs {}
