#[doc = "Register `PR` reader"]
pub type R = crate::R<PRrs>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PRrs>;
#[doc = "Pending bit 0\n\nValue on reset: 0"]
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
#[doc = "Field `PR0` reader - Pending bit 0"]
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
#[doc = "Pending bit 0\n\nValue on reset: 0"]
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
#[doc = "Field `PR0` writer - Pending bit 0"]
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
#[doc = "Field `PR1` reader - Pending bit 1"]
pub use PR0_R as PR1_R;
#[doc = "Field `PR2` reader - Pending bit 2"]
pub use PR0_R as PR2_R;
#[doc = "Field `PR3` reader - Pending bit 3"]
pub use PR0_R as PR3_R;
#[doc = "Field `PR4` reader - Pending bit 4"]
pub use PR0_R as PR4_R;
#[doc = "Field `PR5` reader - Pending bit 5"]
pub use PR0_R as PR5_R;
#[doc = "Field `PR6` reader - Pending bit 6"]
pub use PR0_R as PR6_R;
#[doc = "Field `PR7` reader - Pending bit 7"]
pub use PR0_R as PR7_R;
#[doc = "Field `PR8` reader - Pending bit 8"]
pub use PR0_R as PR8_R;
#[doc = "Field `PR9` reader - Pending bit 9"]
pub use PR0_R as PR9_R;
#[doc = "Field `PR10` reader - Pending bit 10"]
pub use PR0_R as PR10_R;
#[doc = "Field `PR11` reader - Pending bit 11"]
pub use PR0_R as PR11_R;
#[doc = "Field `PR12` reader - Pending bit 12"]
pub use PR0_R as PR12_R;
#[doc = "Field `PR13` reader - Pending bit 13"]
pub use PR0_R as PR13_R;
#[doc = "Field `PR14` reader - Pending bit 14"]
pub use PR0_R as PR14_R;
#[doc = "Field `PR15` reader - Pending bit 15"]
pub use PR0_R as PR15_R;
#[doc = "Field `PR16` reader - Pending bit 16"]
pub use PR0_R as PR16_R;
#[doc = "Field `PR17` reader - Pending bit 17"]
pub use PR0_R as PR17_R;
#[doc = "Field `PR18` reader - Pending bit 18"]
pub use PR0_R as PR18_R;
#[doc = "Field `PR1` writer - Pending bit 1"]
pub use PR0_W as PR1_W;
#[doc = "Field `PR2` writer - Pending bit 2"]
pub use PR0_W as PR2_W;
#[doc = "Field `PR3` writer - Pending bit 3"]
pub use PR0_W as PR3_W;
#[doc = "Field `PR4` writer - Pending bit 4"]
pub use PR0_W as PR4_W;
#[doc = "Field `PR5` writer - Pending bit 5"]
pub use PR0_W as PR5_W;
#[doc = "Field `PR6` writer - Pending bit 6"]
pub use PR0_W as PR6_W;
#[doc = "Field `PR7` writer - Pending bit 7"]
pub use PR0_W as PR7_W;
#[doc = "Field `PR8` writer - Pending bit 8"]
pub use PR0_W as PR8_W;
#[doc = "Field `PR9` writer - Pending bit 9"]
pub use PR0_W as PR9_W;
#[doc = "Field `PR10` writer - Pending bit 10"]
pub use PR0_W as PR10_W;
#[doc = "Field `PR11` writer - Pending bit 11"]
pub use PR0_W as PR11_W;
#[doc = "Field `PR12` writer - Pending bit 12"]
pub use PR0_W as PR12_W;
#[doc = "Field `PR13` writer - Pending bit 13"]
pub use PR0_W as PR13_W;
#[doc = "Field `PR14` writer - Pending bit 14"]
pub use PR0_W as PR14_W;
#[doc = "Field `PR15` writer - Pending bit 15"]
pub use PR0_W as PR15_W;
#[doc = "Field `PR16` writer - Pending bit 16"]
pub use PR0_W as PR16_W;
#[doc = "Field `PR17` writer - Pending bit 17"]
pub use PR0_W as PR17_W;
#[doc = "Field `PR18` writer - Pending bit 18"]
pub use PR0_W as PR18_W;
impl R {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn pr17(&self) -> PR17_R {
        PR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn pr18(&self) -> PR18_R {
        PR18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pr0(&mut self) -> PR0_W<PRrs> {
        PR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pr1(&mut self) -> PR1_W<PRrs> {
        PR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pr2(&mut self) -> PR2_W<PRrs> {
        PR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pr3(&mut self) -> PR3_W<PRrs> {
        PR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pr4(&mut self) -> PR4_W<PRrs> {
        PR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pr5(&mut self) -> PR5_W<PRrs> {
        PR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pr6(&mut self) -> PR6_W<PRrs> {
        PR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pr7(&mut self) -> PR7_W<PRrs> {
        PR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pr8(&mut self) -> PR8_W<PRrs> {
        PR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pr9(&mut self) -> PR9_W<PRrs> {
        PR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pr10(&mut self) -> PR10_W<PRrs> {
        PR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pr11(&mut self) -> PR11_W<PRrs> {
        PR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pr12(&mut self) -> PR12_W<PRrs> {
        PR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pr13(&mut self) -> PR13_W<PRrs> {
        PR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pr14(&mut self) -> PR14_W<PRrs> {
        PR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pr15(&mut self) -> PR15_W<PRrs> {
        PR15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pr16(&mut self) -> PR16_W<PRrs> {
        PR16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pr17(&mut self) -> PR17_W<PRrs> {
        PR17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pr18(&mut self) -> PR18_W<PRrs> {
        PR18_W::new(self, 18)
    }
}
#[doc = "Pending register (EXTI_PR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRrs;
impl crate::RegisterSpec for PRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PRrs {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0007_ffff;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PRrs {
    const RESET_VALUE: u32 = 0;
}
