#[doc = "Register `DIFSEL` reader"]
pub type R = crate::R<DIFSELrs>;
#[doc = "Register `DIFSEL` writer"]
pub type W = crate::W<DIFSELrs>;
#[doc = "Differential mode for channels 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFSEL_0 {
    #[doc = "0: Input channel is configured in single-ended mode"]
    SingleEnded = 0,
    #[doc = "1: Input channel is configured in differential mode"]
    Differential = 1,
}
impl From<DIFSEL_0> for bool {
    #[inline(always)]
    fn from(variant: DIFSEL_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFSEL_0` reader - Differential mode for channels 0"]
pub type DIFSEL_0_R = crate::BitReader<DIFSEL_0>;
impl DIFSEL_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIFSEL_0 {
        match self.bits {
            false => DIFSEL_0::SingleEnded,
            true => DIFSEL_0::Differential,
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFSEL_0::SingleEnded
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFSEL_0::Differential
    }
}
#[doc = "Field `DIFSEL_0` writer - Differential mode for channels 0"]
pub type DIFSEL_0_W<'a, REG> = crate::BitWriter<'a, REG, DIFSEL_0>;
impl<'a, REG> DIFSEL_0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL_0::SingleEnded)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL_0::Differential)
    }
}
#[doc = "Field `DIFSEL_1` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_1_R;
#[doc = "Field `DIFSEL_2` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_2_R;
#[doc = "Field `DIFSEL_3` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_3_R;
#[doc = "Field `DIFSEL_4` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_4_R;
#[doc = "Field `DIFSEL_5` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_5_R;
#[doc = "Field `DIFSEL_6` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_6_R;
#[doc = "Field `DIFSEL_7` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_7_R;
#[doc = "Field `DIFSEL_8` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_8_R;
#[doc = "Field `DIFSEL_9` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_9_R;
#[doc = "Field `DIFSEL_10` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_10_R;
#[doc = "Field `DIFSEL_11` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_11_R;
#[doc = "Field `DIFSEL_12` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_12_R;
#[doc = "Field `DIFSEL_13` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_13_R;
#[doc = "Field `DIFSEL_14` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_14_R;
#[doc = "Field `DIFSEL_15` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_15_R;
#[doc = "Field `DIFSEL_16` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_16_R;
#[doc = "Field `DIFSEL_17` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_17_R;
#[doc = "Field `DIFSEL_18` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_18_R;
#[doc = "Field `DIFSEL_1` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_1_W;
#[doc = "Field `DIFSEL_2` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_2_W;
#[doc = "Field `DIFSEL_3` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_3_W;
#[doc = "Field `DIFSEL_4` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_4_W;
#[doc = "Field `DIFSEL_5` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_5_W;
#[doc = "Field `DIFSEL_6` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_6_W;
#[doc = "Field `DIFSEL_7` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_7_W;
#[doc = "Field `DIFSEL_8` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_8_W;
#[doc = "Field `DIFSEL_9` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_9_W;
#[doc = "Field `DIFSEL_10` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_10_W;
#[doc = "Field `DIFSEL_11` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_11_W;
#[doc = "Field `DIFSEL_12` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_12_W;
#[doc = "Field `DIFSEL_13` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_13_W;
#[doc = "Field `DIFSEL_14` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_14_W;
#[doc = "Field `DIFSEL_15` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_15_W;
#[doc = "Field `DIFSEL_16` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_16_W;
#[doc = "Field `DIFSEL_17` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_17_W;
#[doc = "Field `DIFSEL_18` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_18_W;
impl R {
    #[doc = "Bit 0 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_0(&self) -> DIFSEL_0_R {
        DIFSEL_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_1(&self) -> DIFSEL_1_R {
        DIFSEL_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_2(&self) -> DIFSEL_2_R {
        DIFSEL_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_3(&self) -> DIFSEL_3_R {
        DIFSEL_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_4(&self) -> DIFSEL_4_R {
        DIFSEL_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_5(&self) -> DIFSEL_5_R {
        DIFSEL_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_6(&self) -> DIFSEL_6_R {
        DIFSEL_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_7(&self) -> DIFSEL_7_R {
        DIFSEL_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_8(&self) -> DIFSEL_8_R {
        DIFSEL_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_9(&self) -> DIFSEL_9_R {
        DIFSEL_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_10(&self) -> DIFSEL_10_R {
        DIFSEL_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_11(&self) -> DIFSEL_11_R {
        DIFSEL_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_12(&self) -> DIFSEL_12_R {
        DIFSEL_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_13(&self) -> DIFSEL_13_R {
        DIFSEL_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_14(&self) -> DIFSEL_14_R {
        DIFSEL_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_15(&self) -> DIFSEL_15_R {
        DIFSEL_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_16(&self) -> DIFSEL_16_R {
        DIFSEL_16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_17(&self) -> DIFSEL_17_R {
        DIFSEL_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_18(&self) -> DIFSEL_18_R {
        DIFSEL_18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_0(&mut self) -> DIFSEL_0_W<DIFSELrs> {
        DIFSEL_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_1(&mut self) -> DIFSEL_1_W<DIFSELrs> {
        DIFSEL_1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_2(&mut self) -> DIFSEL_2_W<DIFSELrs> {
        DIFSEL_2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_3(&mut self) -> DIFSEL_3_W<DIFSELrs> {
        DIFSEL_3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_4(&mut self) -> DIFSEL_4_W<DIFSELrs> {
        DIFSEL_4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_5(&mut self) -> DIFSEL_5_W<DIFSELrs> {
        DIFSEL_5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_6(&mut self) -> DIFSEL_6_W<DIFSELrs> {
        DIFSEL_6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_7(&mut self) -> DIFSEL_7_W<DIFSELrs> {
        DIFSEL_7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_8(&mut self) -> DIFSEL_8_W<DIFSELrs> {
        DIFSEL_8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_9(&mut self) -> DIFSEL_9_W<DIFSELrs> {
        DIFSEL_9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_10(&mut self) -> DIFSEL_10_W<DIFSELrs> {
        DIFSEL_10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_11(&mut self) -> DIFSEL_11_W<DIFSELrs> {
        DIFSEL_11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_12(&mut self) -> DIFSEL_12_W<DIFSELrs> {
        DIFSEL_12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_13(&mut self) -> DIFSEL_13_W<DIFSELrs> {
        DIFSEL_13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_14(&mut self) -> DIFSEL_14_W<DIFSELrs> {
        DIFSEL_14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_15(&mut self) -> DIFSEL_15_W<DIFSELrs> {
        DIFSEL_15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_16(&mut self) -> DIFSEL_16_W<DIFSELrs> {
        DIFSEL_16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_17(&mut self) -> DIFSEL_17_W<DIFSELrs> {
        DIFSEL_17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_18(&mut self) -> DIFSEL_18_W<DIFSELrs> {
        DIFSEL_18_W::new(self, 18)
    }
}
#[doc = "Differential Mode Selection Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`difsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIFSELrs;
impl crate::RegisterSpec for DIFSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`difsel::R`](R) reader structure"]
impl crate::Readable for DIFSELrs {}
#[doc = "`write(|w| ..)` method takes [`difsel::W`](W) writer structure"]
impl crate::Writable for DIFSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIFSEL to value 0"]
impl crate::Resettable for DIFSELrs {
    const RESET_VALUE: u32 = 0;
}
