///Register `ITLINE0` reader
pub type R = crate::R<ITLINE0rs>;
/**Window watchdog interrupt pending flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDG {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<WWDG> for bool {
    #[inline(always)]
    fn from(variant: WWDG) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDG` reader - Window watchdog interrupt pending flag
pub type WWDG_R = crate::BitReader<WWDG>;
impl WWDG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDG {
        match self.bits {
            false => WWDG::NotInterrupted,
            true => WWDG::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == WWDG::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == WWDG::Interrupted
    }
}
impl R {
    ///Bit 0 - Window watchdog interrupt pending flag
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE0")
            .field("wwdg", &self.wwdg())
            .finish()
    }
}
/**interrupt line 0 status register

You can [`read`](crate::Reg::read) this register and get [`itline0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#SYSCFG:ITLINE0)*/
pub struct ITLINE0rs;
impl crate::RegisterSpec for ITLINE0rs {
    type Ux = u32;
}
///`read()` method returns [`itline0::R`](R) reader structure
impl crate::Readable for ITLINE0rs {}
///`reset()` method sets ITLINE0 to value 0
impl crate::Resettable for ITLINE0rs {}
