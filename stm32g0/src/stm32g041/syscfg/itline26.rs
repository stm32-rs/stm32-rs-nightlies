///Register `ITLINE26` reader
pub type R = crate::R<ITLINE26rs>;
/**SPI2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<SPI2> for bool {
    #[inline(always)]
    fn from(variant: SPI2) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI2` reader - SPI2
pub type SPI2_R = crate::BitReader<SPI2>;
impl SPI2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI2 {
        match self.bits {
            false => SPI2::NotInterrupted,
            true => SPI2::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == SPI2::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == SPI2::Interrupted
    }
}
///Field `SPI3` reader - SPI3
pub use SPI2_R as SPI3_R;
impl R {
    ///Bit 0 - SPI2
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 1) != 0)
    }
    ///Bit 14 - SPI3
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE26")
            .field("spi2", &self.spi2())
            .field("spi3", &self.spi3())
            .finish()
    }
}
/**interrupt line 26 status register

You can [`read`](crate::Reg::read) this register and get [`itline26::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#SYSCFG:ITLINE26)*/
pub struct ITLINE26rs;
impl crate::RegisterSpec for ITLINE26rs {
    type Ux = u32;
}
///`read()` method returns [`itline26::R`](R) reader structure
impl crate::Readable for ITLINE26rs {}
///`reset()` method sets ITLINE26 to value 0
impl crate::Resettable for ITLINE26rs {}
