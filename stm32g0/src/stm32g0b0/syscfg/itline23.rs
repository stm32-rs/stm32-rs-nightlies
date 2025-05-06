///Register `ITLINE23` reader
pub type R = crate::R<ITLINE23rs>;
/**I2C1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<I2C1> for bool {
    #[inline(always)]
    fn from(variant: I2C1) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1` reader - I2C1
pub type I2C1_R = crate::BitReader<I2C1>;
impl I2C1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1 {
        match self.bits {
            false => I2C1::NotInterrupted,
            true => I2C1::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == I2C1::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == I2C1::Interrupted
    }
}
impl R {
    ///Bit 0 - I2C1
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE23")
            .field("i2c1", &self.i2c1())
            .finish()
    }
}
/**interrupt line 23 status register

You can [`read`](crate::Reg::read) this register and get [`itline23::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#SYSCFG:ITLINE23)*/
pub struct ITLINE23rs;
impl crate::RegisterSpec for ITLINE23rs {
    type Ux = u32;
}
///`read()` method returns [`itline23::R`](R) reader structure
impl crate::Readable for ITLINE23rs {}
///`reset()` method sets ITLINE23 to value 0
impl crate::Resettable for ITLINE23rs {}
