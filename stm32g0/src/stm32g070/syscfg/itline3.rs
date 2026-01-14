///Register `ITLINE3` reader
pub type R = crate::R<ITLINE3rs>;
/**FLASH_ITF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASH_ITF {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<FLASH_ITF> for bool {
    #[inline(always)]
    fn from(variant: FLASH_ITF) -> Self {
        variant as u8 != 0
    }
}
///Field `FLASH_ITF` reader - FLASH_ITF
pub type FLASH_ITF_R = crate::BitReader<FLASH_ITF>;
impl FLASH_ITF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLASH_ITF {
        match self.bits {
            false => FLASH_ITF::NotInterrupted,
            true => FLASH_ITF::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == FLASH_ITF::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == FLASH_ITF::Interrupted
    }
}
///Field `FLASH_ECC` reader - FLASH_ECC
pub use FLASH_ITF_R as FLASH_ECC_R;
impl R {
    ///Bit 0 - FLASH_ITF
    #[inline(always)]
    pub fn flash_itf(&self) -> FLASH_ITF_R {
        FLASH_ITF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FLASH_ECC
    #[inline(always)]
    pub fn flash_ecc(&self) -> FLASH_ECC_R {
        FLASH_ECC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE3")
            .field("flash_itf", &self.flash_itf())
            .field("flash_ecc", &self.flash_ecc())
            .finish()
    }
}
/**interrupt line 3 status register

You can [`read`](crate::Reg::read) this register and get [`itline3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#SYSCFG:ITLINE3)*/
pub struct ITLINE3rs;
impl crate::RegisterSpec for ITLINE3rs {
    type Ux = u32;
}
///`read()` method returns [`itline3::R`](R) reader structure
impl crate::Readable for ITLINE3rs {}
///`reset()` method sets ITLINE3 to value 0
impl crate::Resettable for ITLINE3rs {}
