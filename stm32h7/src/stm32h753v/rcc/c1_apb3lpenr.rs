#[doc = "Register `C1_APB3LPENR` reader"]
pub type R = crate::R<C1_APB3LPENRrs>;
#[doc = "Register `C1_APB3LPENR` writer"]
pub type W = crate::W<C1_APB3LPENRrs>;
#[doc = "LTDC peripheral clock enable during CSleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCLPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<LTDCLPEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCLPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCLPEN` reader - LTDC peripheral clock enable during CSleep mode"]
pub type LTDCLPEN_R = crate::BitReader<LTDCLPEN>;
impl LTDCLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LTDCLPEN {
        match self.bits {
            false => LTDCLPEN::Disabled,
            true => LTDCLPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCLPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCLPEN::Enabled
    }
}
#[doc = "Field `LTDCLPEN` writer - LTDC peripheral clock enable during CSleep mode"]
pub type LTDCLPEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCLPEN>;
impl<'a, REG> LTDCLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCLPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCLPEN::Enabled)
    }
}
#[doc = "Field `WWDG1LPEN` reader - WWDG1 Clock Enable During CSleep Mode"]
pub use LTDCLPEN_R as WWDG1LPEN_R;
#[doc = "Field `WWDG1LPEN` writer - WWDG1 Clock Enable During CSleep Mode"]
pub use LTDCLPEN_W as WWDG1LPEN_W;
impl R {
    #[doc = "Bit 3 - LTDC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn wwdg1lpen(&self) -> WWDG1LPEN_R {
        WWDG1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<C1_APB3LPENRrs> {
        LTDCLPEN_W::new(self, 3)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg1lpen(&mut self) -> WWDG1LPEN_W<C1_APB3LPENRrs> {
        WWDG1LPEN_W::new(self, 6)
    }
}
#[doc = "RCC APB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb3lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb3lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_APB3LPENRrs;
impl crate::RegisterSpec for C1_APB3LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb3lpenr::R`](R) reader structure"]
impl crate::Readable for C1_APB3LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`c1_apb3lpenr::W`](W) writer structure"]
impl crate::Writable for C1_APB3LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1_APB3LPENR to value 0"]
impl crate::Resettable for C1_APB3LPENRrs {
    const RESET_VALUE: u32 = 0;
}
