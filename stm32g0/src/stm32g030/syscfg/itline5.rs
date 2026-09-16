///Register `ITLINE5` reader
pub type R = crate::R<ITLINE5rs>;
/**EXTI0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTI0 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<EXTI0> for bool {
    #[inline(always)]
    fn from(variant: EXTI0) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTI0` reader - EXTI0
pub type EXTI0_R = crate::BitReader<EXTI0>;
impl EXTI0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTI0 {
        match self.bits {
            false => EXTI0::NotInterrupted,
            true => EXTI0::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == EXTI0::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == EXTI0::Interrupted
    }
}
///Field `EXTI1` reader - EXTI1
pub use EXTI0_R as EXTI1_R;
impl R {
    ///Bit 0 - EXTI0
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EXTI1
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE5")
            .field("exti0", &self.exti0())
            .field("exti1", &self.exti1())
            .finish()
    }
}
/**interrupt line 5 status register

You can [`read`](crate::Reg::read) this register and get [`itline5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G030.html#SYSCFG:ITLINE5)*/
pub struct ITLINE5rs;
impl crate::RegisterSpec for ITLINE5rs {
    type Ux = u32;
}
///`read()` method returns [`itline5::R`](R) reader structure
impl crate::Readable for ITLINE5rs {}
///`reset()` method sets ITLINE5 to value 0
impl crate::Resettable for ITLINE5rs {}
