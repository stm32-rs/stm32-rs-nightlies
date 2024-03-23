#[doc = "Register `FTSR` reader"]
pub type R = crate::R<FTSRrs>;
#[doc = "Register `FTSR` writer"]
pub type W = crate::W<FTSRrs>;
#[doc = "Falling trigger event configuration of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT0 {
    #[doc = "0: Falling edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    Enabled = 1,
}
impl From<FT0> for bool {
    #[inline(always)]
    fn from(variant: FT0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT0` reader - Falling trigger event configuration of line 0"]
pub type FT0_R = crate::BitReader<FT0>;
impl FT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT0 {
        match self.bits {
            false => FT0::Disabled,
            true => FT0::Enabled,
        }
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT0::Disabled
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT0::Enabled
    }
}
#[doc = "Field `FT0` writer - Falling trigger event configuration of line 0"]
pub type FT0_W<'a, REG> = crate::BitWriter<'a, REG, FT0>;
impl<'a, REG> FT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT0::Disabled)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT0::Enabled)
    }
}
#[doc = "Field `FT1` reader - Falling trigger event configuration of line 1"]
pub use FT0_R as FT1_R;
#[doc = "Field `FT2` reader - Falling trigger event configuration of line 2"]
pub use FT0_R as FT2_R;
#[doc = "Field `FT3` reader - Falling trigger event configuration of line 3"]
pub use FT0_R as FT3_R;
#[doc = "Field `FT4` reader - Falling trigger event configuration of line 4"]
pub use FT0_R as FT4_R;
#[doc = "Field `FT5` reader - Falling trigger event configuration of line 5"]
pub use FT0_R as FT5_R;
#[doc = "Field `FT6` reader - Falling trigger event configuration of line 6"]
pub use FT0_R as FT6_R;
#[doc = "Field `FT7` reader - Falling trigger event configuration of line 7"]
pub use FT0_R as FT7_R;
#[doc = "Field `FT8` reader - Falling trigger event configuration of line 8"]
pub use FT0_R as FT8_R;
#[doc = "Field `FT9` reader - Falling trigger event configuration of line 9"]
pub use FT0_R as FT9_R;
#[doc = "Field `FT10` reader - Falling trigger event configuration of line 10"]
pub use FT0_R as FT10_R;
#[doc = "Field `FT11` reader - Falling trigger event configuration of line 11"]
pub use FT0_R as FT11_R;
#[doc = "Field `FT12` reader - Falling trigger event configuration of line 12"]
pub use FT0_R as FT12_R;
#[doc = "Field `FT13` reader - Falling trigger event configuration of line 13"]
pub use FT0_R as FT13_R;
#[doc = "Field `FT14` reader - Falling trigger event configuration of line 14"]
pub use FT0_R as FT14_R;
#[doc = "Field `FT15` reader - Falling trigger event configuration of line 15"]
pub use FT0_R as FT15_R;
#[doc = "Field `FT16` reader - Falling trigger event configuration of line 16"]
pub use FT0_R as FT16_R;
#[doc = "Field `FT17` reader - Falling trigger event configuration of line 17"]
pub use FT0_R as FT17_R;
#[doc = "Field `FT19` reader - Falling trigger event configuration of line 19"]
pub use FT0_R as FT19_R;
#[doc = "Field `FT20` reader - Falling trigger event configuration of line 20"]
pub use FT0_R as FT20_R;
#[doc = "Field `FT21` reader - Falling trigger event configuration of line 21"]
pub use FT0_R as FT21_R;
#[doc = "Field `FT22` reader - Falling trigger event configuration of line 22"]
pub use FT0_R as FT22_R;
#[doc = "Field `FT1` writer - Falling trigger event configuration of line 1"]
pub use FT0_W as FT1_W;
#[doc = "Field `FT2` writer - Falling trigger event configuration of line 2"]
pub use FT0_W as FT2_W;
#[doc = "Field `FT3` writer - Falling trigger event configuration of line 3"]
pub use FT0_W as FT3_W;
#[doc = "Field `FT4` writer - Falling trigger event configuration of line 4"]
pub use FT0_W as FT4_W;
#[doc = "Field `FT5` writer - Falling trigger event configuration of line 5"]
pub use FT0_W as FT5_W;
#[doc = "Field `FT6` writer - Falling trigger event configuration of line 6"]
pub use FT0_W as FT6_W;
#[doc = "Field `FT7` writer - Falling trigger event configuration of line 7"]
pub use FT0_W as FT7_W;
#[doc = "Field `FT8` writer - Falling trigger event configuration of line 8"]
pub use FT0_W as FT8_W;
#[doc = "Field `FT9` writer - Falling trigger event configuration of line 9"]
pub use FT0_W as FT9_W;
#[doc = "Field `FT10` writer - Falling trigger event configuration of line 10"]
pub use FT0_W as FT10_W;
#[doc = "Field `FT11` writer - Falling trigger event configuration of line 11"]
pub use FT0_W as FT11_W;
#[doc = "Field `FT12` writer - Falling trigger event configuration of line 12"]
pub use FT0_W as FT12_W;
#[doc = "Field `FT13` writer - Falling trigger event configuration of line 13"]
pub use FT0_W as FT13_W;
#[doc = "Field `FT14` writer - Falling trigger event configuration of line 14"]
pub use FT0_W as FT14_W;
#[doc = "Field `FT15` writer - Falling trigger event configuration of line 15"]
pub use FT0_W as FT15_W;
#[doc = "Field `FT16` writer - Falling trigger event configuration of line 16"]
pub use FT0_W as FT16_W;
#[doc = "Field `FT17` writer - Falling trigger event configuration of line 17"]
pub use FT0_W as FT17_W;
#[doc = "Field `FT19` writer - Falling trigger event configuration of line 19"]
pub use FT0_W as FT19_W;
#[doc = "Field `FT20` writer - Falling trigger event configuration of line 20"]
pub use FT0_W as FT20_W;
#[doc = "Field `FT21` writer - Falling trigger event configuration of line 21"]
pub use FT0_W as FT21_W;
#[doc = "Field `FT22` writer - Falling trigger event configuration of line 22"]
pub use FT0_W as FT22_W;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    pub fn ft16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    pub fn ft17(&self) -> FT17_R {
        FT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Falling trigger event configuration of line 19"]
    #[inline(always)]
    pub fn ft19(&self) -> FT19_R {
        FT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Falling trigger event configuration of line 20"]
    #[inline(always)]
    pub fn ft20(&self) -> FT20_R {
        FT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling trigger event configuration of line 21"]
    #[inline(always)]
    pub fn ft21(&self) -> FT21_R {
        FT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Falling trigger event configuration of line 22"]
    #[inline(always)]
    pub fn ft22(&self) -> FT22_R {
        FT22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn ft0(&mut self) -> FT0_W<FTSRrs> {
        FT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn ft1(&mut self) -> FT1_W<FTSRrs> {
        FT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn ft2(&mut self) -> FT2_W<FTSRrs> {
        FT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn ft3(&mut self) -> FT3_W<FTSRrs> {
        FT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn ft4(&mut self) -> FT4_W<FTSRrs> {
        FT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn ft5(&mut self) -> FT5_W<FTSRrs> {
        FT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn ft6(&mut self) -> FT6_W<FTSRrs> {
        FT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn ft7(&mut self) -> FT7_W<FTSRrs> {
        FT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn ft8(&mut self) -> FT8_W<FTSRrs> {
        FT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn ft9(&mut self) -> FT9_W<FTSRrs> {
        FT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn ft10(&mut self) -> FT10_W<FTSRrs> {
        FT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn ft11(&mut self) -> FT11_W<FTSRrs> {
        FT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn ft12(&mut self) -> FT12_W<FTSRrs> {
        FT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn ft13(&mut self) -> FT13_W<FTSRrs> {
        FT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn ft14(&mut self) -> FT14_W<FTSRrs> {
        FT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn ft15(&mut self) -> FT15_W<FTSRrs> {
        FT15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn ft16(&mut self) -> FT16_W<FTSRrs> {
        FT16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn ft17(&mut self) -> FT17_W<FTSRrs> {
        FT17_W::new(self, 17)
    }
    #[doc = "Bit 19 - Falling trigger event configuration of line 19"]
    #[inline(always)]
    #[must_use]
    pub fn ft19(&mut self) -> FT19_W<FTSRrs> {
        FT19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Falling trigger event configuration of line 20"]
    #[inline(always)]
    #[must_use]
    pub fn ft20(&mut self) -> FT20_W<FTSRrs> {
        FT20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Falling trigger event configuration of line 21"]
    #[inline(always)]
    #[must_use]
    pub fn ft21(&mut self) -> FT21_W<FTSRrs> {
        FT21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Falling trigger event configuration of line 22"]
    #[inline(always)]
    #[must_use]
    pub fn ft22(&mut self) -> FT22_W<FTSRrs> {
        FT22_W::new(self, 22)
    }
}
#[doc = "Falling Trigger selection register (EXTI_FTSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSRrs;
impl crate::RegisterSpec for FTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr::R`](R) reader structure"]
impl crate::Readable for FTSRrs {}
#[doc = "`write(|w| ..)` method takes [`ftsr::W`](W) writer structure"]
impl crate::Writable for FTSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTSR to value 0"]
impl crate::Resettable for FTSRrs {
    const RESET_VALUE: u32 = 0;
}
