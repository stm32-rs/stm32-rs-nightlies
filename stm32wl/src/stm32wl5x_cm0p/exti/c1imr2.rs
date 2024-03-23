#[doc = "Register `C1IMR2` reader"]
pub type R = crate::R<C1IMR2rs>;
#[doc = "Register `C1IMR2` writer"]
pub type W = crate::W<C1IMR2rs>;
#[doc = "wakeup with interrupt mask on event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM34 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM34> for bool {
    #[inline(always)]
    fn from(variant: IM34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM34` reader - wakeup with interrupt mask on event input"]
pub type IM34_R = crate::BitReader<IM34>;
impl IM34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM34 {
        match self.bits {
            false => IM34::Masked,
            true => IM34::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM34::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM34::Unmasked
    }
}
#[doc = "Field `IM34` writer - wakeup with interrupt mask on event input"]
pub type IM34_W<'a, REG> = crate::BitWriter<'a, REG, IM34>;
impl<'a, REG> IM34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(IM34::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(IM34::Unmasked)
    }
}
#[doc = "Field `IM36` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM36_R;
#[doc = "Field `IM37` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM37_R;
#[doc = "Field `IM38` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM38_R;
#[doc = "Field `IM39` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM39_R;
#[doc = "Field `IM40` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM40_R;
#[doc = "Field `IM41` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM41_R;
#[doc = "Field `IM42` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM42_R;
#[doc = "Field `IM43` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM43_R;
#[doc = "Field `IM44` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM44_R;
#[doc = "Field `IM45` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM45_R;
#[doc = "Field `IM46` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM46_R;
#[doc = "Field `IM36` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM36_W;
#[doc = "Field `IM37` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM37_W;
#[doc = "Field `IM38` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM38_W;
#[doc = "Field `IM39` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM39_W;
#[doc = "Field `IM40` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM40_W;
#[doc = "Field `IM41` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM41_W;
#[doc = "Field `IM42` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM42_W;
#[doc = "Field `IM43` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM43_W;
#[doc = "Field `IM44` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM44_W;
#[doc = "Field `IM45` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM45_W;
#[doc = "Field `IM46` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM46_W;
impl R {
    #[doc = "Bit 2 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im44(&self) -> IM44_R {
        IM44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im45(&self) -> IM45_R {
        IM45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im46(&self) -> IM46_R {
        IM46_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im34(&mut self) -> IM34_W<C1IMR2rs> {
        IM34_W::new(self, 2)
    }
    #[doc = "Bit 4 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im36(&mut self) -> IM36_W<C1IMR2rs> {
        IM36_W::new(self, 4)
    }
    #[doc = "Bit 5 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im37(&mut self) -> IM37_W<C1IMR2rs> {
        IM37_W::new(self, 5)
    }
    #[doc = "Bit 6 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im38(&mut self) -> IM38_W<C1IMR2rs> {
        IM38_W::new(self, 6)
    }
    #[doc = "Bit 7 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im39(&mut self) -> IM39_W<C1IMR2rs> {
        IM39_W::new(self, 7)
    }
    #[doc = "Bit 8 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im40(&mut self) -> IM40_W<C1IMR2rs> {
        IM40_W::new(self, 8)
    }
    #[doc = "Bit 9 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im41(&mut self) -> IM41_W<C1IMR2rs> {
        IM41_W::new(self, 9)
    }
    #[doc = "Bit 10 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im42(&mut self) -> IM42_W<C1IMR2rs> {
        IM42_W::new(self, 10)
    }
    #[doc = "Bit 11 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im43(&mut self) -> IM43_W<C1IMR2rs> {
        IM43_W::new(self, 11)
    }
    #[doc = "Bit 12 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im44(&mut self) -> IM44_W<C1IMR2rs> {
        IM44_W::new(self, 12)
    }
    #[doc = "Bit 13 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im45(&mut self) -> IM45_W<C1IMR2rs> {
        IM45_W::new(self, 13)
    }
    #[doc = "Bit 14 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im46(&mut self) -> IM46_W<C1IMR2rs> {
        IM46_W::new(self, 14)
    }
}
#[doc = "wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1imr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1imr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1IMR2rs;
impl crate::RegisterSpec for C1IMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1imr2::R`](R) reader structure"]
impl crate::Readable for C1IMR2rs {}
#[doc = "`write(|w| ..)` method takes [`c1imr2::W`](W) writer structure"]
impl crate::Writable for C1IMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1IMR2 to value 0"]
impl crate::Resettable for C1IMR2rs {
    const RESET_VALUE: u32 = 0;
}
