///Register `ITLINE16` reader
pub type R = crate::R<ITLINE16rs>;
/**TIM3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM3> for bool {
    #[inline(always)]
    fn from(variant: TIM3) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM3` reader - TIM3
pub type TIM3_R = crate::BitReader<TIM3>;
impl TIM3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM3 {
        match self.bits {
            false => TIM3::NotInterrupted,
            true => TIM3::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM3::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM3::Interrupted
    }
}
///Field `TIM4` reader - TIM4
pub use TIM3_R as TIM4_R;
impl R {
    ///Bit 0 - TIM3
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM4
    #[inline(always)]
    pub fn tim4(&self) -> TIM4_R {
        TIM4_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE16")
            .field("tim3", &self.tim3())
            .field("tim4", &self.tim4())
            .finish()
    }
}
/**interrupt line 16 status register

You can [`read`](crate::Reg::read) this register and get [`itline16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#SYSCFG:ITLINE16)*/
pub struct ITLINE16rs;
impl crate::RegisterSpec for ITLINE16rs {
    type Ux = u32;
}
///`read()` method returns [`itline16::R`](R) reader structure
impl crate::Readable for ITLINE16rs {}
///`reset()` method sets ITLINE16 to value 0
impl crate::Resettable for ITLINE16rs {}
