///Register `ITLINE18` reader
pub type R = crate::R<ITLINE18rs>;
/**TIM7

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM7> for bool {
    #[inline(always)]
    fn from(variant: TIM7) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM7` reader - TIM7
pub type TIM7_R = crate::BitReader<TIM7>;
impl TIM7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM7 {
        match self.bits {
            false => TIM7::NotInterrupted,
            true => TIM7::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM7::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM7::Interrupted
    }
}
///Field `LPTIM2` reader - LPTIM2
pub use TIM7_R as LPTIM2_R;
impl R {
    ///Bit 0 - TIM7
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPTIM2
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE18")
            .field("tim7", &self.tim7())
            .field("lptim2", &self.lptim2())
            .finish()
    }
}
/**interrupt line 18 status register

You can [`read`](crate::Reg::read) this register and get [`itline18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#SYSCFG:ITLINE18)*/
pub struct ITLINE18rs;
impl crate::RegisterSpec for ITLINE18rs {
    type Ux = u32;
}
///`read()` method returns [`itline18::R`](R) reader structure
impl crate::Readable for ITLINE18rs {}
///`reset()` method sets ITLINE18 to value 0
impl crate::Resettable for ITLINE18rs {}
