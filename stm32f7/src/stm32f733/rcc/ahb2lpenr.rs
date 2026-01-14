///Register `AHB2LPENR` reader
pub type R = crate::R<AHB2LPENRrs>;
///Register `AHB2LPENR` writer
pub type W = crate::W<AHB2LPENRrs>;
/**AES module clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESLPEN {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<AESLPEN> for bool {
    #[inline(always)]
    fn from(variant: AESLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AESLPEN` reader - AES module clock enable during Sleep mode
pub type AESLPEN_R = crate::BitReader<AESLPEN>;
impl AESLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AESLPEN {
        match self.bits {
            false => AESLPEN::DisabledInSleep,
            true => AESLPEN::EnabledInSleep,
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == AESLPEN::DisabledInSleep
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == AESLPEN::EnabledInSleep
    }
}
///Field `AESLPEN` writer - AES module clock enable during Sleep mode
pub type AESLPEN_W<'a, REG> = crate::BitWriter<'a, REG, AESLPEN>;
impl<'a, REG> AESLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(AESLPEN::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(AESLPEN::EnabledInSleep)
    }
}
///Field `RNGLPEN` reader - Random number generator clock enable during Sleep mode
pub use AESLPEN_R as RNGLPEN_R;
///Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode
pub use AESLPEN_R as OTGFSLPEN_R;
///Field `RNGLPEN` writer - Random number generator clock enable during Sleep mode
pub use AESLPEN_W as RNGLPEN_W;
///Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode
pub use AESLPEN_W as OTGFSLPEN_W;
impl R {
    ///Bit 4 - AES module clock enable during Sleep mode
    #[inline(always)]
    pub fn aeslpen(&self) -> AESLPEN_R {
        AESLPEN_R::new(((self.bits >> 4) & 1) != 0)
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
            .field("aeslpen", &self.aeslpen())
            .field("otgfslpen", &self.otgfslpen())
            .field("rnglpen", &self.rnglpen())
            .finish()
    }
}
impl W {
    ///Bit 4 - AES module clock enable during Sleep mode
    #[inline(always)]
    pub fn aeslpen(&mut self) -> AESLPEN_W<'_, AHB2LPENRrs> {
        AESLPEN_W::new(self, 4)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#RCC:AHB2LPENR)*/
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
