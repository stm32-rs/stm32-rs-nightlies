#[doc = "Register `APB3ENR` reader"]
pub type R = crate::R<APB3ENRrs>;
#[doc = "Register `APB3ENR` writer"]
pub type W = crate::W<APB3ENRrs>;
#[doc = "LTDC peripheral clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<LTDCEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCEN` reader - LTDC peripheral clock enable"]
pub type LTDCEN_R = crate::BitReader<LTDCEN>;
impl LTDCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LTDCEN {
        match self.bits {
            false => LTDCEN::Disabled,
            true => LTDCEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN::Enabled
    }
}
#[doc = "Field `LTDCEN` writer - LTDC peripheral clock enable"]
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCEN>;
impl<'a, REG> LTDCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Enabled)
    }
}
#[doc = "Field `WWDG1EN` reader - WWDG1 Clock Enable"]
pub use LTDCEN_R as WWDG1EN_R;
#[doc = "Field `WWDG1EN` writer - WWDG1 Clock Enable"]
pub use LTDCEN_W as WWDG1EN_W;
impl R {
    #[doc = "Bit 3 - LTDC peripheral clock enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable"]
    #[inline(always)]
    pub fn wwdg1en(&self) -> WWDG1EN_R {
        WWDG1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<APB3ENRrs> {
        LTDCEN_W::new(self, 3)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg1en(&mut self) -> WWDG1EN_W<APB3ENRrs> {
        WWDG1EN_W::new(self, 6)
    }
}
#[doc = "RCC APB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3ENRrs;
impl crate::RegisterSpec for APB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3enr::R`](R) reader structure"]
impl crate::Readable for APB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure"]
impl crate::Writable for APB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3ENR to value 0"]
impl crate::Resettable for APB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
