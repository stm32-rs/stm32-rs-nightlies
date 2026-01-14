///Register `ITLINE14` reader
pub type R = crate::R<ITLINE14rs>;
/**TIM1_CC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1_CC {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM1_CC> for bool {
    #[inline(always)]
    fn from(variant: TIM1_CC) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1_CC` reader - TIM1_CC
pub type TIM1_CC_R = crate::BitReader<TIM1_CC>;
impl TIM1_CC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1_CC {
        match self.bits {
            false => TIM1_CC::NotInterrupted,
            true => TIM1_CC::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM1_CC::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM1_CC::Interrupted
    }
}
impl R {
    ///Bit 0 - TIM1_CC
    #[inline(always)]
    pub fn tim1_cc(&self) -> TIM1_CC_R {
        TIM1_CC_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE14")
            .field("tim1_cc", &self.tim1_cc())
            .finish()
    }
}
/**interrupt line 14 status register

You can [`read`](crate::Reg::read) this register and get [`itline14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G050.html#SYSCFG:ITLINE14)*/
pub struct ITLINE14rs;
impl crate::RegisterSpec for ITLINE14rs {
    type Ux = u32;
}
///`read()` method returns [`itline14::R`](R) reader structure
impl crate::Readable for ITLINE14rs {}
///`reset()` method sets ITLINE14 to value 0
impl crate::Resettable for ITLINE14rs {}
