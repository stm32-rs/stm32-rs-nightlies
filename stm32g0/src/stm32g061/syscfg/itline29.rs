///Register `ITLINE29` reader
pub type R = crate::R<ITLINE29rs>;
/**USART3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<USART3> for bool {
    #[inline(always)]
    fn from(variant: USART3) -> Self {
        variant as u8 != 0
    }
}
///Field `USART3` reader - USART3
pub type USART3_R = crate::BitReader<USART3>;
impl USART3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART3 {
        match self.bits {
            false => USART3::NotInterrupted,
            true => USART3::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == USART3::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == USART3::Interrupted
    }
}
///Field `USART4` reader - USART4
pub use USART3_R as USART4_R;
///Field `LPUART1` reader - LPUART1
pub use USART3_R as LPUART1_R;
///Field `USART5` reader - USART5
pub use USART3_R as USART5_R;
///Field `USART6` reader - USART6
pub use USART3_R as USART6_R;
impl R {
    ///Bit 0 - USART3
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART4
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LPUART1
    #[inline(always)]
    pub fn lpuart1(&self) -> LPUART1_R {
        LPUART1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - USART5
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - USART6
    #[inline(always)]
    pub fn usart6(&self) -> USART6_R {
        USART6_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE29")
            .field("usart3", &self.usart3())
            .field("usart4", &self.usart4())
            .field("lpuart1", &self.lpuart1())
            .field("usart5", &self.usart5())
            .field("usart6", &self.usart6())
            .finish()
    }
}
/**interrupt line 29 status register

You can [`read`](crate::Reg::read) this register and get [`itline29::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#SYSCFG:ITLINE29)*/
pub struct ITLINE29rs;
impl crate::RegisterSpec for ITLINE29rs {
    type Ux = u32;
}
///`read()` method returns [`itline29::R`](R) reader structure
impl crate::Readable for ITLINE29rs {}
///`reset()` method sets ITLINE29 to value 0
impl crate::Resettable for ITLINE29rs {}
