///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**Field `SADD` reader - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\[7:1\]
must be written with the 7-bit slave address to be sent. Bits SADD\[9\], SADD\[8\]
and SADD\[0\]
are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\[9:0\]
must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed.*/
pub type SADD_R = crate::FieldReader<u16>;
/**Field `SADD` writer - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\[7:1\]
must be written with the 7-bit slave address to be sent. Bits SADD\[9\], SADD\[8\]
and SADD\[0\]
are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\[9:0\]
must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed.*/
pub type SADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RD_WRN` reader - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.
pub type RD_WRN_R = crate::BitReader;
///Field `RD_WRN` writer - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.
pub type RD_WRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD10` reader - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.
pub type ADD10_R = crate::BitReader;
///Field `ADD10` writer - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.
pub type HEAD10R_R = crate::BitReader;
///Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.
pub type HEAD10R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.
pub type START_R = crate::BitReader;
///Field `START` writer - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.
pub type STOP_R = crate::BitReader;
///Field `STOP` writer - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` reader - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used only in slave mode: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.
pub type NACK_R = crate::BitReader;
///Field `NACK` writer - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used only in slave mode: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBYTES` reader - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don t care in slave mode with SBC = 0. Note: Changing these bits when the START bit is set is not allowed.
pub type NBYTES_R = crate::FieldReader;
///Field `NBYTES` writer - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don t care in slave mode with SBC = 0. Note: Changing these bits when the START bit is set is not allowed.
pub type NBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RELOAD` reader - NBYTES reload mode This bit is set and cleared by software.
pub type RELOAD_R = crate::BitReader;
///Field `RELOAD` writer - NBYTES reload mode This bit is set and cleared by software.
pub type RELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTOEND` reader - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.
pub type AUTOEND_R = crate::BitReader;
///Field `AUTOEND` writer - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.
pub type AUTOEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PECBYTE` reader - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit has no effect when RELOAD is set, and in slave mode when SBC = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type PECBYTE_R = crate::BitReader;
///Field `PECBYTE` writer - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit has no effect when RELOAD is set, and in slave mode when SBC = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type PECBYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bits 0:9 - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\[7:1\]
    must be written with the 7-bit slave address to be sent. Bits SADD\[9\], SADD\[8\]
    and SADD\[0\]
    are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\[9:0\]
    must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed.*/
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used only in slave mode: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don t care in slave mode with SBC = 0. Note: Changing these bits when the START bit is set is not allowed.
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - NBYTES reload mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit has no effect when RELOAD is set, and in slave mode when SBC = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
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
    /**Bits 0:9 - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\[7:1\]
    must be written with the 7-bit slave address to be sent. Bits SADD\[9\], SADD\[8\]
    and SADD\[0\]
    are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\[9:0\]
    must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed.*/
    #[inline(always)]
    pub fn sadd(&mut self) -> SADD_W<CR2rs> {
        SADD_W::new(self, 0)
    }
    ///Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<CR2rs> {
        RD_WRN_W::new(self, 10)
    }
    ///Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W<CR2rs> {
        ADD10_W::new(self, 11)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn head10r(&mut self) -> HEAD10R_W<CR2rs> {
        HEAD10R_W::new(self, 12)
    }
    ///Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CR2rs> {
        START_W::new(self, 13)
    }
    ///Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 14)
    }
    ///Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used only in slave mode: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<CR2rs> {
        NACK_W::new(self, 15)
    }
    ///Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don t care in slave mode with SBC = 0. Note: Changing these bits when the START bit is set is not allowed.
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W<CR2rs> {
        NBYTES_W::new(self, 16)
    }
    ///Bit 24 - NBYTES reload mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<CR2rs> {
        RELOAD_W::new(self, 24)
    }
    ///Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.
    #[inline(always)]
    pub fn autoend(&mut self) -> AUTOEND_W<CR2rs> {
        AUTOEND_W::new(self, 25)
    }
    ///Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit has no effect when RELOAD is set, and in slave mode when SBC = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PECBYTE_W<CR2rs> {
        PECBYTE_W::new(self, 26)
    }
}
/**I2C control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#I2C1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
