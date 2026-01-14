///Register `C1IMR2` reader
pub type R = crate::R<C1IMR2rs>;
///Register `C1IMR2` writer
pub type W = crate::W<C1IMR2rs>;
/**CPUm Wakeup with interrupt Mask on Event input

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
///Field `IM34` reader - CPUm Wakeup with interrupt Mask on Event input
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
///Field `IM34` writer - CPUm Wakeup with interrupt Mask on Event input
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
///Field `IM38` reader - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_R as IM38_R;
///Field `IM42` reader - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_R as IM42_R;
///Field `IM43` reader - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_R as IM43_R;
///Field `IM44` reader - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_R as IM44_R;
///Field `IM45` reader - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_R as IM45_R;
///Field `IM46` reader - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_R as IM46_R;
///Field `IM38` writer - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_W as IM38_W;
///Field `IM42` writer - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_W as IM42_W;
///Field `IM43` writer - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_W as IM43_W;
///Field `IM44` writer - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_W as IM44_W;
///Field `IM45` writer - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_W as IM45_W;
///Field `IM46` writer - CPUm Wakeup with interrupt Mask on Event input
pub use IM34_W as IM46_W;
impl R {
    ///Bit 2 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im44(&self) -> IM44_R {
        IM44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im45(&self) -> IM45_R {
        IM45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im46(&self) -> IM46_R {
        IM46_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1IMR2")
            .field("im34", &self.im34())
            .field("im38", &self.im38())
            .field("im42", &self.im42())
            .field("im43", &self.im43())
            .field("im44", &self.im44())
            .field("im45", &self.im45())
            .field("im46", &self.im46())
            .finish()
    }
}
impl W {
    ///Bit 2 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W<'_, C1IMR2rs> {
        IM34_W::new(self, 2)
    }
    ///Bit 6 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im38(&mut self) -> IM38_W<'_, C1IMR2rs> {
        IM38_W::new(self, 6)
    }
    ///Bit 10 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im42(&mut self) -> IM42_W<'_, C1IMR2rs> {
        IM42_W::new(self, 10)
    }
    ///Bit 11 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im43(&mut self) -> IM43_W<'_, C1IMR2rs> {
        IM43_W::new(self, 11)
    }
    ///Bit 12 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im44(&mut self) -> IM44_W<'_, C1IMR2rs> {
        IM44_W::new(self, 12)
    }
    ///Bit 13 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im45(&mut self) -> IM45_W<'_, C1IMR2rs> {
        IM45_W::new(self, 13)
    }
    ///Bit 14 - CPUm Wakeup with interrupt Mask on Event input
    #[inline(always)]
    pub fn im46(&mut self) -> IM46_W<'_, C1IMR2rs> {
        IM46_W::new(self, 14)
    }
}
/**interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c1imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#EXTI:C1IMR2)*/
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
