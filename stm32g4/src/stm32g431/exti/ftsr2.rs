#[doc = "Register `FTSR2` reader"]
pub type R = crate::R<FTSR2rs>;
#[doc = "Register `FTSR2` writer"]
pub type W = crate::W<FTSR2rs>;
#[doc = "Falling trigger event configuration of line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT32 {
    #[doc = "0: Falling edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    Enabled = 1,
}
impl From<FT32> for bool {
    #[inline(always)]
    fn from(variant: FT32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT32` reader - Falling trigger event configuration of line 32"]
pub type FT32_R = crate::BitReader<FT32>;
impl FT32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT32 {
        match self.bits {
            false => FT32::Disabled,
            true => FT32::Enabled,
        }
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT32::Disabled
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT32::Enabled
    }
}
#[doc = "Field `FT32` writer - Falling trigger event configuration of line 32"]
pub type FT32_W<'a, REG> = crate::BitWriter<'a, REG, FT32>;
impl<'a, REG> FT32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT32::Disabled)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT32::Enabled)
    }
}
#[doc = "Field `FT33` reader - Falling trigger event configuration of line 33"]
pub use FT32_R as FT33_R;
#[doc = "Field `FT40` reader - Falling trigger event configuration of line 40"]
pub use FT32_R as FT40_R;
#[doc = "Field `FT41` reader - Falling trigger event configuration of line 41"]
pub use FT32_R as FT41_R;
#[doc = "Field `FT33` writer - Falling trigger event configuration of line 33"]
pub use FT32_W as FT33_W;
#[doc = "Field `FT40` writer - Falling trigger event configuration of line 40"]
pub use FT32_W as FT40_W;
#[doc = "Field `FT41` writer - Falling trigger event configuration of line 41"]
pub use FT32_W as FT41_W;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration of line 32"]
    #[inline(always)]
    pub fn ft32(&self) -> FT32_R {
        FT32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 33"]
    #[inline(always)]
    pub fn ft33(&self) -> FT33_R {
        FT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 40"]
    #[inline(always)]
    pub fn ft40(&self) -> FT40_R {
        FT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 41"]
    #[inline(always)]
    pub fn ft41(&self) -> FT41_R {
        FT41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration of line 32"]
    #[inline(always)]
    #[must_use]
    pub fn ft32(&mut self) -> FT32_W<FTSR2rs> {
        FT32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 33"]
    #[inline(always)]
    #[must_use]
    pub fn ft33(&mut self) -> FT33_W<FTSR2rs> {
        FT33_W::new(self, 1)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 40"]
    #[inline(always)]
    #[must_use]
    pub fn ft40(&mut self) -> FT40_W<FTSR2rs> {
        FT40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 41"]
    #[inline(always)]
    #[must_use]
    pub fn ft41(&mut self) -> FT41_W<FTSR2rs> {
        FT41_W::new(self, 9)
    }
}
#[doc = "Falling Trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr2::R`](R) reader structure"]
impl crate::Readable for FTSR2rs {}
#[doc = "`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure"]
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for FTSR2rs {
    const RESET_VALUE: u32 = 0;
}
