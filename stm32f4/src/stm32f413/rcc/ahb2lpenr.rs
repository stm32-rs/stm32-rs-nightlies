///Register `AHB2LPENR` reader
pub type R = crate::R<AHB2LPENRrs>;
///Register `AHB2LPENR` writer
pub type W = crate::W<AHB2LPENRrs>;
/**Flexible memory controller module clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSMCLPEN {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<FSMCLPEN> for bool {
    #[inline(always)]
    fn from(variant: FSMCLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FSMCLPEN` reader - Flexible memory controller module clock enable during Sleep mode
pub type FSMCLPEN_R = crate::BitReader<FSMCLPEN>;
impl FSMCLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSMCLPEN {
        match self.bits {
            false => FSMCLPEN::DisabledInSleep,
            true => FSMCLPEN::EnabledInSleep,
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN::DisabledInSleep
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN::EnabledInSleep
    }
}
///Field `FSMCLPEN` writer - Flexible memory controller module clock enable during Sleep mode
pub type FSMCLPEN_W<'a, REG> = crate::BitWriter<'a, REG, FSMCLPEN>;
impl<'a, REG> FSMCLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCLPEN::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCLPEN::EnabledInSleep)
    }
}
///Field `QSPILPEN` reader - QUADSPI memory controller module clock enable during Sleep mode
pub use FSMCLPEN_R as QSPILPEN_R;
///Field `RNGLPEN` reader - RNGLPEN
pub use FSMCLPEN_R as RNGLPEN_R;
///Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode
pub use FSMCLPEN_R as OTGFSLPEN_R;
///Field `QSPILPEN` writer - QUADSPI memory controller module clock enable during Sleep mode
pub use FSMCLPEN_W as QSPILPEN_W;
///Field `RNGLPEN` writer - RNGLPEN
pub use FSMCLPEN_W as RNGLPEN_W;
///Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode
pub use FSMCLPEN_W as OTGFSLPEN_W;
impl R {
    ///Bit 0 - Flexible memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn fsmclpen(&self) -> FSMCLPEN_R {
        FSMCLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - QUADSPI memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 6 - RNGLPEN
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
            .field("fsmclpen", &self.fsmclpen())
            .field("otgfslpen", &self.otgfslpen())
            .field("rnglpen", &self.rnglpen())
            .field("qspilpen", &self.qspilpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn fsmclpen(&mut self) -> FSMCLPEN_W<'_, AHB2LPENRrs> {
        FSMCLPEN_W::new(self, 0)
    }
    ///Bit 1 - QUADSPI memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<'_, AHB2LPENRrs> {
        QSPILPEN_W::new(self, 1)
    }
    ///Bit 6 - RNGLPEN
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#RCC:AHB2LPENR)*/
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
