///Register `ITLINE22` reader
pub type R = crate::R<ITLINE22rs>;
/**TIM17

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM17> for bool {
    #[inline(always)]
    fn from(variant: TIM17) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM17` reader - TIM17
pub type TIM17_R = crate::BitReader<TIM17>;
impl TIM17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM17 {
        match self.bits {
            false => TIM17::NotInterrupted,
            true => TIM17::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM17::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM17::Interrupted
    }
}
///Field `FDCAN1_IT1` reader - FDCAN1_IT1
pub use TIM17_R as FDCAN1_IT1_R;
///Field `FDCAN2_IT1` reader - FDCAN2_IT1
pub use TIM17_R as FDCAN2_IT1_R;
impl R {
    ///Bit 0 - TIM17
    #[inline(always)]
    pub fn tim17(&self) -> TIM17_R {
        TIM17_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FDCAN1_IT1
    #[inline(always)]
    pub fn fdcan1_it1(&self) -> FDCAN1_IT1_R {
        FDCAN1_IT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FDCAN2_IT1
    #[inline(always)]
    pub fn fdcan2_it1(&self) -> FDCAN2_IT1_R {
        FDCAN2_IT1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE22")
            .field("tim17", &self.tim17())
            .field("fdcan1_it1", &self.fdcan1_it1())
            .field("fdcan2_it1", &self.fdcan2_it1())
            .finish()
    }
}
/**interrupt line 22 status register

You can [`read`](crate::Reg::read) this register and get [`itline22::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE22)*/
pub struct ITLINE22rs;
impl crate::RegisterSpec for ITLINE22rs {
    type Ux = u32;
}
///`read()` method returns [`itline22::R`](R) reader structure
impl crate::Readable for ITLINE22rs {}
///`reset()` method sets ITLINE22 to value 0
impl crate::Resettable for ITLINE22rs {}
