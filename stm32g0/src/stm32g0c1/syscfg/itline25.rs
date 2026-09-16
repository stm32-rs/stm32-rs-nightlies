///Register `ITLINE25` reader
pub type R = crate::R<ITLINE25rs>;
/**SPI1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<SPI1> for bool {
    #[inline(always)]
    fn from(variant: SPI1) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI1` reader - SPI1
pub type SPI1_R = crate::BitReader<SPI1>;
impl SPI1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI1 {
        match self.bits {
            false => SPI1::NotInterrupted,
            true => SPI1::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == SPI1::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == SPI1::Interrupted
    }
}
impl R {
    ///Bit 0 - SPI1
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE25")
            .field("spi1", &self.spi1())
            .finish()
    }
}
/**interrupt line 25 status register

You can [`read`](crate::Reg::read) this register and get [`itline25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#SYSCFG:ITLINE25)*/
pub struct ITLINE25rs;
impl crate::RegisterSpec for ITLINE25rs {
    type Ux = u32;
}
///`read()` method returns [`itline25::R`](R) reader structure
impl crate::Readable for ITLINE25rs {}
///`reset()` method sets ITLINE25 to value 0
impl crate::Resettable for ITLINE25rs {}
