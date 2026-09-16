///Register `ITLINE28` reader
pub type R = crate::R<ITLINE28rs>;
/**USART2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<USART2> for bool {
    #[inline(always)]
    fn from(variant: USART2) -> Self {
        variant as u8 != 0
    }
}
///Field `USART2` reader - USART2
pub type USART2_R = crate::BitReader<USART2>;
impl USART2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART2 {
        match self.bits {
            false => USART2::NotInterrupted,
            true => USART2::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == USART2::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == USART2::Interrupted
    }
}
impl R {
    ///Bit 0 - USART2
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE28")
            .field("usart2", &self.usart2())
            .finish()
    }
}
/**interrupt line 28 status register

You can [`read`](crate::Reg::read) this register and get [`itline28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#SYSCFG:ITLINE28)*/
pub struct ITLINE28rs;
impl crate::RegisterSpec for ITLINE28rs {
    type Ux = u32;
}
///`read()` method returns [`itline28::R`](R) reader structure
impl crate::Readable for ITLINE28rs {}
///`reset()` method sets ITLINE28 to value 0
impl crate::Resettable for ITLINE28rs {}
