#[doc = "Register `FTSR` reader"]
pub type R = crate::R<FTSRrs>;
#[doc = "Register `FTSR` writer"]
pub type W = crate::W<FTSRrs>;
#[doc = "Falling trigger event configuration of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR0 {
    #[doc = "0: Falling edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    Enabled = 1,
}
impl From<TR0> for bool {
    #[inline(always)]
    fn from(variant: TR0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR0` reader - Falling trigger event configuration of line 0"]
pub type TR0_R = crate::BitReader<TR0>;
impl TR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TR0 {
        match self.bits {
            false => TR0::Disabled,
            true => TR0::Enabled,
        }
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR0::Disabled
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR0::Enabled
    }
}
#[doc = "Field `TR0` writer - Falling trigger event configuration of line 0"]
pub type TR0_W<'a, REG> = crate::BitWriter<'a, REG, TR0>;
impl<'a, REG> TR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR0::Disabled)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR0::Enabled)
    }
}
#[doc = "Field `TR1` reader - Falling trigger event configuration of line 1"]
pub use TR0_R as TR1_R;
#[doc = "Field `TR2` reader - Falling trigger event configuration of line 2"]
pub use TR0_R as TR2_R;
#[doc = "Field `TR3` reader - Falling trigger event configuration of line 3"]
pub use TR0_R as TR3_R;
#[doc = "Field `TR4` reader - Falling trigger event configuration of line 4"]
pub use TR0_R as TR4_R;
#[doc = "Field `TR5` reader - Falling trigger event configuration of line 5"]
pub use TR0_R as TR5_R;
#[doc = "Field `TR6` reader - Falling trigger event configuration of line 6"]
pub use TR0_R as TR6_R;
#[doc = "Field `TR7` reader - Falling trigger event configuration of line 7"]
pub use TR0_R as TR7_R;
#[doc = "Field `TR8` reader - Falling trigger event configuration of line 8"]
pub use TR0_R as TR8_R;
#[doc = "Field `TR9` reader - Falling trigger event configuration of line 9"]
pub use TR0_R as TR9_R;
#[doc = "Field `TR10` reader - Falling trigger event configuration of line 10"]
pub use TR0_R as TR10_R;
#[doc = "Field `TR11` reader - Falling trigger event configuration of line 11"]
pub use TR0_R as TR11_R;
#[doc = "Field `TR12` reader - Falling trigger event configuration of line 12"]
pub use TR0_R as TR12_R;
#[doc = "Field `TR13` reader - Falling trigger event configuration of line 13"]
pub use TR0_R as TR13_R;
#[doc = "Field `TR14` reader - Falling trigger event configuration of line 14"]
pub use TR0_R as TR14_R;
#[doc = "Field `TR15` reader - Falling trigger event configuration of line 15"]
pub use TR0_R as TR15_R;
#[doc = "Field `TR16` reader - Falling trigger event configuration of line 16"]
pub use TR0_R as TR16_R;
#[doc = "Field `TR17` reader - Falling trigger event configuration of line 17"]
pub use TR0_R as TR17_R;
#[doc = "Field `TR18` reader - Falling trigger event configuration of line 18"]
pub use TR0_R as TR18_R;
#[doc = "Field `TR19` reader - Falling trigger event configuration of line 19"]
pub use TR0_R as TR19_R;
#[doc = "Field `TR20` reader - Falling trigger event configuration of line 20"]
pub use TR0_R as TR20_R;
#[doc = "Field `TR21` reader - Falling trigger event configuration of line 21"]
pub use TR0_R as TR21_R;
#[doc = "Field `TR22` reader - Falling trigger event configuration of line 22"]
pub use TR0_R as TR22_R;
#[doc = "Field `TR1` writer - Falling trigger event configuration of line 1"]
pub use TR0_W as TR1_W;
#[doc = "Field `TR2` writer - Falling trigger event configuration of line 2"]
pub use TR0_W as TR2_W;
#[doc = "Field `TR3` writer - Falling trigger event configuration of line 3"]
pub use TR0_W as TR3_W;
#[doc = "Field `TR4` writer - Falling trigger event configuration of line 4"]
pub use TR0_W as TR4_W;
#[doc = "Field `TR5` writer - Falling trigger event configuration of line 5"]
pub use TR0_W as TR5_W;
#[doc = "Field `TR6` writer - Falling trigger event configuration of line 6"]
pub use TR0_W as TR6_W;
#[doc = "Field `TR7` writer - Falling trigger event configuration of line 7"]
pub use TR0_W as TR7_W;
#[doc = "Field `TR8` writer - Falling trigger event configuration of line 8"]
pub use TR0_W as TR8_W;
#[doc = "Field `TR9` writer - Falling trigger event configuration of line 9"]
pub use TR0_W as TR9_W;
#[doc = "Field `TR10` writer - Falling trigger event configuration of line 10"]
pub use TR0_W as TR10_W;
#[doc = "Field `TR11` writer - Falling trigger event configuration of line 11"]
pub use TR0_W as TR11_W;
#[doc = "Field `TR12` writer - Falling trigger event configuration of line 12"]
pub use TR0_W as TR12_W;
#[doc = "Field `TR13` writer - Falling trigger event configuration of line 13"]
pub use TR0_W as TR13_W;
#[doc = "Field `TR14` writer - Falling trigger event configuration of line 14"]
pub use TR0_W as TR14_W;
#[doc = "Field `TR15` writer - Falling trigger event configuration of line 15"]
pub use TR0_W as TR15_W;
#[doc = "Field `TR16` writer - Falling trigger event configuration of line 16"]
pub use TR0_W as TR16_W;
#[doc = "Field `TR17` writer - Falling trigger event configuration of line 17"]
pub use TR0_W as TR17_W;
#[doc = "Field `TR18` writer - Falling trigger event configuration of line 18"]
pub use TR0_W as TR18_W;
#[doc = "Field `TR19` writer - Falling trigger event configuration of line 19"]
pub use TR0_W as TR19_W;
#[doc = "Field `TR20` writer - Falling trigger event configuration of line 20"]
pub use TR0_W as TR20_W;
#[doc = "Field `TR21` writer - Falling trigger event configuration of line 21"]
pub use TR0_W as TR21_W;
#[doc = "Field `TR22` writer - Falling trigger event configuration of line 22"]
pub use TR0_W as TR22_W;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    pub fn tr0(&self) -> TR0_R {
        TR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    pub fn tr1(&self) -> TR1_R {
        TR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    pub fn tr2(&self) -> TR2_R {
        TR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    pub fn tr3(&self) -> TR3_R {
        TR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    pub fn tr4(&self) -> TR4_R {
        TR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    pub fn tr5(&self) -> TR5_R {
        TR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    pub fn tr6(&self) -> TR6_R {
        TR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    pub fn tr7(&self) -> TR7_R {
        TR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    pub fn tr8(&self) -> TR8_R {
        TR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    pub fn tr9(&self) -> TR9_R {
        TR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    pub fn tr10(&self) -> TR10_R {
        TR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    pub fn tr11(&self) -> TR11_R {
        TR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    pub fn tr12(&self) -> TR12_R {
        TR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    pub fn tr13(&self) -> TR13_R {
        TR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    pub fn tr14(&self) -> TR14_R {
        TR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    pub fn tr15(&self) -> TR15_R {
        TR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    pub fn tr16(&self) -> TR16_R {
        TR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    pub fn tr17(&self) -> TR17_R {
        TR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling trigger event configuration of line 18"]
    #[inline(always)]
    pub fn tr18(&self) -> TR18_R {
        TR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Falling trigger event configuration of line 19"]
    #[inline(always)]
    pub fn tr19(&self) -> TR19_R {
        TR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Falling trigger event configuration of line 20"]
    #[inline(always)]
    pub fn tr20(&self) -> TR20_R {
        TR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling trigger event configuration of line 21"]
    #[inline(always)]
    pub fn tr21(&self) -> TR21_R {
        TR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Falling trigger event configuration of line 22"]
    #[inline(always)]
    pub fn tr22(&self) -> TR22_R {
        TR22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn tr0(&mut self) -> TR0_W<FTSRrs> {
        TR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn tr1(&mut self) -> TR1_W<FTSRrs> {
        TR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn tr2(&mut self) -> TR2_W<FTSRrs> {
        TR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn tr3(&mut self) -> TR3_W<FTSRrs> {
        TR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn tr4(&mut self) -> TR4_W<FTSRrs> {
        TR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn tr5(&mut self) -> TR5_W<FTSRrs> {
        TR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn tr6(&mut self) -> TR6_W<FTSRrs> {
        TR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn tr7(&mut self) -> TR7_W<FTSRrs> {
        TR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn tr8(&mut self) -> TR8_W<FTSRrs> {
        TR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn tr9(&mut self) -> TR9_W<FTSRrs> {
        TR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn tr10(&mut self) -> TR10_W<FTSRrs> {
        TR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn tr11(&mut self) -> TR11_W<FTSRrs> {
        TR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn tr12(&mut self) -> TR12_W<FTSRrs> {
        TR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn tr13(&mut self) -> TR13_W<FTSRrs> {
        TR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn tr14(&mut self) -> TR14_W<FTSRrs> {
        TR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn tr15(&mut self) -> TR15_W<FTSRrs> {
        TR15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn tr16(&mut self) -> TR16_W<FTSRrs> {
        TR16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn tr17(&mut self) -> TR17_W<FTSRrs> {
        TR17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Falling trigger event configuration of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn tr18(&mut self) -> TR18_W<FTSRrs> {
        TR18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Falling trigger event configuration of line 19"]
    #[inline(always)]
    #[must_use]
    pub fn tr19(&mut self) -> TR19_W<FTSRrs> {
        TR19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Falling trigger event configuration of line 20"]
    #[inline(always)]
    #[must_use]
    pub fn tr20(&mut self) -> TR20_W<FTSRrs> {
        TR20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Falling trigger event configuration of line 21"]
    #[inline(always)]
    #[must_use]
    pub fn tr21(&mut self) -> TR21_W<FTSRrs> {
        TR21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Falling trigger event configuration of line 22"]
    #[inline(always)]
    #[must_use]
    pub fn tr22(&mut self) -> TR22_W<FTSRrs> {
        TR22_W::new(self, 22)
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
