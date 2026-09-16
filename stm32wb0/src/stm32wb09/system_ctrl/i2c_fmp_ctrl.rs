///Register `I2C_FMP_CTRL` reader
pub type R = crate::R<I2C_FMP_CTRLrs>;
///Register `I2C_FMP_CTRL` writer
pub type W = crate::W<I2C_FMP_CTRLrs>;
///Field `I2C1_PA0_FMP` reader - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed
pub type I2C1_PA0_FMP_R = crate::BitReader;
///Field `I2C1_PA0_FMP` writer - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed
pub type I2C1_PA0_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1_PA1_FMP` reader - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed
pub type I2C1_PA1_FMP_R = crate::BitReader;
///Field `I2C1_PA1_FMP` writer - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed
pub type I2C1_PA1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1_PB6_FMP` reader - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed.
pub type I2C1_PB6_FMP_R = crate::BitReader;
///Field `I2C1_PB6_FMP` writer - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed.
pub type I2C1_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1_PB7_FMP` reader - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed
pub type I2C1_PB7_FMP_R = crate::BitReader;
///Field `I2C1_PB7_FMP` writer - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed
pub type I2C1_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed
    #[inline(always)]
    pub fn i2c1_pa0_fmp(&self) -> I2C1_PA0_FMP_R {
        I2C1_PA0_FMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed
    #[inline(always)]
    pub fn i2c1_pa1_fmp(&self) -> I2C1_PA1_FMP_R {
        I2C1_PA1_FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed.
    #[inline(always)]
    pub fn i2c1_pb6_fmp(&self) -> I2C1_PB6_FMP_R {
        I2C1_PB6_FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed
    #[inline(always)]
    pub fn i2c1_pb7_fmp(&self) -> I2C1_PB7_FMP_R {
        I2C1_PB7_FMP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_FMP_CTRL")
            .field("i2c1_pa0_fmp", &self.i2c1_pa0_fmp())
            .field("i2c1_pa1_fmp", &self.i2c1_pa1_fmp())
            .field("i2c1_pb6_fmp", &self.i2c1_pb6_fmp())
            .field("i2c1_pb7_fmp", &self.i2c1_pb7_fmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed
    #[inline(always)]
    pub fn i2c1_pa0_fmp(&mut self) -> I2C1_PA0_FMP_W<'_, I2C_FMP_CTRLrs> {
        I2C1_PA0_FMP_W::new(self, 0)
    }
    ///Bit 1 - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed
    #[inline(always)]
    pub fn i2c1_pa1_fmp(&mut self) -> I2C1_PA1_FMP_W<'_, I2C_FMP_CTRLrs> {
        I2C1_PA1_FMP_W::new(self, 1)
    }
    ///Bit 2 - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed.
    #[inline(always)]
    pub fn i2c1_pb6_fmp(&mut self) -> I2C1_PB6_FMP_W<'_, I2C_FMP_CTRLrs> {
        I2C1_PB6_FMP_W::new(self, 2)
    }
    ///Bit 3 - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed
    #[inline(always)]
    pub fn i2c1_pb7_fmp(&mut self) -> I2C1_PB7_FMP_W<'_, I2C_FMP_CTRLrs> {
        I2C1_PB7_FMP_W::new(self, 3)
    }
}
/**I2C_FMP_CTRL register

You can [`read`](crate::Reg::read) this register and get [`i2c_fmp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fmp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#SYSTEM_CTRL:I2C_FMP_CTRL)*/
pub struct I2C_FMP_CTRLrs;
impl crate::RegisterSpec for I2C_FMP_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_fmp_ctrl::R`](R) reader structure
impl crate::Readable for I2C_FMP_CTRLrs {}
///`write(|w| ..)` method takes [`i2c_fmp_ctrl::W`](W) writer structure
impl crate::Writable for I2C_FMP_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_FMP_CTRL to value 0
impl crate::Resettable for I2C_FMP_CTRLrs {}
