///Register `I2C_TXDR` reader
pub type R = crate::R<I2C_TXDRrs>;
///Register `I2C_TXDR` writer
pub type W = crate::W<I2C_TXDRrs>;
///Field `TXDATA` reader - Eight bits (8-bit) transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1.
pub type TXDATA_R = crate::FieldReader;
///Field `TXDATA` writer - Eight bits (8-bit) transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1.
pub type TXDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Eight bits (8-bit) transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1.
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_TXDR")
            .field("txdata", &self.txdata())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Eight bits (8-bit) transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1.
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W<'_, I2C_TXDRrs> {
        TXDATA_W::new(self, 0)
    }
}
/**I2C_TXDR register

You can [`read`](crate::Reg::read) this register and get [`i2c_txdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_txdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C2:I2C_TXDR)*/
pub struct I2C_TXDRrs;
impl crate::RegisterSpec for I2C_TXDRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_txdr::R`](R) reader structure
impl crate::Readable for I2C_TXDRrs {}
///`write(|w| ..)` method takes [`i2c_txdr::W`](W) writer structure
impl crate::Writable for I2C_TXDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_TXDR to value 0
impl crate::Resettable for I2C_TXDRrs {}
