///Register `ITLINE24` reader
pub type R = crate::R<ITLINE24rs>;
/**I2C2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<I2C2> for bool {
    #[inline(always)]
    fn from(variant: I2C2) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C2` reader - I2C2
pub type I2C2_R = crate::BitReader<I2C2>;
impl I2C2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C2 {
        match self.bits {
            false => I2C2::NotInterrupted,
            true => I2C2::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == I2C2::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == I2C2::Interrupted
    }
}
impl R {
    ///Bit 0 - I2C2
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE24")
            .field("i2c2", &self.i2c2())
            .finish()
    }
}
/**interrupt line 24 status register

You can [`read`](crate::Reg::read) this register and get [`itline24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G030.html#SYSCFG:ITLINE24)*/
pub struct ITLINE24rs;
impl crate::RegisterSpec for ITLINE24rs {
    type Ux = u32;
}
///`read()` method returns [`itline24::R`](R) reader structure
impl crate::Readable for ITLINE24rs {}
///`reset()` method sets ITLINE24 to value 0
impl crate::Resettable for ITLINE24rs {}
