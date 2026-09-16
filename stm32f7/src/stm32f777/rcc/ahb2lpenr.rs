///Register `AHB2LPENR` reader
pub type R = crate::R<AHB2LPENRrs>;
///Register `AHB2LPENR` writer
pub type W = crate::W<AHB2LPENRrs>;
/**Camera interface enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMILPEN {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<DCMILPEN> for bool {
    #[inline(always)]
    fn from(variant: DCMILPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMILPEN` reader - Camera interface enable during Sleep mode
pub type DCMILPEN_R = crate::BitReader<DCMILPEN>;
impl DCMILPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMILPEN {
        match self.bits {
            false => DCMILPEN::DisabledInSleep,
            true => DCMILPEN::EnabledInSleep,
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == DCMILPEN::DisabledInSleep
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == DCMILPEN::EnabledInSleep
    }
}
///Field `DCMILPEN` writer - Camera interface enable during Sleep mode
pub type DCMILPEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMILPEN>;
impl<'a, REG> DCMILPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(DCMILPEN::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(DCMILPEN::EnabledInSleep)
    }
}
///Field `JPEGLPEN` reader - JPEG module enabled during Sleep mode
pub use DCMILPEN_R as JPEGLPEN_R;
///Field `CRYPLPEN` reader - Cryptography modules clock enable during Sleep mode
pub use DCMILPEN_R as CRYPLPEN_R;
///Field `HASHLPEN` reader - Hash modules clock enable during Sleep mode
pub use DCMILPEN_R as HASHLPEN_R;
///Field `RNGLPEN` reader - Random number generator clock enable during Sleep mode
pub use DCMILPEN_R as RNGLPEN_R;
///Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode
pub use DCMILPEN_R as OTGFSLPEN_R;
///Field `JPEGLPEN` writer - JPEG module enabled during Sleep mode
pub use DCMILPEN_W as JPEGLPEN_W;
///Field `CRYPLPEN` writer - Cryptography modules clock enable during Sleep mode
pub use DCMILPEN_W as CRYPLPEN_W;
///Field `HASHLPEN` writer - Hash modules clock enable during Sleep mode
pub use DCMILPEN_W as HASHLPEN_W;
///Field `RNGLPEN` writer - Random number generator clock enable during Sleep mode
pub use DCMILPEN_W as RNGLPEN_W;
///Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode
pub use DCMILPEN_W as OTGFSLPEN_W;
impl R {
    ///Bit 0 - Camera interface enable during Sleep mode
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - JPEG module enabled during Sleep mode
    #[inline(always)]
    pub fn jpeglpen(&self) -> JPEGLPEN_R {
        JPEGLPEN_R::new(((self.bits >> 1) & 1) != 0)
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
            .field("dcmilpen", &self.dcmilpen())
            .field("otgfslpen", &self.otgfslpen())
            .field("rnglpen", &self.rnglpen())
            .field("hashlpen", &self.hashlpen())
            .field("cryplpen", &self.cryplpen())
            .field("jpeglpen", &self.jpeglpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Camera interface enable during Sleep mode
    #[inline(always)]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<'_, AHB2LPENRrs> {
        DCMILPEN_W::new(self, 0)
    }
    ///Bit 1 - JPEG module enabled during Sleep mode
    #[inline(always)]
    pub fn jpeglpen(&mut self) -> JPEGLPEN_W<'_, AHB2LPENRrs> {
        JPEGLPEN_W::new(self, 1)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#RCC:AHB2LPENR)*/
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
