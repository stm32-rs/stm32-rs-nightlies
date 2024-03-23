#[doc = "Register `PR` reader"]
pub type R = crate::R<PRrs>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PRrs>;
#[doc = "Pending bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF0R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PIF0R> for bool {
    #[inline(always)]
    fn from(variant: PIF0R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF0` reader - Pending bit 0"]
pub type PIF0_R = crate::BitReader<PIF0R>;
impl PIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIF0R {
        match self.bits {
            false => PIF0R::NotPending,
            true => PIF0R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF0R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF0R::Pending
    }
}
#[doc = "Pending bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF0W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PIF0W> for bool {
    #[inline(always)]
    fn from(variant: PIF0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF0` writer - Pending bit 0"]
pub type PIF0_W<'a, REG> = crate::BitWriter1C<'a, REG, PIF0W>;
impl<'a, REG> PIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PIF0W::Clear)
    }
}
#[doc = "Field `PIF1` reader - Pending bit 1"]
pub use PIF0_R as PIF1_R;
#[doc = "Field `PIF2` reader - Pending bit 2"]
pub use PIF0_R as PIF2_R;
#[doc = "Field `PIF3` reader - Pending bit 3"]
pub use PIF0_R as PIF3_R;
#[doc = "Field `PIF4` reader - Pending bit 4"]
pub use PIF0_R as PIF4_R;
#[doc = "Field `PIF5` reader - Pending bit 5"]
pub use PIF0_R as PIF5_R;
#[doc = "Field `PIF6` reader - Pending bit 6"]
pub use PIF0_R as PIF6_R;
#[doc = "Field `PIF7` reader - Pending bit 7"]
pub use PIF0_R as PIF7_R;
#[doc = "Field `PIF8` reader - Pending bit 8"]
pub use PIF0_R as PIF8_R;
#[doc = "Field `PIF9` reader - Pending bit 9"]
pub use PIF0_R as PIF9_R;
#[doc = "Field `PIF10` reader - Pending bit 10"]
pub use PIF0_R as PIF10_R;
#[doc = "Field `PIF11` reader - Pending bit 11"]
pub use PIF0_R as PIF11_R;
#[doc = "Field `PIF12` reader - Pending bit 12"]
pub use PIF0_R as PIF12_R;
#[doc = "Field `PIF13` reader - Pending bit 13"]
pub use PIF0_R as PIF13_R;
#[doc = "Field `PIF14` reader - Pending bit 14"]
pub use PIF0_R as PIF14_R;
#[doc = "Field `PIF15` reader - Pending bit 15"]
pub use PIF0_R as PIF15_R;
#[doc = "Field `PIF16` reader - Pending bit 16"]
pub use PIF0_R as PIF16_R;
#[doc = "Field `PIF17` reader - Pending bit 17"]
pub use PIF0_R as PIF17_R;
#[doc = "Field `PIF19` reader - Pending bit 19"]
pub use PIF0_R as PIF19_R;
#[doc = "Field `PIF20` reader - Pending bit 20"]
pub use PIF0_R as PIF20_R;
#[doc = "Field `PIF21` reader - Pending bit 21"]
pub use PIF0_R as PIF21_R;
#[doc = "Field `PIF22` reader - Pending bit 22"]
pub use PIF0_R as PIF22_R;
#[doc = "Field `PIF1` writer - Pending bit 1"]
pub use PIF0_W as PIF1_W;
#[doc = "Field `PIF2` writer - Pending bit 2"]
pub use PIF0_W as PIF2_W;
#[doc = "Field `PIF3` writer - Pending bit 3"]
pub use PIF0_W as PIF3_W;
#[doc = "Field `PIF4` writer - Pending bit 4"]
pub use PIF0_W as PIF4_W;
#[doc = "Field `PIF5` writer - Pending bit 5"]
pub use PIF0_W as PIF5_W;
#[doc = "Field `PIF6` writer - Pending bit 6"]
pub use PIF0_W as PIF6_W;
#[doc = "Field `PIF7` writer - Pending bit 7"]
pub use PIF0_W as PIF7_W;
#[doc = "Field `PIF8` writer - Pending bit 8"]
pub use PIF0_W as PIF8_W;
#[doc = "Field `PIF9` writer - Pending bit 9"]
pub use PIF0_W as PIF9_W;
#[doc = "Field `PIF10` writer - Pending bit 10"]
pub use PIF0_W as PIF10_W;
#[doc = "Field `PIF11` writer - Pending bit 11"]
pub use PIF0_W as PIF11_W;
#[doc = "Field `PIF12` writer - Pending bit 12"]
pub use PIF0_W as PIF12_W;
#[doc = "Field `PIF13` writer - Pending bit 13"]
pub use PIF0_W as PIF13_W;
#[doc = "Field `PIF14` writer - Pending bit 14"]
pub use PIF0_W as PIF14_W;
#[doc = "Field `PIF15` writer - Pending bit 15"]
pub use PIF0_W as PIF15_W;
#[doc = "Field `PIF16` writer - Pending bit 16"]
pub use PIF0_W as PIF16_W;
#[doc = "Field `PIF17` writer - Pending bit 17"]
pub use PIF0_W as PIF17_W;
#[doc = "Field `PIF19` writer - Pending bit 19"]
pub use PIF0_W as PIF19_W;
#[doc = "Field `PIF20` writer - Pending bit 20"]
pub use PIF0_W as PIF20_W;
#[doc = "Field `PIF21` writer - Pending bit 21"]
pub use PIF0_W as PIF21_W;
#[doc = "Field `PIF22` writer - Pending bit 22"]
pub use PIF0_W as PIF22_W;
impl R {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn pif17(&self) -> PIF17_R {
        PIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    pub fn pif19(&self) -> PIF19_R {
        PIF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    pub fn pif20(&self) -> PIF20_R {
        PIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    pub fn pif22(&self) -> PIF22_R {
        PIF22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pif0(&mut self) -> PIF0_W<PRrs> {
        PIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pif1(&mut self) -> PIF1_W<PRrs> {
        PIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pif2(&mut self) -> PIF2_W<PRrs> {
        PIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pif3(&mut self) -> PIF3_W<PRrs> {
        PIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pif4(&mut self) -> PIF4_W<PRrs> {
        PIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pif5(&mut self) -> PIF5_W<PRrs> {
        PIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pif6(&mut self) -> PIF6_W<PRrs> {
        PIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pif7(&mut self) -> PIF7_W<PRrs> {
        PIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pif8(&mut self) -> PIF8_W<PRrs> {
        PIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pif9(&mut self) -> PIF9_W<PRrs> {
        PIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pif10(&mut self) -> PIF10_W<PRrs> {
        PIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pif11(&mut self) -> PIF11_W<PRrs> {
        PIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pif12(&mut self) -> PIF12_W<PRrs> {
        PIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pif13(&mut self) -> PIF13_W<PRrs> {
        PIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pif14(&mut self) -> PIF14_W<PRrs> {
        PIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pif15(&mut self) -> PIF15_W<PRrs> {
        PIF15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pif16(&mut self) -> PIF16_W<PRrs> {
        PIF16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pif17(&mut self) -> PIF17_W<PRrs> {
        PIF17_W::new(self, 17)
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pif19(&mut self) -> PIF19_W<PRrs> {
        PIF19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pif20(&mut self) -> PIF20_W<PRrs> {
        PIF20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pif21(&mut self) -> PIF21_W<PRrs> {
        PIF21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pif22(&mut self) -> PIF22_W<PRrs> {
        PIF22_W::new(self, 22)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x007b_ffff;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PRrs {
    const RESET_VALUE: u32 = 0;
}
