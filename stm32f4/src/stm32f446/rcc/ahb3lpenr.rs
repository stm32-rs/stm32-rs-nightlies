#[doc = "Register `AHB3LPENR` reader"]
pub type R = crate::R<AHB3LPENRrs>;
#[doc = "Register `AHB3LPENR` writer"]
pub type W = crate::W<AHB3LPENRrs>;
#[doc = "Flexible memory controller module clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCLPEN {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<FMCLPEN> for bool {
    #[inline(always)]
    fn from(variant: FMCLPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCLPEN` reader - Flexible memory controller module clock enable during Sleep mode"]
pub type FMCLPEN_R = crate::BitReader<FMCLPEN>;
impl FMCLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMCLPEN {
        match self.bits {
            false => FMCLPEN::DisabledInSleep,
            true => FMCLPEN::EnabledInSleep,
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FMCLPEN::DisabledInSleep
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FMCLPEN::EnabledInSleep
    }
}
#[doc = "Field `FMCLPEN` writer - Flexible memory controller module clock enable during Sleep mode"]
pub type FMCLPEN_W<'a, REG> = crate::BitWriter<'a, REG, FMCLPEN>;
impl<'a, REG> FMCLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(FMCLPEN::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(FMCLPEN::EnabledInSleep)
    }
}
#[doc = "Field `QSPILPEN` reader - QUADSPI memory controller module clock enable during Sleep mode"]
pub use FMCLPEN_R as QSPILPEN_R;
#[doc = "Field `QSPILPEN` writer - QUADSPI memory controller module clock enable during Sleep mode"]
pub use FMCLPEN_W as QSPILPEN_W;
impl R {
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QUADSPI memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<AHB3LPENRrs> {
        FMCLPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - QUADSPI memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<AHB3LPENRrs> {
        QSPILPEN_W::new(self, 1)
    }
}
#[doc = "AHB3 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3LPENRrs;
impl crate::RegisterSpec for AHB3LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3lpenr::R`](R) reader structure"]
impl crate::Readable for AHB3LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3lpenr::W`](W) writer structure"]
impl crate::Writable for AHB3LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3LPENR to value 0x01"]
impl crate::Resettable for AHB3LPENRrs {
    const RESET_VALUE: u32 = 0x01;
}
