#[doc = "Register `EMR` reader"]
pub type R = crate::R<EMRrs>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EMRrs>;
#[doc = "Event Mask on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM0 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<EM0> for bool {
    #[inline(always)]
    fn from(variant: EM0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM0` reader - Event Mask on line 0"]
pub type EM0_R = crate::BitReader<EM0>;
impl EM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM0 {
        match self.bits {
            false => EM0::Masked,
            true => EM0::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EM0::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EM0::Unmasked
    }
}
#[doc = "Field `EM0` writer - Event Mask on line 0"]
pub type EM0_W<'a, REG> = crate::BitWriter<'a, REG, EM0>;
impl<'a, REG> EM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EM0::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EM0::Unmasked)
    }
}
#[doc = "Field `EM1` reader - Event Mask on line 1"]
pub use EM0_R as EM1_R;
#[doc = "Field `EM2` reader - Event Mask on line 2"]
pub use EM0_R as EM2_R;
#[doc = "Field `EM3` reader - Event Mask on line 3"]
pub use EM0_R as EM3_R;
#[doc = "Field `EM4` reader - Event Mask on line 4"]
pub use EM0_R as EM4_R;
#[doc = "Field `EM5` reader - Event Mask on line 5"]
pub use EM0_R as EM5_R;
#[doc = "Field `EM6` reader - Event Mask on line 6"]
pub use EM0_R as EM6_R;
#[doc = "Field `EM7` reader - Event Mask on line 7"]
pub use EM0_R as EM7_R;
#[doc = "Field `EM8` reader - Event Mask on line 8"]
pub use EM0_R as EM8_R;
#[doc = "Field `EM9` reader - Event Mask on line 9"]
pub use EM0_R as EM9_R;
#[doc = "Field `EM10` reader - Event Mask on line 10"]
pub use EM0_R as EM10_R;
#[doc = "Field `EM11` reader - Event Mask on line 11"]
pub use EM0_R as EM11_R;
#[doc = "Field `EM12` reader - Event Mask on line 12"]
pub use EM0_R as EM12_R;
#[doc = "Field `EM13` reader - Event Mask on line 13"]
pub use EM0_R as EM13_R;
#[doc = "Field `EM14` reader - Event Mask on line 14"]
pub use EM0_R as EM14_R;
#[doc = "Field `EM15` reader - Event Mask on line 15"]
pub use EM0_R as EM15_R;
#[doc = "Field `EM16` reader - Event Mask on line 16"]
pub use EM0_R as EM16_R;
#[doc = "Field `EM17` reader - Event Mask on line 17"]
pub use EM0_R as EM17_R;
#[doc = "Field `EM18` reader - Event Mask on line 18"]
pub use EM0_R as EM18_R;
#[doc = "Field `EM19` reader - Event Mask on line 19"]
pub use EM0_R as EM19_R;
#[doc = "Field `EM20` reader - Event Mask on line 20"]
pub use EM0_R as EM20_R;
#[doc = "Field `EM21` reader - Event Mask on line 21"]
pub use EM0_R as EM21_R;
#[doc = "Field `EM22` reader - Event Mask on line 22"]
pub use EM0_R as EM22_R;
#[doc = "Field `EM23` reader - Event Mask on line 23"]
pub use EM0_R as EM23_R;
#[doc = "Field `EM1` writer - Event Mask on line 1"]
pub use EM0_W as EM1_W;
#[doc = "Field `EM2` writer - Event Mask on line 2"]
pub use EM0_W as EM2_W;
#[doc = "Field `EM3` writer - Event Mask on line 3"]
pub use EM0_W as EM3_W;
#[doc = "Field `EM4` writer - Event Mask on line 4"]
pub use EM0_W as EM4_W;
#[doc = "Field `EM5` writer - Event Mask on line 5"]
pub use EM0_W as EM5_W;
#[doc = "Field `EM6` writer - Event Mask on line 6"]
pub use EM0_W as EM6_W;
#[doc = "Field `EM7` writer - Event Mask on line 7"]
pub use EM0_W as EM7_W;
#[doc = "Field `EM8` writer - Event Mask on line 8"]
pub use EM0_W as EM8_W;
#[doc = "Field `EM9` writer - Event Mask on line 9"]
pub use EM0_W as EM9_W;
#[doc = "Field `EM10` writer - Event Mask on line 10"]
pub use EM0_W as EM10_W;
#[doc = "Field `EM11` writer - Event Mask on line 11"]
pub use EM0_W as EM11_W;
#[doc = "Field `EM12` writer - Event Mask on line 12"]
pub use EM0_W as EM12_W;
#[doc = "Field `EM13` writer - Event Mask on line 13"]
pub use EM0_W as EM13_W;
#[doc = "Field `EM14` writer - Event Mask on line 14"]
pub use EM0_W as EM14_W;
#[doc = "Field `EM15` writer - Event Mask on line 15"]
pub use EM0_W as EM15_W;
#[doc = "Field `EM16` writer - Event Mask on line 16"]
pub use EM0_W as EM16_W;
#[doc = "Field `EM17` writer - Event Mask on line 17"]
pub use EM0_W as EM17_W;
#[doc = "Field `EM18` writer - Event Mask on line 18"]
pub use EM0_W as EM18_W;
#[doc = "Field `EM19` writer - Event Mask on line 19"]
pub use EM0_W as EM19_W;
#[doc = "Field `EM20` writer - Event Mask on line 20"]
pub use EM0_W as EM20_W;
#[doc = "Field `EM21` writer - Event Mask on line 21"]
pub use EM0_W as EM21_W;
#[doc = "Field `EM22` writer - Event Mask on line 22"]
pub use EM0_W as EM22_W;
#[doc = "Field `EM23` writer - Event Mask on line 23"]
pub use EM0_W as EM23_W;
impl R {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    pub fn em16(&self) -> EM16_R {
        EM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    pub fn em18(&self) -> EM18_R {
        EM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event Mask on line 19"]
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Event Mask on line 20"]
    #[inline(always)]
    pub fn em20(&self) -> EM20_R {
        EM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Event Mask on line 21"]
    #[inline(always)]
    pub fn em21(&self) -> EM21_R {
        EM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event Mask on line 22"]
    #[inline(always)]
    pub fn em22(&self) -> EM22_R {
        EM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Event Mask on line 23"]
    #[inline(always)]
    pub fn em23(&self) -> EM23_R {
        EM23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn em0(&mut self) -> EM0_W<EMRrs> {
        EM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn em1(&mut self) -> EM1_W<EMRrs> {
        EM1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn em2(&mut self) -> EM2_W<EMRrs> {
        EM2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn em3(&mut self) -> EM3_W<EMRrs> {
        EM3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn em4(&mut self) -> EM4_W<EMRrs> {
        EM4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn em5(&mut self) -> EM5_W<EMRrs> {
        EM5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn em6(&mut self) -> EM6_W<EMRrs> {
        EM6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn em7(&mut self) -> EM7_W<EMRrs> {
        EM7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn em8(&mut self) -> EM8_W<EMRrs> {
        EM8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn em9(&mut self) -> EM9_W<EMRrs> {
        EM9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn em10(&mut self) -> EM10_W<EMRrs> {
        EM10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn em11(&mut self) -> EM11_W<EMRrs> {
        EM11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn em12(&mut self) -> EM12_W<EMRrs> {
        EM12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn em13(&mut self) -> EM13_W<EMRrs> {
        EM13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn em14(&mut self) -> EM14_W<EMRrs> {
        EM14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn em15(&mut self) -> EM15_W<EMRrs> {
        EM15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn em16(&mut self) -> EM16_W<EMRrs> {
        EM16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn em17(&mut self) -> EM17_W<EMRrs> {
        EM17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn em18(&mut self) -> EM18_W<EMRrs> {
        EM18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Event Mask on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn em19(&mut self) -> EM19_W<EMRrs> {
        EM19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Event Mask on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn em20(&mut self) -> EM20_W<EMRrs> {
        EM20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Event Mask on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn em21(&mut self) -> EM21_W<EMRrs> {
        EM21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Event Mask on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn em22(&mut self) -> EM22_W<EMRrs> {
        EM22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Event Mask on line 23"]
    #[inline(always)]
    #[must_use]
    pub fn em23(&mut self) -> EM23_W<EMRrs> {
        EM23_W::new(self, 23)
    }
}
#[doc = "Event mask register (EXTI_EMR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMRrs;
impl crate::RegisterSpec for EMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EMRrs {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMRrs {
    const RESET_VALUE: u32 = 0;
}
