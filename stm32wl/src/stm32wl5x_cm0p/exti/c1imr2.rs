///Register `C1IMR2` reader
pub type R = crate::R<C1IMR2rs>;
///Register `C1IMR2` writer
pub type W = crate::W<C1IMR2rs>;
/**Wake-up with interrupt mask on event input 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_MASK {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<INTERRUPT_MASK> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `IM34` reader - Wake-up with interrupt mask on event input 34
pub type IM34_R = crate::BitReader<INTERRUPT_MASK>;
impl IM34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_MASK {
        match self.bits {
            false => INTERRUPT_MASK::Masked,
            true => INTERRUPT_MASK::Unmasked,
        }
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == INTERRUPT_MASK::Masked
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == INTERRUPT_MASK::Unmasked
    }
}
///Field `IM34` writer - Wake-up with interrupt mask on event input 34
pub type IM34_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> IM34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Unmasked)
    }
}
///Field `IM36` reader - Wake-up with interrupt mask on event input 36
pub use IM34_R as IM36_R;
///Field `IM37` reader - Wake-up with interrupt mask on event input 37
pub use IM34_R as IM37_R;
///Field `IM38` reader - Wake-up with interrupt mask on event input 38
pub use IM34_R as IM38_R;
///Field `IM39` reader - Wake-up with interrupt mask on event input 39
pub use IM34_R as IM39_R;
///Field `IM40` reader - Wake-up with interrupt mask on event input 40
pub use IM34_R as IM40_R;
///Field `IM41` reader - Wake-up with interrupt mask on event input 41
pub use IM34_R as IM41_R;
///Field `IM42` reader - Wake-up with interrupt mask on event input 42
pub use IM34_R as IM42_R;
///Field `IM43` reader - Wake-up with interrupt mask on event input 43
pub use IM34_R as IM43_R;
///Field `IM44` reader - Wake-up with interrupt mask on event input 44
pub use IM34_R as IM44_R;
///Field `IM45` reader - Wake-up with interrupt mask on event input 45
pub use IM34_R as IM45_R;
///Field `IM46` reader - Wake-up with interrupt mask on event input 46
pub use IM34_R as IM46_R;
///Field `IM36` writer - Wake-up with interrupt mask on event input 36
pub use IM34_W as IM36_W;
///Field `IM37` writer - Wake-up with interrupt mask on event input 37
pub use IM34_W as IM37_W;
///Field `IM38` writer - Wake-up with interrupt mask on event input 38
pub use IM34_W as IM38_W;
///Field `IM39` writer - Wake-up with interrupt mask on event input 39
pub use IM34_W as IM39_W;
///Field `IM40` writer - Wake-up with interrupt mask on event input 40
pub use IM34_W as IM40_W;
///Field `IM41` writer - Wake-up with interrupt mask on event input 41
pub use IM34_W as IM41_W;
///Field `IM42` writer - Wake-up with interrupt mask on event input 42
pub use IM34_W as IM42_W;
///Field `IM43` writer - Wake-up with interrupt mask on event input 43
pub use IM34_W as IM43_W;
///Field `IM44` writer - Wake-up with interrupt mask on event input 44
pub use IM34_W as IM44_W;
///Field `IM45` writer - Wake-up with interrupt mask on event input 45
pub use IM34_W as IM45_W;
///Field `IM46` writer - Wake-up with interrupt mask on event input 46
pub use IM34_W as IM46_W;
impl R {
    ///Bit 2 - Wake-up with interrupt mask on event input 34
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Wake-up with interrupt mask on event input 36
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wake-up with interrupt mask on event input 37
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wake-up with interrupt mask on event input 38
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wake-up with interrupt mask on event input 39
    #[inline(always)]
    pub fn im39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Wake-up with interrupt mask on event input 40
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wake-up with interrupt mask on event input 41
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wake-up with interrupt mask on event input 42
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Wake-up with interrupt mask on event input 43
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wake-up with interrupt mask on event input 44
    #[inline(always)]
    pub fn im44(&self) -> IM44_R {
        IM44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wake-up with interrupt mask on event input 45
    #[inline(always)]
    pub fn im45(&self) -> IM45_R {
        IM45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wake-up with interrupt mask on event input 46
    #[inline(always)]
    pub fn im46(&self) -> IM46_R {
        IM46_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1IMR2")
            .field("im34", &self.im34())
            .field("im36", &self.im36())
            .field("im37", &self.im37())
            .field("im38", &self.im38())
            .field("im39", &self.im39())
            .field("im40", &self.im40())
            .field("im41", &self.im41())
            .field("im42", &self.im42())
            .field("im43", &self.im43())
            .field("im44", &self.im44())
            .field("im45", &self.im45())
            .field("im46", &self.im46())
            .finish()
    }
}
impl W {
    ///Bit 2 - Wake-up with interrupt mask on event input 34
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W<'_, C1IMR2rs> {
        IM34_W::new(self, 2)
    }
    ///Bit 4 - Wake-up with interrupt mask on event input 36
    #[inline(always)]
    pub fn im36(&mut self) -> IM36_W<'_, C1IMR2rs> {
        IM36_W::new(self, 4)
    }
    ///Bit 5 - Wake-up with interrupt mask on event input 37
    #[inline(always)]
    pub fn im37(&mut self) -> IM37_W<'_, C1IMR2rs> {
        IM37_W::new(self, 5)
    }
    ///Bit 6 - Wake-up with interrupt mask on event input 38
    #[inline(always)]
    pub fn im38(&mut self) -> IM38_W<'_, C1IMR2rs> {
        IM38_W::new(self, 6)
    }
    ///Bit 7 - Wake-up with interrupt mask on event input 39
    #[inline(always)]
    pub fn im39(&mut self) -> IM39_W<'_, C1IMR2rs> {
        IM39_W::new(self, 7)
    }
    ///Bit 8 - Wake-up with interrupt mask on event input 40
    #[inline(always)]
    pub fn im40(&mut self) -> IM40_W<'_, C1IMR2rs> {
        IM40_W::new(self, 8)
    }
    ///Bit 9 - Wake-up with interrupt mask on event input 41
    #[inline(always)]
    pub fn im41(&mut self) -> IM41_W<'_, C1IMR2rs> {
        IM41_W::new(self, 9)
    }
    ///Bit 10 - Wake-up with interrupt mask on event input 42
    #[inline(always)]
    pub fn im42(&mut self) -> IM42_W<'_, C1IMR2rs> {
        IM42_W::new(self, 10)
    }
    ///Bit 11 - Wake-up with interrupt mask on event input 43
    #[inline(always)]
    pub fn im43(&mut self) -> IM43_W<'_, C1IMR2rs> {
        IM43_W::new(self, 11)
    }
    ///Bit 12 - Wake-up with interrupt mask on event input 44
    #[inline(always)]
    pub fn im44(&mut self) -> IM44_W<'_, C1IMR2rs> {
        IM44_W::new(self, 12)
    }
    ///Bit 13 - Wake-up with interrupt mask on event input 45
    #[inline(always)]
    pub fn im45(&mut self) -> IM45_W<'_, C1IMR2rs> {
        IM45_W::new(self, 13)
    }
    ///Bit 14 - Wake-up with interrupt mask on event input 46
    #[inline(always)]
    pub fn im46(&mut self) -> IM46_W<'_, C1IMR2rs> {
        IM46_W::new(self, 14)
    }
}
/**EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c1imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#EXTI:C1IMR2)*/
pub struct C1IMR2rs;
impl crate::RegisterSpec for C1IMR2rs {
    type Ux = u32;
}
///`read()` method returns [`c1imr2::R`](R) reader structure
impl crate::Readable for C1IMR2rs {}
///`write(|w| ..)` method takes [`c1imr2::W`](W) writer structure
impl crate::Writable for C1IMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1IMR2 to value 0
impl crate::Resettable for C1IMR2rs {}
