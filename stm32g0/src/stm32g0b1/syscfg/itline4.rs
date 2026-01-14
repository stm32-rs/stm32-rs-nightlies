///Register `ITLINE4` reader
pub type R = crate::R<ITLINE4rs>;
/**RCC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCC {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<RCC> for bool {
    #[inline(always)]
    fn from(variant: RCC) -> Self {
        variant as u8 != 0
    }
}
///Field `RCC` reader - RCC
pub type RCC_R = crate::BitReader<RCC>;
impl RCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCC {
        match self.bits {
            false => RCC::NotInterrupted,
            true => RCC::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == RCC::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == RCC::Interrupted
    }
}
///Field `CRS` reader - CRS
pub use RCC_R as CRS_R;
impl R {
    ///Bit 0 - RCC
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CRS
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE4")
            .field("rcc", &self.rcc())
            .field("crs", &self.crs())
            .finish()
    }
}
/**interrupt line 4 status register

You can [`read`](crate::Reg::read) this register and get [`itline4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#SYSCFG:ITLINE4)*/
pub struct ITLINE4rs;
impl crate::RegisterSpec for ITLINE4rs {
    type Ux = u32;
}
///`read()` method returns [`itline4::R`](R) reader structure
impl crate::Readable for ITLINE4rs {}
///`reset()` method sets ITLINE4 to value 0
impl crate::Resettable for ITLINE4rs {}
