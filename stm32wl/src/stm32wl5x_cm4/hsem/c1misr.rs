///Register `C1MISR` reader
pub type R = crate::R<C1MISRrs>;
/**masked interrupt(N) semaphore n status bit after enable (mask)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISF0 {
    ///0: No interrupt pending after masking
    NotPending = 0,
    ///1: Interrupt pending after masking
    Pending = 1,
}
impl From<MISF0> for bool {
    #[inline(always)]
    fn from(variant: MISF0) -> Self {
        variant as u8 != 0
    }
}
///Field `MISF0` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub type MISF0_R = crate::BitReader<MISF0>;
impl MISF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MISF0 {
        match self.bits {
            false => MISF0::NotPending,
            true => MISF0::Pending,
        }
    }
    ///No interrupt pending after masking
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == MISF0::NotPending
    }
    ///Interrupt pending after masking
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MISF0::Pending
    }
}
///Field `MISF1` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF1_R;
///Field `MISF2` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF2_R;
///Field `MISF3` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF3_R;
///Field `MISF4` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF4_R;
///Field `MISF5` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF5_R;
///Field `MISF6` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF6_R;
///Field `MISF7` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF7_R;
///Field `MISF8` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF8_R;
///Field `MISF9` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF9_R;
///Field `MISF10` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF10_R;
///Field `MISF11` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF11_R;
///Field `MISF12` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF12_R;
///Field `MISF13` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF13_R;
///Field `MISF14` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF14_R;
///Field `MISF15` reader - masked interrupt(N) semaphore n status bit after enable (mask)
pub use MISF0_R as MISF15_R;
impl R {
    ///Bit 0 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf0(&self) -> MISF0_R {
        MISF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf1(&self) -> MISF1_R {
        MISF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf2(&self) -> MISF2_R {
        MISF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf3(&self) -> MISF3_R {
        MISF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf4(&self) -> MISF4_R {
        MISF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf5(&self) -> MISF5_R {
        MISF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf6(&self) -> MISF6_R {
        MISF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf7(&self) -> MISF7_R {
        MISF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf8(&self) -> MISF8_R {
        MISF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf9(&self) -> MISF9_R {
        MISF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf10(&self) -> MISF10_R {
        MISF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf11(&self) -> MISF11_R {
        MISF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf12(&self) -> MISF12_R {
        MISF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf13(&self) -> MISF13_R {
        MISF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf14(&self) -> MISF14_R {
        MISF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - masked interrupt(N) semaphore n status bit after enable (mask)
    #[inline(always)]
    pub fn misf15(&self) -> MISF15_R {
        MISF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1MISR")
            .field("misf0", &self.misf0())
            .field("misf1", &self.misf1())
            .field("misf2", &self.misf2())
            .field("misf3", &self.misf3())
            .field("misf4", &self.misf4())
            .field("misf5", &self.misf5())
            .field("misf6", &self.misf6())
            .field("misf7", &self.misf7())
            .field("misf8", &self.misf8())
            .field("misf9", &self.misf9())
            .field("misf10", &self.misf10())
            .field("misf11", &self.misf11())
            .field("misf12", &self.misf12())
            .field("misf13", &self.misf13())
            .field("misf14", &self.misf14())
            .field("misf15", &self.misf15())
            .finish()
    }
}
/**HSEM Masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#HSEM:C1MISR)*/
pub struct C1MISRrs;
impl crate::RegisterSpec for C1MISRrs {
    type Ux = u32;
}
///`read()` method returns [`c1misr::R`](R) reader structure
impl crate::Readable for C1MISRrs {}
///`reset()` method sets C1MISR to value 0
impl crate::Resettable for C1MISRrs {
    const RESET_VALUE: u32 = 0;
}
