#[doc = "Register `PR1` reader"]
pub type R = crate::R<PR1rs>;
#[doc = "Register `PR1` writer"]
pub type W = crate::W<PR1rs>;
#[doc = "Configurable event inputs Pending bit\n\nValue on reset: 0"]
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
#[doc = "Field `PIF0` reader - Configurable event inputs Pending bit"]
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
#[doc = "Configurable event inputs Pending bit\n\nValue on reset: 0"]
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
#[doc = "Field `PIF0` writer - Configurable event inputs Pending bit"]
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
#[doc = "Field `PIF1` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF1_R;
#[doc = "Field `PIF2` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF2_R;
#[doc = "Field `PIF3` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF3_R;
#[doc = "Field `PIF4` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF4_R;
#[doc = "Field `PIF5` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF5_R;
#[doc = "Field `PIF6` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF6_R;
#[doc = "Field `PIF7` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF7_R;
#[doc = "Field `PIF8` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF8_R;
#[doc = "Field `PIF9` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF9_R;
#[doc = "Field `PIF10` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF10_R;
#[doc = "Field `PIF11` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF11_R;
#[doc = "Field `PIF12` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF12_R;
#[doc = "Field `PIF13` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF13_R;
#[doc = "Field `PIF14` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF14_R;
#[doc = "Field `PIF15` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF15_R;
#[doc = "Field `PIF16` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF16_R;
#[doc = "Field `PIF21` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF21_R;
#[doc = "Field `PIF22` reader - Configurable event inputs Pending bit"]
pub use PIF0_R as PIF22_R;
#[doc = "Field `PIF1` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF1_W;
#[doc = "Field `PIF2` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF2_W;
#[doc = "Field `PIF3` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF3_W;
#[doc = "Field `PIF4` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF4_W;
#[doc = "Field `PIF5` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF5_W;
#[doc = "Field `PIF6` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF6_W;
#[doc = "Field `PIF7` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF7_W;
#[doc = "Field `PIF8` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF8_W;
#[doc = "Field `PIF9` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF9_W;
#[doc = "Field `PIF10` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF10_W;
#[doc = "Field `PIF11` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF11_W;
#[doc = "Field `PIF12` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF12_W;
#[doc = "Field `PIF13` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF13_W;
#[doc = "Field `PIF14` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF14_W;
#[doc = "Field `PIF15` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF15_W;
#[doc = "Field `PIF16` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF16_W;
#[doc = "Field `PIF21` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF21_W;
#[doc = "Field `PIF22` writer - Configurable event inputs Pending bit"]
pub use PIF0_W as PIF22_W;
impl R {
    #[doc = "Bit 0 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif22(&self) -> PIF22_R {
        PIF22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif0(&mut self) -> PIF0_W<PR1rs> {
        PIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif1(&mut self) -> PIF1_W<PR1rs> {
        PIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif2(&mut self) -> PIF2_W<PR1rs> {
        PIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif3(&mut self) -> PIF3_W<PR1rs> {
        PIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif4(&mut self) -> PIF4_W<PR1rs> {
        PIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif5(&mut self) -> PIF5_W<PR1rs> {
        PIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif6(&mut self) -> PIF6_W<PR1rs> {
        PIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif7(&mut self) -> PIF7_W<PR1rs> {
        PIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif8(&mut self) -> PIF8_W<PR1rs> {
        PIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif9(&mut self) -> PIF9_W<PR1rs> {
        PIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif10(&mut self) -> PIF10_W<PR1rs> {
        PIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif11(&mut self) -> PIF11_W<PR1rs> {
        PIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif12(&mut self) -> PIF12_W<PR1rs> {
        PIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif13(&mut self) -> PIF13_W<PR1rs> {
        PIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif14(&mut self) -> PIF14_W<PR1rs> {
        PIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif15(&mut self) -> PIF15_W<PR1rs> {
        PIF15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif16(&mut self) -> PIF16_W<PR1rs> {
        PIF16_W::new(self, 16)
    }
    #[doc = "Bit 21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif21(&mut self) -> PIF21_W<PR1rs> {
        PIF21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif22(&mut self) -> PIF22_W<PR1rs> {
        PIF22_W::new(self, 22)
    }
}
#[doc = "EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PR1rs;
impl crate::RegisterSpec for PR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1::R`](R) reader structure"]
impl crate::Readable for PR1rs {}
#[doc = "`write(|w| ..)` method takes [`pr1::W`](W) writer structure"]
impl crate::Writable for PR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0061_ffff;
}
#[doc = "`reset()` method sets PR1 to value 0"]
impl crate::Resettable for PR1rs {
    const RESET_VALUE: u32 = 0;
}
