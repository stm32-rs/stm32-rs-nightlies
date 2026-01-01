///Register `AHB2LPENR` reader
pub type R = crate::R<AHB2LPENRrs>;
///Register `AHB2LPENR` writer
pub type W = crate::W<AHB2LPENRrs>;
///Field `DCMILPEN` reader - Camera interface enable during Sleep mode
pub type DCMILPEN_R = crate::BitReader;
///Field `DCMILPEN` writer - Camera interface enable during Sleep mode
pub type DCMILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPLPEN` reader - Cryptography modules clock enable during Sleep mode
pub type CRYPLPEN_R = crate::BitReader;
///Field `CRYPLPEN` writer - Cryptography modules clock enable during Sleep mode
pub type CRYPLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHLPEN` reader - Hash modules clock enable during Sleep mode
pub type HASHLPEN_R = crate::BitReader;
///Field `HASHLPEN` writer - Hash modules clock enable during Sleep mode
pub type HASHLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGLPEN` reader - Random number generator clock enable during Sleep mode
pub type RNGLPEN_R = crate::BitReader;
///Field `RNGLPEN` writer - Random number generator clock enable during Sleep mode
pub type RNGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode
pub type OTGFSLPEN_R = crate::BitReader;
///Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode
pub type OTGFSLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Camera interface enable during Sleep mode
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Cryptography modules clock enable during Sleep mode
    #[inline(always)]
    pub fn cryplpen(&self) -> CRYPLPEN_R {
        CRYPLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Hash modules clock enable during Sleep mode
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Random number generator clock enable during Sleep mode
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB OTG FS clock enable during Sleep mode
    #[inline(always)]
    pub fn otgfslpen(&self) -> OTGFSLPEN_R {
        OTGFSLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2LPENR")
            .field("otgfslpen", &self.otgfslpen())
            .field("rnglpen", &self.rnglpen())
            .field("hashlpen", &self.hashlpen())
            .field("cryplpen", &self.cryplpen())
            .field("dcmilpen", &self.dcmilpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Camera interface enable during Sleep mode
    #[inline(always)]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<'_, AHB2LPENRrs> {
        DCMILPEN_W::new(self, 0)
    }
    ///Bit 4 - Cryptography modules clock enable during Sleep mode
    #[inline(always)]
    pub fn cryplpen(&mut self) -> CRYPLPEN_W<'_, AHB2LPENRrs> {
        CRYPLPEN_W::new(self, 4)
    }
    ///Bit 5 - Hash modules clock enable during Sleep mode
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<'_, AHB2LPENRrs> {
        HASHLPEN_W::new(self, 5)
    }
    ///Bit 6 - Random number generator clock enable during Sleep mode
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<'_, AHB2LPENRrs> {
        RNGLPEN_W::new(self, 6)
    }
    ///Bit 7 - USB OTG FS clock enable during Sleep mode
    #[inline(always)]
    pub fn otgfslpen(&mut self) -> OTGFSLPEN_W<'_, AHB2LPENRrs> {
        OTGFSLPEN_W::new(self, 7)
    }
}
/**AHB2 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#RCC:AHB2LPENR)*/
pub struct AHB2LPENRrs;
impl crate::RegisterSpec for AHB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2lpenr::R`](R) reader structure
impl crate::Readable for AHB2LPENRrs {}
///`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure
impl crate::Writable for AHB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2LPENR to value 0xf1
impl crate::Resettable for AHB2LPENRrs {
    const RESET_VALUE: u32 = 0xf1;
}
