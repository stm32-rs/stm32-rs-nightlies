#[doc = "Register `AHB3ENR` reader"]
pub type R = crate::R<AHB3ENRrs>;
#[doc = "Register `AHB3ENR` writer"]
pub type W = crate::W<AHB3ENRrs>;
#[doc = "Flexible static memory controller module clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSMCEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<FSMCEN> for bool {
    #[inline(always)]
    fn from(variant: FSMCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSMCEN` reader - Flexible static memory controller module clock enable"]
pub type FSMCEN_R = crate::BitReader<FSMCEN>;
impl FSMCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSMCEN {
        match self.bits {
            false => FSMCEN::Disabled,
            true => FSMCEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSMCEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSMCEN::Enabled
    }
}
#[doc = "Field `FSMCEN` writer - Flexible static memory controller module clock enable"]
pub type FSMCEN_W<'a, REG> = crate::BitWriter<'a, REG, FSMCEN>;
impl<'a, REG> FSMCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCEN::Enabled)
    }
}
#[doc = "Field `QSPIEN` reader - QUADSPI memory controller module clock enable"]
pub use FSMCEN_R as QSPIEN_R;
#[doc = "Field `QSPIEN` writer - QUADSPI memory controller module clock enable"]
pub use FSMCEN_W as QSPIEN_W;
impl R {
    #[doc = "Bit 0 - Flexible static memory controller module clock enable"]
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QUADSPI memory controller module clock enable"]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller module clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fsmcen(&mut self) -> FSMCEN_W<AHB3ENRrs> {
        FSMCEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - QUADSPI memory controller module clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspien(&mut self) -> QSPIEN_W<AHB3ENRrs> {
        QSPIEN_W::new(self, 1)
    }
}
#[doc = "RCC AHB3 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3enr::R`](R) reader structure"]
impl crate::Readable for AHB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure"]
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3ENR to value 0"]
impl crate::Resettable for AHB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
