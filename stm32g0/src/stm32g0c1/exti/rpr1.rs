#[doc = "Register `RPR1` reader"]
pub type R = crate::R<RPR1rs>;
#[doc = "Register `RPR1` writer"]
pub type W = crate::W<RPR1rs>;
#[doc = "Rising edge event pending for configurable line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF0R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<RPIF0R> for bool {
    #[inline(always)]
    fn from(variant: RPIF0R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF0` reader - Rising edge event pending for configurable line"]
pub type RPIF0_R = crate::BitReader<RPIF0R>;
impl RPIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF0R {
        match self.bits {
            false => RPIF0R::NotPending,
            true => RPIF0R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RPIF0R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RPIF0R::Pending
    }
}
#[doc = "Rising edge event pending for configurable line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF0W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<RPIF0W> for bool {
    #[inline(always)]
    fn from(variant: RPIF0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF0` writer - Rising edge event pending for configurable line"]
pub type RPIF0_W<'a, REG> = crate::BitWriter1C<'a, REG, RPIF0W>;
impl<'a, REG> RPIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF0W::Clear)
    }
}
#[doc = "Field `RPIF1` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF1_R;
#[doc = "Field `RPIF2` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF2_R;
#[doc = "Field `RPIF3` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF3_R;
#[doc = "Field `RPIF4` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF4_R;
#[doc = "Field `RPIF5` reader - configurable event inputs x rising edge Pending bit"]
pub use RPIF0_R as RPIF5_R;
#[doc = "Field `RPIF6` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF6_R;
#[doc = "Field `RPIF7` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF7_R;
#[doc = "Field `RPIF8` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF8_R;
#[doc = "Field `RPIF9` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF9_R;
#[doc = "Field `RPIF10` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF10_R;
#[doc = "Field `RPIF11` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF11_R;
#[doc = "Field `RPIF12` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF12_R;
#[doc = "Field `RPIF13` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF13_R;
#[doc = "Field `RPIF14` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF14_R;
#[doc = "Field `RPIF15` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF15_R;
#[doc = "Field `RPIF16` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF16_R;
#[doc = "Field `RPIF17` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF17_R;
#[doc = "Field `RPIF18` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF18_R;
#[doc = "Field `RPIF20` reader - Rising edge event pending for configurable line"]
pub use RPIF0_R as RPIF20_R;
#[doc = "Field `RPIF1` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF1_W;
#[doc = "Field `RPIF2` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF2_W;
#[doc = "Field `RPIF3` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF3_W;
#[doc = "Field `RPIF4` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF4_W;
#[doc = "Field `RPIF5` writer - configurable event inputs x rising edge Pending bit"]
pub use RPIF0_W as RPIF5_W;
#[doc = "Field `RPIF6` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF6_W;
#[doc = "Field `RPIF7` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF7_W;
#[doc = "Field `RPIF8` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF8_W;
#[doc = "Field `RPIF9` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF9_W;
#[doc = "Field `RPIF10` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF10_W;
#[doc = "Field `RPIF11` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF11_W;
#[doc = "Field `RPIF12` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF12_W;
#[doc = "Field `RPIF13` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF13_W;
#[doc = "Field `RPIF14` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF14_W;
#[doc = "Field `RPIF15` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF15_W;
#[doc = "Field `RPIF16` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF16_W;
#[doc = "Field `RPIF17` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF17_W;
#[doc = "Field `RPIF18` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF18_W;
#[doc = "Field `RPIF20` writer - Rising edge event pending for configurable line"]
pub use RPIF0_W as RPIF20_W;
impl R {
    #[doc = "Bit 0 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit"]
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif17(&self) -> RPIF17_R {
        RPIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif18(&self) -> RPIF18_R {
        RPIF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising edge event pending for configurable line"]
    #[inline(always)]
    pub fn rpif20(&self) -> RPIF20_R {
        RPIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif0(&mut self) -> RPIF0_W<RPR1rs> {
        RPIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif1(&mut self) -> RPIF1_W<RPR1rs> {
        RPIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif2(&mut self) -> RPIF2_W<RPR1rs> {
        RPIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif3(&mut self) -> RPIF3_W<RPR1rs> {
        RPIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif4(&mut self) -> RPIF4_W<RPR1rs> {
        RPIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpif5(&mut self) -> RPIF5_W<RPR1rs> {
        RPIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif6(&mut self) -> RPIF6_W<RPR1rs> {
        RPIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif7(&mut self) -> RPIF7_W<RPR1rs> {
        RPIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif8(&mut self) -> RPIF8_W<RPR1rs> {
        RPIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif9(&mut self) -> RPIF9_W<RPR1rs> {
        RPIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif10(&mut self) -> RPIF10_W<RPR1rs> {
        RPIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif11(&mut self) -> RPIF11_W<RPR1rs> {
        RPIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif12(&mut self) -> RPIF12_W<RPR1rs> {
        RPIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif13(&mut self) -> RPIF13_W<RPR1rs> {
        RPIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif14(&mut self) -> RPIF14_W<RPR1rs> {
        RPIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif15(&mut self) -> RPIF15_W<RPR1rs> {
        RPIF15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif16(&mut self) -> RPIF16_W<RPR1rs> {
        RPIF16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif17(&mut self) -> RPIF17_W<RPR1rs> {
        RPIF17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif18(&mut self) -> RPIF18_W<RPR1rs> {
        RPIF18_W::new(self, 18)
    }
    #[doc = "Bit 20 - Rising edge event pending for configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn rpif20(&mut self) -> RPIF20_W<RPR1rs> {
        RPIF20_W::new(self, 20)
    }
}
#[doc = "EXTI rising edge pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPR1rs;
impl crate::RegisterSpec for RPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr1::R`](R) reader structure"]
impl crate::Readable for RPR1rs {}
#[doc = "`write(|w| ..)` method takes [`rpr1::W`](W) writer structure"]
impl crate::Writable for RPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0017_ffff;
}
#[doc = "`reset()` method sets RPR1 to value 0"]
impl crate::Resettable for RPR1rs {
    const RESET_VALUE: u32 = 0;
}
