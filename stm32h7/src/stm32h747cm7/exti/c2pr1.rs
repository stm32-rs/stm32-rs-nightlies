#[doc = "Register `C2PR1` reader"]
pub type R = crate::R<C2PR1rs>;
#[doc = "Register `C2PR1` writer"]
pub type W = crate::W<C2PR1rs>;
#[doc = "Configurable event input pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR0R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PR0R> for bool {
    #[inline(always)]
    fn from(variant: PR0R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR0` reader - Configurable event input pending"]
pub type PR0_R = crate::BitReader<PR0R>;
impl PR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PR0R {
        match self.bits {
            false => PR0R::NotPending,
            true => PR0R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR0R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR0R::Pending
    }
}
#[doc = "Configurable event input pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR0W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PR0W> for bool {
    #[inline(always)]
    fn from(variant: PR0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR0` writer - Configurable event input pending"]
pub type PR0_W<'a, REG> = crate::BitWriter1C<'a, REG, PR0W>;
impl<'a, REG> PR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PR0W::Clear)
    }
}
#[doc = "Field `PR1` reader - Configurable event input pending"]
pub use PR0_R as PR1_R;
#[doc = "Field `PR2` reader - Configurable event input pending"]
pub use PR0_R as PR2_R;
#[doc = "Field `PR3` reader - Configurable event input pending"]
pub use PR0_R as PR3_R;
#[doc = "Field `PR4` reader - Configurable event input pending"]
pub use PR0_R as PR4_R;
#[doc = "Field `PR5` reader - Configurable event input pending"]
pub use PR0_R as PR5_R;
#[doc = "Field `PR6` reader - Configurable event input pending"]
pub use PR0_R as PR6_R;
#[doc = "Field `PR7` reader - Configurable event input pending"]
pub use PR0_R as PR7_R;
#[doc = "Field `PR8` reader - Configurable event input pending"]
pub use PR0_R as PR8_R;
#[doc = "Field `PR9` reader - Configurable event input pending"]
pub use PR0_R as PR9_R;
#[doc = "Field `PR10` reader - Configurable event input pending"]
pub use PR0_R as PR10_R;
#[doc = "Field `PR11` reader - Configurable event input pending"]
pub use PR0_R as PR11_R;
#[doc = "Field `PR12` reader - Configurable event input pending"]
pub use PR0_R as PR12_R;
#[doc = "Field `PR13` reader - Configurable event input pending"]
pub use PR0_R as PR13_R;
#[doc = "Field `PR14` reader - Configurable event input pending"]
pub use PR0_R as PR14_R;
#[doc = "Field `PR15` reader - Configurable event input pending"]
pub use PR0_R as PR15_R;
#[doc = "Field `PR16` reader - Configurable event input pending"]
pub use PR0_R as PR16_R;
#[doc = "Field `PR17` reader - Configurable event input pending"]
pub use PR0_R as PR17_R;
#[doc = "Field `PR18` reader - Configurable event input pending"]
pub use PR0_R as PR18_R;
#[doc = "Field `PR19` reader - Configurable event input pending"]
pub use PR0_R as PR19_R;
#[doc = "Field `PR20` reader - Configurable event input pending"]
pub use PR0_R as PR20_R;
#[doc = "Field `PR21` reader - Configurable event input pending"]
pub use PR0_R as PR21_R;
#[doc = "Field `PR1` writer - Configurable event input pending"]
pub use PR0_W as PR1_W;
#[doc = "Field `PR2` writer - Configurable event input pending"]
pub use PR0_W as PR2_W;
#[doc = "Field `PR3` writer - Configurable event input pending"]
pub use PR0_W as PR3_W;
#[doc = "Field `PR4` writer - Configurable event input pending"]
pub use PR0_W as PR4_W;
#[doc = "Field `PR5` writer - Configurable event input pending"]
pub use PR0_W as PR5_W;
#[doc = "Field `PR6` writer - Configurable event input pending"]
pub use PR0_W as PR6_W;
#[doc = "Field `PR7` writer - Configurable event input pending"]
pub use PR0_W as PR7_W;
#[doc = "Field `PR8` writer - Configurable event input pending"]
pub use PR0_W as PR8_W;
#[doc = "Field `PR9` writer - Configurable event input pending"]
pub use PR0_W as PR9_W;
#[doc = "Field `PR10` writer - Configurable event input pending"]
pub use PR0_W as PR10_W;
#[doc = "Field `PR11` writer - Configurable event input pending"]
pub use PR0_W as PR11_W;
#[doc = "Field `PR12` writer - Configurable event input pending"]
pub use PR0_W as PR12_W;
#[doc = "Field `PR13` writer - Configurable event input pending"]
pub use PR0_W as PR13_W;
#[doc = "Field `PR14` writer - Configurable event input pending"]
pub use PR0_W as PR14_W;
#[doc = "Field `PR15` writer - Configurable event input pending"]
pub use PR0_W as PR15_W;
#[doc = "Field `PR16` writer - Configurable event input pending"]
pub use PR0_W as PR16_W;
#[doc = "Field `PR17` writer - Configurable event input pending"]
pub use PR0_W as PR17_W;
#[doc = "Field `PR18` writer - Configurable event input pending"]
pub use PR0_W as PR18_W;
#[doc = "Field `PR19` writer - Configurable event input pending"]
pub use PR0_W as PR19_W;
#[doc = "Field `PR20` writer - Configurable event input pending"]
pub use PR0_W as PR20_W;
#[doc = "Field `PR21` writer - Configurable event input pending"]
pub use PR0_W as PR21_W;
impl R {
    #[doc = "Bit 0 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr17(&self) -> PR17_R {
        PR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr18(&self) -> PR18_R {
        PR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr19(&self) -> PR19_R {
        PR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr20(&self) -> PR20_R {
        PR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr21(&self) -> PR21_R {
        PR21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr0(&mut self) -> PR0_W<C2PR1rs> {
        PR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr1(&mut self) -> PR1_W<C2PR1rs> {
        PR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr2(&mut self) -> PR2_W<C2PR1rs> {
        PR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr3(&mut self) -> PR3_W<C2PR1rs> {
        PR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr4(&mut self) -> PR4_W<C2PR1rs> {
        PR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr5(&mut self) -> PR5_W<C2PR1rs> {
        PR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr6(&mut self) -> PR6_W<C2PR1rs> {
        PR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr7(&mut self) -> PR7_W<C2PR1rs> {
        PR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr8(&mut self) -> PR8_W<C2PR1rs> {
        PR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr9(&mut self) -> PR9_W<C2PR1rs> {
        PR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr10(&mut self) -> PR10_W<C2PR1rs> {
        PR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr11(&mut self) -> PR11_W<C2PR1rs> {
        PR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr12(&mut self) -> PR12_W<C2PR1rs> {
        PR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr13(&mut self) -> PR13_W<C2PR1rs> {
        PR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr14(&mut self) -> PR14_W<C2PR1rs> {
        PR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr15(&mut self) -> PR15_W<C2PR1rs> {
        PR15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr16(&mut self) -> PR16_W<C2PR1rs> {
        PR16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr17(&mut self) -> PR17_W<C2PR1rs> {
        PR17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr18(&mut self) -> PR18_W<C2PR1rs> {
        PR18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr19(&mut self) -> PR19_W<C2PR1rs> {
        PR19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr20(&mut self) -> PR20_W<C2PR1rs> {
        PR20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configurable event input pending"]
    #[inline(always)]
    #[must_use]
    pub fn pr21(&mut self) -> PR21_W<C2PR1rs> {
        PR21_W::new(self, 21)
    }
}
#[doc = "CPU2 EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2pr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2pr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2PR1rs;
impl crate::RegisterSpec for C2PR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2pr1::R`](R) reader structure"]
impl crate::Readable for C2PR1rs {}
#[doc = "`write(|w| ..)` method takes [`c2pr1::W`](W) writer structure"]
impl crate::Writable for C2PR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x003f_ffff;
}
#[doc = "`reset()` method sets C2PR1 to value 0"]
impl crate::Resettable for C2PR1rs {
    const RESET_VALUE: u32 = 0;
}
