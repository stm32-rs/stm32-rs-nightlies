///Register `ITLINE13` reader
pub type R = crate::R<ITLINE13rs>;
/**TIM1_CCU

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1_CCU {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TIM1_CCU> for bool {
    #[inline(always)]
    fn from(variant: TIM1_CCU) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1_CCU` reader - TIM1_CCU
pub type TIM1_CCU_R = crate::BitReader<TIM1_CCU>;
impl TIM1_CCU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1_CCU {
        match self.bits {
            false => TIM1_CCU::NotInterrupted,
            true => TIM1_CCU::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TIM1_CCU::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TIM1_CCU::Interrupted
    }
}
///Field `TIM1_TRG` reader - TIM1_TRG
pub use TIM1_CCU_R as TIM1_TRG_R;
///Field `TIM1_UPD` reader - TIM1_UPD
pub use TIM1_CCU_R as TIM1_UPD_R;
///Field `TIM1_BRK` reader - TIM1_BRK
pub use TIM1_CCU_R as TIM1_BRK_R;
impl R {
    ///Bit 0 - TIM1_CCU
    #[inline(always)]
    pub fn tim1_ccu(&self) -> TIM1_CCU_R {
        TIM1_CCU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM1_TRG
    #[inline(always)]
    pub fn tim1_trg(&self) -> TIM1_TRG_R {
        TIM1_TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM1_UPD
    #[inline(always)]
    pub fn tim1_upd(&self) -> TIM1_UPD_R {
        TIM1_UPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM1_BRK
    #[inline(always)]
    pub fn tim1_brk(&self) -> TIM1_BRK_R {
        TIM1_BRK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE13")
            .field("tim1_ccu", &self.tim1_ccu())
            .field("tim1_trg", &self.tim1_trg())
            .field("tim1_upd", &self.tim1_upd())
            .field("tim1_brk", &self.tim1_brk())
            .finish()
    }
}
/**interrupt line 13 status register

You can [`read`](crate::Reg::read) this register and get [`itline13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G050.html#SYSCFG:ITLINE13)*/
pub struct ITLINE13rs;
impl crate::RegisterSpec for ITLINE13rs {
    type Ux = u32;
}
///`read()` method returns [`itline13::R`](R) reader structure
impl crate::Readable for ITLINE13rs {}
///`reset()` method sets ITLINE13 to value 0
impl crate::Resettable for ITLINE13rs {}
