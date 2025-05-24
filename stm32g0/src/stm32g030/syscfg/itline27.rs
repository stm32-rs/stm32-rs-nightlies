///Register `ITLINE27` reader
pub type R = crate::R<ITLINE27rs>;
/**USART1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<USART1> for bool {
    #[inline(always)]
    fn from(variant: USART1) -> Self {
        variant as u8 != 0
    }
}
///Field `USART1` reader - USART1
pub type USART1_R = crate::BitReader<USART1>;
impl USART1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1 {
        match self.bits {
            false => USART1::NotInterrupted,
            true => USART1::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == USART1::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == USART1::Interrupted
    }
}
impl R {
    ///Bit 0 - USART1
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE27")
            .field("usart1", &self.usart1())
            .finish()
    }
}
/**interrupt line 27 status register

You can [`read`](crate::Reg::read) this register and get [`itline27::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G030.html#SYSCFG:ITLINE27)*/
pub struct ITLINE27rs;
impl crate::RegisterSpec for ITLINE27rs {
    type Ux = u32;
}
///`read()` method returns [`itline27::R`](R) reader structure
impl crate::Readable for ITLINE27rs {}
///`reset()` method sets ITLINE27 to value 0
impl crate::Resettable for ITLINE27rs {}
