#[doc = "Register `AHB3LPENR` reader"]
pub type R = crate::R<AHB3LPENRrs>;
#[doc = "Register `AHB3LPENR` writer"]
pub type W = crate::W<AHB3LPENRrs>;
#[doc = "Flexible static memory controller module clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSMCLPEN {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<FSMCLPEN> for bool {
    #[inline(always)]
    fn from(variant: FSMCLPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSMCLPEN` reader - Flexible static memory controller module clock enable during Sleep mode"]
pub type FSMCLPEN_R = crate::BitReader<FSMCLPEN>;
impl FSMCLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSMCLPEN {
        match self.bits {
            false => FSMCLPEN::DisabledInSleep,
            true => FSMCLPEN::EnabledInSleep,
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN::DisabledInSleep
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN::EnabledInSleep
    }
}
#[doc = "Field `FSMCLPEN` writer - Flexible static memory controller module clock enable during Sleep mode"]
pub type FSMCLPEN_W<'a, REG> = crate::BitWriter<'a, REG, FSMCLPEN>;
impl<'a, REG> FSMCLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCLPEN::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCLPEN::EnabledInSleep)
    }
}
impl R {
    #[doc = "Bit 0 - Flexible static memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fsmclpen(&self) -> FSMCLPEN_R {
        FSMCLPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fsmclpen(&mut self) -> FSMCLPEN_W<AHB3LPENRrs> {
        FSMCLPEN_W::new(self, 0)
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
