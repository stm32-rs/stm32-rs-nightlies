///Register `ITLINE15` reader
pub type R = crate::R<ITLINE15rs>;
/**TIM2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM2> for bool {
    #[inline(always)]
    fn from(variant: TIM2) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2` reader - TIM2
pub type TIM2_R = crate::BitReader<TIM2>;
impl TIM2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2 {
        match self.bits {
            false => TIM2::NotInterrupted,
            true => TIM2::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM2::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM2::Interrupted
    }
}
impl R {
    ///Bit 0 - TIM2
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE15")
            .field("tim2", &self.tim2())
            .finish()
    }
}
/**interrupt line 15 status register

You can [`read`](crate::Reg::read) this register and get [`itline15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#SYSCFG:ITLINE15)*/
pub struct ITLINE15rs;
impl crate::RegisterSpec for ITLINE15rs {
    type Ux = u32;
}
///`read()` method returns [`itline15::R`](R) reader structure
impl crate::Readable for ITLINE15rs {}
///`reset()` method sets ITLINE15 to value 0
impl crate::Resettable for ITLINE15rs {}
