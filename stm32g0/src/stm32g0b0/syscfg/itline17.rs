///Register `ITLINE17` reader
pub type R = crate::R<ITLINE17rs>;
/**TIM6

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM6> for bool {
    #[inline(always)]
    fn from(variant: TIM6) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM6` reader - TIM6
pub type TIM6_R = crate::BitReader<TIM6>;
impl TIM6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM6 {
        match self.bits {
            false => TIM6::NotInterrupted,
            true => TIM6::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM6::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM6::Interrupted
    }
}
impl R {
    ///Bit 0 - TIM6
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE17")
            .field("tim6", &self.tim6())
            .finish()
    }
}
/**interrupt line 17 status register

You can [`read`](crate::Reg::read) this register and get [`itline17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#SYSCFG:ITLINE17)*/
pub struct ITLINE17rs;
impl crate::RegisterSpec for ITLINE17rs {
    type Ux = u32;
}
///`read()` method returns [`itline17::R`](R) reader structure
impl crate::Readable for ITLINE17rs {}
///`reset()` method sets ITLINE17 to value 0
impl crate::Resettable for ITLINE17rs {}
