#[doc = "Register `FTSR2` reader"]
pub type R = crate::R<FTSR2rs>;
#[doc = "Register `FTSR2` writer"]
pub type W = crate::W<FTSR2rs>;
#[doc = "Falling trigger event configuration bit of line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT35 {
    #[doc = "0: Falling edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    Enabled = 1,
}
impl From<FT35> for bool {
    #[inline(always)]
    fn from(variant: FT35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT35` reader - Falling trigger event configuration bit of line 35"]
pub type FT35_R = crate::BitReader<FT35>;
impl FT35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT35 {
        match self.bits {
            false => FT35::Disabled,
            true => FT35::Enabled,
        }
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT35::Disabled
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT35::Enabled
    }
}
#[doc = "Field `FT35` writer - Falling trigger event configuration bit of line 35"]
pub type FT35_W<'a, REG> = crate::BitWriter<'a, REG, FT35>;
impl<'a, REG> FT35_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT35::Disabled)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT35::Enabled)
    }
}
#[doc = "Field `FT36` reader - Falling trigger event configuration bit of line 36"]
pub use FT35_R as FT36_R;
#[doc = "Field `FT37` reader - Falling trigger event configuration bit of line 37"]
pub use FT35_R as FT37_R;
#[doc = "Field `FT38` reader - Falling trigger event configuration bit of line 38"]
pub use FT35_R as FT38_R;
#[doc = "Field `FT36` writer - Falling trigger event configuration bit of line 36"]
pub use FT35_W as FT36_W;
#[doc = "Field `FT37` writer - Falling trigger event configuration bit of line 37"]
pub use FT35_W as FT37_W;
#[doc = "Field `FT38` writer - Falling trigger event configuration bit of line 38"]
pub use FT35_W as FT38_W;
impl R {
    #[doc = "Bit 3 - Falling trigger event configuration bit of line 35"]
    #[inline(always)]
    pub fn ft35(&self) -> FT35_R {
        FT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of line 36"]
    #[inline(always)]
    pub fn ft36(&self) -> FT36_R {
        FT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of line 37"]
    #[inline(always)]
    pub fn ft37(&self) -> FT37_R {
        FT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn ft38(&self) -> FT38_R {
        FT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Falling trigger event configuration bit of line 35"]
    #[inline(always)]
    #[must_use]
    pub fn ft35(&mut self) -> FT35_W<FTSR2rs> {
        FT35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of line 36"]
    #[inline(always)]
    #[must_use]
    pub fn ft36(&mut self) -> FT36_W<FTSR2rs> {
        FT36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of line 37"]
    #[inline(always)]
    #[must_use]
    pub fn ft37(&mut self) -> FT37_W<FTSR2rs> {
        FT37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of line 38"]
    #[inline(always)]
    #[must_use]
    pub fn ft38(&mut self) -> FT38_W<FTSR2rs> {
        FT38_W::new(self, 6)
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
