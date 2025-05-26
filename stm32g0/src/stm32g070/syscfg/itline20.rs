///Register `ITLINE20` reader
pub type R = crate::R<ITLINE20rs>;
/**TIM15

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM15 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM15> for bool {
    #[inline(always)]
    fn from(variant: TIM15) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM15` reader - TIM15
pub type TIM15_R = crate::BitReader<TIM15>;
impl TIM15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM15 {
        match self.bits {
            false => TIM15::NotInterrupted,
            true => TIM15::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM15::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM15::Interrupted
    }
}
impl R {
    ///Bit 0 - TIM15
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE20")
            .field("tim15", &self.tim15())
            .finish()
    }
}
/**interrupt line 20 status register

You can [`read`](crate::Reg::read) this register and get [`itline20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#SYSCFG:ITLINE20)*/
pub struct ITLINE20rs;
impl crate::RegisterSpec for ITLINE20rs {
    type Ux = u32;
}
///`read()` method returns [`itline20::R`](R) reader structure
impl crate::Readable for ITLINE20rs {}
///`reset()` method sets ITLINE20 to value 0
impl crate::Resettable for ITLINE20rs {}
