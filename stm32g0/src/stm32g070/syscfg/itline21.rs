///Register `ITLINE21` reader
pub type R = crate::R<ITLINE21rs>;
/**TIM16

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM16> for bool {
    #[inline(always)]
    fn from(variant: TIM16) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM16` reader - TIM16
pub type TIM16_R = crate::BitReader<TIM16>;
impl TIM16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM16 {
        match self.bits {
            false => TIM16::NotInterrupted,
            true => TIM16::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM16::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM16::Interrupted
    }
}
impl R {
    ///Bit 0 - TIM16
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE21")
            .field("tim16", &self.tim16())
            .finish()
    }
}
/**interrupt line 21 status register

You can [`read`](crate::Reg::read) this register and get [`itline21::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#SYSCFG:ITLINE21)*/
pub struct ITLINE21rs;
impl crate::RegisterSpec for ITLINE21rs {
    type Ux = u32;
}
///`read()` method returns [`itline21::R`](R) reader structure
impl crate::Readable for ITLINE21rs {}
///`reset()` method sets ITLINE21 to value 0
impl crate::Resettable for ITLINE21rs {}
