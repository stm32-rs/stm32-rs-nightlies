///Register `ITLINE6` reader
pub type R = crate::R<ITLINE6rs>;
/**EXTI2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTI2 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<EXTI2> for bool {
    #[inline(always)]
    fn from(variant: EXTI2) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTI2` reader - EXTI2
pub type EXTI2_R = crate::BitReader<EXTI2>;
impl EXTI2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTI2 {
        match self.bits {
            false => EXTI2::NotInterrupted,
            true => EXTI2::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == EXTI2::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == EXTI2::Interrupted
    }
}
///Field `EXTI3` reader - EXTI3
pub use EXTI2_R as EXTI3_R;
impl R {
    ///Bit 0 - EXTI2
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EXTI3
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE6")
            .field("exti2", &self.exti2())
            .field("exti3", &self.exti3())
            .finish()
    }
}
/**interrupt line 6 status register

You can [`read`](crate::Reg::read) this register and get [`itline6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G050.html#SYSCFG:ITLINE6)*/
pub struct ITLINE6rs;
impl crate::RegisterSpec for ITLINE6rs {
    type Ux = u32;
}
///`read()` method returns [`itline6::R`](R) reader structure
impl crate::Readable for ITLINE6rs {}
///`reset()` method sets ITLINE6 to value 0
impl crate::Resettable for ITLINE6rs {}
