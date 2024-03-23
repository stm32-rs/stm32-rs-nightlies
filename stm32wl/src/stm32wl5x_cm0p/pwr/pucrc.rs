#[doc = "Register `PUCRC` reader"]
pub type R = crate::R<PUCRCrs>;
#[doc = "Register `PUCRC` writer"]
pub type W = crate::W<PUCRCrs>;
#[doc = "PU0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU0 {
    #[doc = "0: Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    Enabled = 1,
}
impl From<PU0> for bool {
    #[inline(always)]
    fn from(variant: PU0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PU0` reader - PU0"]
pub type PU0_R = crate::BitReader<PU0>;
impl PU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PU0 {
        match self.bits {
            false => PU0::Disabled,
            true => PU0::Enabled,
        }
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU0::Disabled
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU0::Enabled
    }
}
#[doc = "Field `PU0` writer - PU0"]
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG, PU0>;
impl<'a, REG> PU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Disabled)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Enabled)
    }
}
#[doc = "Field `PU1` reader - PU1"]
pub use PU0_R as PU1_R;
#[doc = "Field `PU2` reader - PU2"]
pub use PU0_R as PU2_R;
#[doc = "Field `PU3` reader - PU3"]
pub use PU0_R as PU3_R;
#[doc = "Field `PU4` reader - PU4"]
pub use PU0_R as PU4_R;
#[doc = "Field `PU5` reader - PU5"]
pub use PU0_R as PU5_R;
#[doc = "Field `PU6` reader - PU6"]
pub use PU0_R as PU6_R;
#[doc = "Field `PU1` writer - PU1"]
pub use PU0_W as PU1_W;
#[doc = "Field `PU2` writer - PU2"]
pub use PU0_W as PU2_W;
#[doc = "Field `PU3` writer - PU3"]
pub use PU0_W as PU3_W;
#[doc = "Field `PU4` writer - PU4"]
pub use PU0_W as PU4_W;
#[doc = "Field `PU5` writer - PU5"]
pub use PU0_W as PU5_W;
#[doc = "Field `PU6` writer - PU6"]
pub use PU0_W as PU6_W;
#[doc = "PU13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU13 {
    #[doc = "0: Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    Enabled = 1,
}
impl From<PU13> for bool {
    #[inline(always)]
    fn from(variant: PU13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PU13` reader - PU13"]
pub type PU13_R = crate::BitReader<PU13>;
impl PU13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PU13 {
        match self.bits {
            false => PU13::Disabled,
            true => PU13::Enabled,
        }
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU13::Disabled
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU13::Enabled
    }
}
#[doc = "Field `PU13` writer - PU13"]
pub type PU13_W<'a, REG> = crate::BitWriter<'a, REG, PU13>;
impl<'a, REG> PU13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU13::Disabled)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU13::Enabled)
    }
}
#[doc = "Field `PU14` reader - PU14"]
pub use PU13_R as PU14_R;
#[doc = "Field `PU15` reader - Port PC\\[y\\]
pull-up (y=13 to 15)"]
pub use PU13_R as PU15_R;
#[doc = "Field `PU14` writer - PU14"]
pub use PU13_W as PU14_W;
#[doc = "Field `PU15` writer - Port PC\\[y\\]
pull-up (y=13 to 15)"]
pub use PU13_W as PU15_W;
impl R {
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PU4"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PU5"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PU6"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - PU13"]
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PU14"]
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port PC\\[y\\]
pull-up (y=13 to 15)"]
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<PUCRCrs> {
        PU0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<PUCRCrs> {
        PU1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<PUCRCrs> {
        PU2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<PUCRCrs> {
        PU3_W::new(self, 3)
    }
    #[doc = "Bit 4 - PU4"]
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<PUCRCrs> {
        PU4_W::new(self, 4)
    }
    #[doc = "Bit 5 - PU5"]
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> PU5_W<PUCRCrs> {
        PU5_W::new(self, 5)
    }
    #[doc = "Bit 6 - PU6"]
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<PUCRCrs> {
        PU6_W::new(self, 6)
    }
    #[doc = "Bit 13 - PU13"]
    #[inline(always)]
    #[must_use]
    pub fn pu13(&mut self) -> PU13_W<PUCRCrs> {
        PU13_W::new(self, 13)
    }
    #[doc = "Bit 14 - PU14"]
    #[inline(always)]
    #[must_use]
    pub fn pu14(&mut self) -> PU14_W<PUCRCrs> {
        PU14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port PC\\[y\\]
pull-up (y=13 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu15(&mut self) -> PU15_W<PUCRCrs> {
        PU15_W::new(self, 15)
    }
}
#[doc = "Power Port C pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRCrs;
impl crate::RegisterSpec for PUCRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucrc::R`](R) reader structure"]
impl crate::Readable for PUCRCrs {}
#[doc = "`write(|w| ..)` method takes [`pucrc::W`](W) writer structure"]
impl crate::Writable for PUCRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCRC to value 0"]
impl crate::Resettable for PUCRCrs {
    const RESET_VALUE: u32 = 0;
}
