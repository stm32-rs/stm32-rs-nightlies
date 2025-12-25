///Register `IMR2` reader
pub type R = crate::R<IMR2rs>;
///Register `IMR2` writer
pub type W = crate::W<IMR2rs>;
/**Interrupt Mask on external/internal line 32

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
///Field `MR32` reader - Interrupt Mask on external/internal line 32
pub type MR32_R = crate::BitReader<INTERRUPT_MASK>;
impl MR32_R {
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
///Field `MR32` writer - Interrupt Mask on external/internal line 32
pub type MR32_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> MR32_W<'a, REG>
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
///Field `MR33` reader - Interrupt Mask on external/internal line 33
pub use MR32_R as MR33_R;
///Field `MR34` reader - Interrupt Mask on external/internal line 34
pub use MR32_R as MR34_R;
///Field `MR35` reader - Interrupt Mask on external/internal line 35
pub use MR32_R as MR35_R;
///Field `MR33` writer - Interrupt Mask on external/internal line 33
pub use MR32_W as MR33_W;
///Field `MR34` writer - Interrupt Mask on external/internal line 34
pub use MR32_W as MR34_W;
///Field `MR35` writer - Interrupt Mask on external/internal line 35
pub use MR32_W as MR35_W;
impl R {
    ///Bit 0 - Interrupt Mask on external/internal line 32
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt Mask on external/internal line 33
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt Mask on external/internal line 34
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Mask on external/internal line 35
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR2")
            .field("mr32", &self.mr32())
            .field("mr33", &self.mr33())
            .field("mr34", &self.mr34())
            .field("mr35", &self.mr35())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt Mask on external/internal line 32
    #[inline(always)]
    pub fn mr32(&mut self) -> MR32_W<'_, IMR2rs> {
        MR32_W::new(self, 0)
    }
    ///Bit 1 - Interrupt Mask on external/internal line 33
    #[inline(always)]
    pub fn mr33(&mut self) -> MR33_W<'_, IMR2rs> {
        MR33_W::new(self, 1)
    }
    ///Bit 2 - Interrupt Mask on external/internal line 34
    #[inline(always)]
    pub fn mr34(&mut self) -> MR34_W<'_, IMR2rs> {
        MR34_W::new(self, 2)
    }
    ///Bit 3 - Interrupt Mask on external/internal line 35
    #[inline(always)]
    pub fn mr35(&mut self) -> MR35_W<'_, IMR2rs> {
        MR35_W::new(self, 3)
    }
}
/**Interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#EXTI:IMR2)*/
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
///`reset()` method sets IMR2 to value 0xffff_fffe
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0xffff_fffe;
}
