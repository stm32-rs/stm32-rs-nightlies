///Register `ITLINE19` reader
pub type R = crate::R<ITLINE19rs>;
/**TIM14

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM14 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM14> for bool {
    #[inline(always)]
    fn from(variant: TIM14) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM14` reader - TIM14
pub type TIM14_R = crate::BitReader<TIM14>;
impl TIM14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM14 {
        match self.bits {
            false => TIM14::NotInterrupted,
            true => TIM14::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM14::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM14::Interrupted
    }
}
impl R {
    ///Bit 0 - TIM14
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE19")
            .field("tim14", &self.tim14())
            .finish()
    }
}
/**interrupt line 19 status register

You can [`read`](crate::Reg::read) this register and get [`itline19::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#SYSCFG:ITLINE19)*/
pub struct ITLINE19rs;
impl crate::RegisterSpec for ITLINE19rs {
    type Ux = u32;
}
///`read()` method returns [`itline19::R`](R) reader structure
impl crate::Readable for ITLINE19rs {}
///`reset()` method sets ITLINE19 to value 0
impl crate::Resettable for ITLINE19rs {}
