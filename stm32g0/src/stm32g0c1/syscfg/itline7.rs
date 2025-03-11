///Register `ITLINE7` reader
pub type R = crate::R<ITLINE7rs>;
/**EXTI4

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTI4 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<EXTI4> for bool {
    #[inline(always)]
    fn from(variant: EXTI4) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTI4` reader - EXTI4
pub type EXTI4_R = crate::BitReader<EXTI4>;
impl EXTI4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTI4 {
        match self.bits {
            false => EXTI4::NotInterrupted,
            true => EXTI4::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == EXTI4::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == EXTI4::Interrupted
    }
}
///Field `EXTI5` reader - EXTI5
pub use EXTI4_R as EXTI5_R;
///Field `EXTI6` reader - EXTI6
pub use EXTI4_R as EXTI6_R;
///Field `EXTI7` reader - EXTI7
pub use EXTI4_R as EXTI7_R;
///Field `EXTI8` reader - EXTI8
pub use EXTI4_R as EXTI8_R;
///Field `EXTI9` reader - EXTI9
pub use EXTI4_R as EXTI9_R;
///Field `EXTI10` reader - EXTI10
pub use EXTI4_R as EXTI10_R;
///Field `EXTI11` reader - EXTI11
pub use EXTI4_R as EXTI11_R;
///Field `EXTI12` reader - EXTI12
pub use EXTI4_R as EXTI12_R;
///Field `EXTI13` reader - EXTI13
pub use EXTI4_R as EXTI13_R;
///Field `EXTI14` reader - EXTI14
pub use EXTI4_R as EXTI14_R;
///Field `EXTI15` reader - EXTI15
pub use EXTI4_R as EXTI15_R;
impl R {
    ///Bit 0 - EXTI4
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EXTI5
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EXTI6
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EXTI7
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - EXTI8
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - EXTI9
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EXTI10
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - EXTI11
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - EXTI12
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - EXTI13
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - EXTI14
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - EXTI15
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE7")
            .field("exti4", &self.exti4())
            .field("exti5", &self.exti5())
            .field("exti6", &self.exti6())
            .field("exti7", &self.exti7())
            .field("exti8", &self.exti8())
            .field("exti9", &self.exti9())
            .field("exti10", &self.exti10())
            .field("exti11", &self.exti11())
            .field("exti12", &self.exti12())
            .field("exti13", &self.exti13())
            .field("exti14", &self.exti14())
            .field("exti15", &self.exti15())
            .finish()
    }
}
/**interrupt line 7 status register

You can [`read`](crate::Reg::read) this register and get [`itline7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#SYSCFG:ITLINE7)*/
pub struct ITLINE7rs;
impl crate::RegisterSpec for ITLINE7rs {
    type Ux = u32;
}
///`read()` method returns [`itline7::R`](R) reader structure
impl crate::Readable for ITLINE7rs {}
///`reset()` method sets ITLINE7 to value 0
impl crate::Resettable for ITLINE7rs {}
