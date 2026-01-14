///Register `ITLINE31` reader
pub type R = crate::R<ITLINE31rs>;
/**RNG

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNG {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<RNG> for bool {
    #[inline(always)]
    fn from(variant: RNG) -> Self {
        variant as u8 != 0
    }
}
///Field `RNG` reader - RNG
pub type RNG_R = crate::BitReader<RNG>;
impl RNG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNG {
        match self.bits {
            false => RNG::NotInterrupted,
            true => RNG::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == RNG::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == RNG::Interrupted
    }
}
///Field `AES` reader - AES
pub use RNG_R as AES_R;
impl R {
    ///Bit 0 - RNG
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AES
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE31")
            .field("rng", &self.rng())
            .field("aes", &self.aes())
            .finish()
    }
}
/**interrupt line 25 status register

You can [`read`](crate::Reg::read) this register and get [`itline31::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#SYSCFG:ITLINE31)*/
pub struct ITLINE31rs;
impl crate::RegisterSpec for ITLINE31rs {
    type Ux = u32;
}
///`read()` method returns [`itline31::R`](R) reader structure
impl crate::Readable for ITLINE31rs {}
///`reset()` method sets ITLINE31 to value 0
impl crate::Resettable for ITLINE31rs {}
