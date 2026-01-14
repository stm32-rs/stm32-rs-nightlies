///Register `I2C_CR2` reader
pub type R = crate::R<I2C_CR2rs>;
///Register `I2C_CR2` writer
pub type W = crate::W<I2C_CR2rs>;
///Field `SADD` reader - Slave address
pub type SADD_R = crate::FieldReader<u16>;
///Field `SADD` writer - Slave address
pub type SADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RD_WRN` reader - Transfer direction (master mode) - 0: Master requests a write transfer. - 1: Master requests a read transfer.
pub type RD_WRN_R = crate::BitReader;
///Field `RD_WRN` writer - Transfer direction (master mode) - 0: Master requests a write transfer. - 1: Master requests a read transfer.
pub type RD_WRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD10` reader - Ten-bit addressing mode (master mode) - 0: The master operates in 7-bit addressing mode, - 1: The master operates in 10-bit addressing mode
pub type ADD10_R = crate::BitReader;
///Field `ADD10` writer - Ten-bit addressing mode (master mode) - 0: The master operates in 7-bit addressing mode, - 1: The master operates in 10-bit addressing mode
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HEAD10R` reader - Ten bit (10-bit) address header only read direction (master receiver mode) - 0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction. - 1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction.
pub type HEAD10R_R = crate::BitReader;
///Field `HEAD10R` writer - Ten bit (10-bit) address header only read direction (master receiver mode) - 0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction. - 1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction.
pub type HEAD10R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. - 0: No Start generation. - 1: Restart/Start generation: If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free.
pub type START_R = crate::BitReader;
///Field `START` writer - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. - 0: No Start generation. - 1: Restart/Start generation: If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: - 0: No Stop generation. - 1: Stop generation after current byte transfer.
pub type STOP_R = crate::BitReader;
///Field `STOP` writer - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: - 0: No Stop generation. - 1: Stop generation after current byte transfer.
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` reader - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. - 0: an ACK is sent after current received byte. - 1: a NACK is sent after current received byte.
pub type NACK_R = crate::BitReader;
///Field `NACK` writer - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. - 0: an ACK is sent after current received byte. - 1: a NACK is sent after current received byte.
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBYTES` reader - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0.
pub type NBYTES_R = crate::FieldReader;
///Field `NBYTES` writer - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0.
pub type NBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RELOAD` reader - NBYTES reload mode This bit is set and cleared by software. - 0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow). - 1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low.
pub type RELOAD_R = crate::BitReader;
///Field `RELOAD` writer - NBYTES reload mode This bit is set and cleared by software. - 0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow). - 1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low.
pub type RELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTOEND` reader - Automatic end mode (master mode) This bit is set and cleared by software. - 0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low. - 1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred.
pub type AUTOEND_R = crate::BitReader;
///Field `AUTOEND` writer - Automatic end mode (master mode) This bit is set and cleared by software. - 0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low. - 1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred.
pub type AUTOEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PECBYTE` reader - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. - 0: No PEC transfer. - 1: PEC transmission/reception is requested
pub type PECBYTE_R = crate::BitReader;
///Field `PECBYTE` writer - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. - 0: No PEC transfer. - 1: PEC transmission/reception is requested
pub type PECBYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - Slave address
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Transfer direction (master mode) - 0: Master requests a write transfer. - 1: Master requests a read transfer.
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Ten-bit addressing mode (master mode) - 0: The master operates in 7-bit addressing mode, - 1: The master operates in 10-bit addressing mode
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Ten bit (10-bit) address header only read direction (master receiver mode) - 0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction. - 1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction.
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. - 0: No Start generation. - 1: Restart/Start generation: If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: - 0: No Stop generation. - 1: Stop generation after current byte transfer.
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. - 0: an ACK is sent after current received byte. - 1: a NACK is sent after current received byte.
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0.
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - NBYTES reload mode This bit is set and cleared by software. - 0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow). - 1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low.
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. - 0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low. - 1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred.
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. - 0: No PEC transfer. - 1: PEC transmission/reception is requested
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CR2")
            .field("sadd", &self.sadd())
            .field("rd_wrn", &self.rd_wrn())
            .field("add10", &self.add10())
            .field("head10r", &self.head10r())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("nack", &self.nack())
            .field("nbytes", &self.nbytes())
            .field("reload", &self.reload())
            .field("autoend", &self.autoend())
            .field("pecbyte", &self.pecbyte())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Slave address
    #[inline(always)]
    pub fn sadd(&mut self) -> SADD_W<'_, I2C_CR2rs> {
        SADD_W::new(self, 0)
    }
    ///Bit 10 - Transfer direction (master mode) - 0: Master requests a write transfer. - 1: Master requests a read transfer.
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<'_, I2C_CR2rs> {
        RD_WRN_W::new(self, 10)
    }
    ///Bit 11 - Ten-bit addressing mode (master mode) - 0: The master operates in 7-bit addressing mode, - 1: The master operates in 10-bit addressing mode
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W<'_, I2C_CR2rs> {
        ADD10_W::new(self, 11)
    }
    ///Bit 12 - Ten bit (10-bit) address header only read direction (master receiver mode) - 0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction. - 1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction.
    #[inline(always)]
    pub fn head10r(&mut self) -> HEAD10R_W<'_, I2C_CR2rs> {
        HEAD10R_W::new(self, 12)
    }
    ///Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. - 0: No Start generation. - 1: Restart/Start generation: If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, I2C_CR2rs> {
        START_W::new(self, 13)
    }
    ///Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: - 0: No Stop generation. - 1: Stop generation after current byte transfer.
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, I2C_CR2rs> {
        STOP_W::new(self, 14)
    }
    ///Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. - 0: an ACK is sent after current received byte. - 1: a NACK is sent after current received byte.
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<'_, I2C_CR2rs> {
        NACK_W::new(self, 15)
    }
    ///Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0.
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W<'_, I2C_CR2rs> {
        NBYTES_W::new(self, 16)
    }
    ///Bit 24 - NBYTES reload mode This bit is set and cleared by software. - 0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow). - 1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low.
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<'_, I2C_CR2rs> {
        RELOAD_W::new(self, 24)
    }
    ///Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. - 0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low. - 1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred.
    #[inline(always)]
    pub fn autoend(&mut self) -> AUTOEND_W<'_, I2C_CR2rs> {
        AUTOEND_W::new(self, 25)
    }
    ///Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. - 0: No PEC transfer. - 1: PEC transmission/reception is requested
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PECBYTE_W<'_, I2C_CR2rs> {
        PECBYTE_W::new(self, 26)
    }
}
/**I2C_CR2 register

You can [`read`](crate::Reg::read) this register and get [`i2c_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_CR2)*/
pub struct I2C_CR2rs;
impl crate::RegisterSpec for I2C_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`i2c_cr2::R`](R) reader structure
impl crate::Readable for I2C_CR2rs {}
///`write(|w| ..)` method takes [`i2c_cr2::W`](W) writer structure
impl crate::Writable for I2C_CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_CR2 to value 0
impl crate::Resettable for I2C_CR2rs {}
