#[doc = "Register `FPR1` reader"]
pub type R = crate::R<FPR1rs>;
#[doc = "Register `FPR1` writer"]
pub type W = crate::W<FPR1rs>;
#[doc = "configurable event inputs x falling edge pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF0R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<FPIF0R> for bool {
    #[inline(always)]
    fn from(variant: FPIF0R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF0` reader - configurable event inputs x falling edge pending bit."]
pub type FPIF0_R = crate::BitReader<FPIF0R>;
impl FPIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF0R {
        match self.bits {
            false => FPIF0R::NotPending,
            true => FPIF0R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == FPIF0R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FPIF0R::Pending
    }
}
#[doc = "configurable event inputs x falling edge pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF0W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<FPIF0W> for bool {
    #[inline(always)]
    fn from(variant: FPIF0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF0` writer - configurable event inputs x falling edge pending bit."]
pub type FPIF0_W<'a, REG> = crate::BitWriter1C<'a, REG, FPIF0W>;
impl<'a, REG> FPIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF0W::Clear)
    }
}
#[doc = "Field `FPIF1` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF1_R;
#[doc = "Field `FPIF2` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF2_R;
#[doc = "Field `FPIF3` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF3_R;
#[doc = "Field `FPIF4` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF4_R;
#[doc = "Field `FPIF5` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF5_R;
#[doc = "Field `FPIF6` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF6_R;
#[doc = "Field `FPIF7` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF7_R;
#[doc = "Field `FPIF8` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF8_R;
#[doc = "Field `FPIF9` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF9_R;
#[doc = "Field `FPIF10` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF10_R;
#[doc = "Field `FPIF11` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF11_R;
#[doc = "Field `FPIF12` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF12_R;
#[doc = "Field `FPIF13` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF13_R;
#[doc = "Field `FPIF14` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF14_R;
#[doc = "Field `FPIF15` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF15_R;
#[doc = "Field `FPIF16` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF16_R;
#[doc = "Field `FPIF17` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF17_R;
#[doc = "Field `FPIF18` reader - configurable event inputs x falling edge pending bit."]
pub use FPIF0_R as FPIF18_R;
#[doc = "Field `FPIF1` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF1_W;
#[doc = "Field `FPIF2` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF2_W;
#[doc = "Field `FPIF3` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF3_W;
#[doc = "Field `FPIF4` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF4_W;
#[doc = "Field `FPIF5` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF5_W;
#[doc = "Field `FPIF6` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF6_W;
#[doc = "Field `FPIF7` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF7_W;
#[doc = "Field `FPIF8` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF8_W;
#[doc = "Field `FPIF9` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF9_W;
#[doc = "Field `FPIF10` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF10_W;
#[doc = "Field `FPIF11` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF11_W;
#[doc = "Field `FPIF12` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF12_W;
#[doc = "Field `FPIF13` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF13_W;
#[doc = "Field `FPIF14` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF14_W;
#[doc = "Field `FPIF15` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF15_W;
#[doc = "Field `FPIF16` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF16_W;
#[doc = "Field `FPIF17` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF17_W;
#[doc = "Field `FPIF18` writer - configurable event inputs x falling edge pending bit."]
pub use FPIF0_W as FPIF18_W;
impl R {
    #[doc = "Bit 0 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif16(&self) -> FPIF16_R {
        FPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif17(&self) -> FPIF17_R {
        FPIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif18(&self) -> FPIF18_R {
        FPIF18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif0(&mut self) -> FPIF0_W<FPR1rs> {
        FPIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif1(&mut self) -> FPIF1_W<FPR1rs> {
        FPIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif2(&mut self) -> FPIF2_W<FPR1rs> {
        FPIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif3(&mut self) -> FPIF3_W<FPR1rs> {
        FPIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif4(&mut self) -> FPIF4_W<FPR1rs> {
        FPIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif5(&mut self) -> FPIF5_W<FPR1rs> {
        FPIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif6(&mut self) -> FPIF6_W<FPR1rs> {
        FPIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif7(&mut self) -> FPIF7_W<FPR1rs> {
        FPIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif8(&mut self) -> FPIF8_W<FPR1rs> {
        FPIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif9(&mut self) -> FPIF9_W<FPR1rs> {
        FPIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif10(&mut self) -> FPIF10_W<FPR1rs> {
        FPIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif11(&mut self) -> FPIF11_W<FPR1rs> {
        FPIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif12(&mut self) -> FPIF12_W<FPR1rs> {
        FPIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif13(&mut self) -> FPIF13_W<FPR1rs> {
        FPIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif14(&mut self) -> FPIF14_W<FPR1rs> {
        FPIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif15(&mut self) -> FPIF15_W<FPR1rs> {
        FPIF15_W::new(self, 15)
    }
    #[doc = "Bit 16 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif16(&mut self) -> FPIF16_W<FPR1rs> {
        FPIF16_W::new(self, 16)
    }
    #[doc = "Bit 17 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif17(&mut self) -> FPIF17_W<FPR1rs> {
        FPIF17_W::new(self, 17)
    }
    #[doc = "Bit 18 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn fpif18(&mut self) -> FPIF18_W<FPR1rs> {
        FPIF18_W::new(self, 18)
    }
}
#[doc = "EXTI falling edge pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPR1rs;
impl crate::RegisterSpec for FPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpr1::R`](R) reader structure"]
impl crate::Readable for FPR1rs {}
#[doc = "`write(|w| ..)` method takes [`fpr1::W`](W) writer structure"]
impl crate::Writable for FPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0007_ffff;
}
#[doc = "`reset()` method sets FPR1 to value 0"]
impl crate::Resettable for FPR1rs {
    const RESET_VALUE: u32 = 0;
}
