///Register `IMR2` reader
pub type R = crate::R<IMR2rs>;
///Register `IMR2` writer
pub type W = crate::W<IMR2rs>;
/**Interrupt Mask on external/internal line 32

Value on reset: 1*/
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
///Field `IM32` reader - Interrupt Mask on external/internal line 32
pub type IM32_R = crate::BitReader<INTERRUPT_MASK>;
impl IM32_R {
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
///Field `IM32` writer - Interrupt Mask on external/internal line 32
pub type IM32_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> IM32_W<'a, REG>
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
///Field `IM33` reader - Interrupt Mask on external/internal line 33
pub use IM32_R as IM33_R;
///Field `IM34` reader - Interrupt Mask on external/internal line 34
pub use IM32_R as IM34_R;
///Field `IM35` reader - Interrupt Mask on external/internal line 35
pub use IM32_R as IM35_R;
///Field `IM36` reader - Interrupt Mask on external/internal line 36
pub use IM32_R as IM36_R;
///Field `IM37` reader - Interrupt Mask on external/internal line 37
pub use IM32_R as IM37_R;
///Field `IM40` reader - Interrupt Mask on external/internal line 40
pub use IM32_R as IM40_R;
///Field `IM41` reader - Interrupt Mask on external/internal line 41
pub use IM32_R as IM41_R;
///Field `IM42` reader - Interrupt Mask on external/internal line 42
pub use IM32_R as IM42_R;
///Field `IM43` reader - Interrupt Mask on external/internal line 43
pub use IM32_R as IM43_R;
///Field `IM33` writer - Interrupt Mask on external/internal line 33
pub use IM32_W as IM33_W;
///Field `IM34` writer - Interrupt Mask on external/internal line 34
pub use IM32_W as IM34_W;
///Field `IM35` writer - Interrupt Mask on external/internal line 35
pub use IM32_W as IM35_W;
///Field `IM36` writer - Interrupt Mask on external/internal line 36
pub use IM32_W as IM36_W;
///Field `IM37` writer - Interrupt Mask on external/internal line 37
pub use IM32_W as IM37_W;
///Field `IM40` writer - Interrupt Mask on external/internal line 40
pub use IM32_W as IM40_W;
///Field `IM41` writer - Interrupt Mask on external/internal line 41
pub use IM32_W as IM41_W;
///Field `IM42` writer - Interrupt Mask on external/internal line 42
pub use IM32_W as IM42_W;
///Field `IM43` writer - Interrupt Mask on external/internal line 43
pub use IM32_W as IM43_W;
impl R {
    ///Bit 0 - Interrupt Mask on external/internal line 32
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt Mask on external/internal line 33
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt Mask on external/internal line 34
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Mask on external/internal line 35
    #[inline(always)]
    pub fn im35(&self) -> IM35_R {
        IM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt Mask on external/internal line 36
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt Mask on external/internal line 37
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Interrupt Mask on external/internal line 40
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt Mask on external/internal line 41
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt Mask on external/internal line 42
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt Mask on external/internal line 43
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR2")
            .field("im32", &self.im32())
            .field("im33", &self.im33())
            .field("im34", &self.im34())
            .field("im35", &self.im35())
            .field("im36", &self.im36())
            .field("im37", &self.im37())
            .field("im40", &self.im40())
            .field("im41", &self.im41())
            .field("im42", &self.im42())
            .field("im43", &self.im43())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt Mask on external/internal line 32
    #[inline(always)]
    pub fn im32(&mut self) -> IM32_W<'_, IMR2rs> {
        IM32_W::new(self, 0)
    }
    ///Bit 1 - Interrupt Mask on external/internal line 33
    #[inline(always)]
    pub fn im33(&mut self) -> IM33_W<'_, IMR2rs> {
        IM33_W::new(self, 1)
    }
    ///Bit 2 - Interrupt Mask on external/internal line 34
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W<'_, IMR2rs> {
        IM34_W::new(self, 2)
    }
    ///Bit 3 - Interrupt Mask on external/internal line 35
    #[inline(always)]
    pub fn im35(&mut self) -> IM35_W<'_, IMR2rs> {
        IM35_W::new(self, 3)
    }
    ///Bit 4 - Interrupt Mask on external/internal line 36
    #[inline(always)]
    pub fn im36(&mut self) -> IM36_W<'_, IMR2rs> {
        IM36_W::new(self, 4)
    }
    ///Bit 5 - Interrupt Mask on external/internal line 37
    #[inline(always)]
    pub fn im37(&mut self) -> IM37_W<'_, IMR2rs> {
        IM37_W::new(self, 5)
    }
    ///Bit 8 - Interrupt Mask on external/internal line 40
    #[inline(always)]
    pub fn im40(&mut self) -> IM40_W<'_, IMR2rs> {
        IM40_W::new(self, 8)
    }
    ///Bit 9 - Interrupt Mask on external/internal line 41
    #[inline(always)]
    pub fn im41(&mut self) -> IM41_W<'_, IMR2rs> {
        IM41_W::new(self, 9)
    }
    ///Bit 10 - Interrupt Mask on external/internal line 42
    #[inline(always)]
    pub fn im42(&mut self) -> IM42_W<'_, IMR2rs> {
        IM42_W::new(self, 10)
    }
    ///Bit 11 - Interrupt Mask on external/internal line 43
    #[inline(always)]
    pub fn im43(&mut self) -> IM43_W<'_, IMR2rs> {
        IM43_W::new(self, 11)
    }
}
/**Interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#EXTI:IMR2)*/
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
///`read()` method returns [`imr2::R`](R) reader structure
impl crate::Readable for IMR2rs {}
///`write(|w| ..)` method takes [`imr2::W`](W) writer structure
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR2 to value 0xffff_ff87
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0xffff_ff87;
}
