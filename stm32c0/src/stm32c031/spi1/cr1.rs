///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `CPHA` reader - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
pub type CPHA_R = crate::BitReader;
///Field `CPHA` writer - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPOL` reader - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
pub type CPOL_R = crate::BitReader;
///Field `CPOL` writer - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTR` reader - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode.
pub type MSTR_R = crate::BitReader;
///Field `MSTR` writer - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode.
pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR` reader - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode.
pub type BR_R = crate::FieldReader;
///Field `BR` writer - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode.
pub type BR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPE` reader - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page 1349. This bit is not used in I2S mode.
pub type SPE_R = crate::BitReader;
///Field `SPE` writer - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page 1349. This bit is not used in I2S mode.
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSBFIRST` reader - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode.
pub type LSBFIRST_R = crate::BitReader;
///Field `LSBFIRST` writer - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode.
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSI` reader - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode.
pub type SSI_R = crate::BitReader;
///Field `SSI` writer - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode.
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSM` reader - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode.
pub type SSM_R = crate::BitReader;
///Field `SSM` writer - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode.
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXONLY` reader - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode.
pub type RXONLY_R = crate::BitReader;
///Field `RXONLY` writer - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode.
pub type RXONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCL` reader - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = 0) for correct operation. This bit is not used in I2S mode.
pub type CRCL_R = crate::BitReader;
///Field `CRCL` writer - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = 0) for correct operation. This bit is not used in I2S mode.
pub type CRCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCNEXT` reader - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPI_DR register. This bit is not used in I2S mode.
pub type CRCNEXT_R = crate::BitReader;
///Field `CRCNEXT` writer - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPI_DR register. This bit is not used in I2S mode.
pub type CRCNEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = 0) for correct operation. This bit is not used in I2S mode.
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = 0) for correct operation. This bit is not used in I2S mode.
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIDIOE` reader - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode.
pub type BIDIOE_R = crate::BitReader;
///Field `BIDIOE` writer - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode.
pub type BIDIOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIDIMODE` reader - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode.
pub type BIDIMODE_R = crate::BitReader;
///Field `BIDIMODE` writer - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode.
pub type BIDIMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode.
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page 1349. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode.
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = 0) for correct operation. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPI_DR register. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = 0) for correct operation. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode.
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("mstr", &self.mstr())
            .field("br", &self.br())
            .field("spe", &self.spe())
            .field("lsbfirst", &self.lsbfirst())
            .field("ssi", &self.ssi())
            .field("ssm", &self.ssm())
            .field("rxonly", &self.rxonly())
            .field("crcl", &self.crcl())
            .field("crcnext", &self.crcnext())
            .field("crcen", &self.crcen())
            .field("bidioe", &self.bidioe())
            .field("bidimode", &self.bidimode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<CR1rs> {
        CPHA_W::new(self, 0)
    }
    ///Bit 1 - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<CR1rs> {
        CPOL_W::new(self, 1)
    }
    ///Bit 2 - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<CR1rs> {
        MSTR_W::new(self, 2)
    }
    ///Bits 3:5 - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode.
    #[inline(always)]
    pub fn br(&mut self) -> BR_W<CR1rs> {
        BR_W::new(self, 3)
    }
    ///Bit 6 - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page 1349. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<CR1rs> {
        SPE_W::new(self, 6)
    }
    ///Bit 7 - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<CR1rs> {
        LSBFIRST_W::new(self, 7)
    }
    ///Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<CR1rs> {
        SSI_W::new(self, 8)
    }
    ///Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W<CR1rs> {
        SSM_W::new(self, 9)
    }
    ///Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode.
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W<CR1rs> {
        RXONLY_W::new(self, 10)
    }
    ///Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = 0) for correct operation. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn crcl(&mut self) -> CRCL_W<CR1rs> {
        CRCL_W::new(self, 11)
    }
    ///Bit 12 - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPI_DR register. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W<CR1rs> {
        CRCNEXT_W::new(self, 12)
    }
    ///Bit 13 - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = 0) for correct operation. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<CR1rs> {
        CRCEN_W::new(self, 13)
    }
    ///Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn bidioe(&mut self) -> BIDIOE_W<CR1rs> {
        BIDIOE_W::new(self, 14)
    }
    ///Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode.
    #[inline(always)]
    pub fn bidimode(&mut self) -> BIDIMODE_W<CR1rs> {
        BIDIMODE_W::new(self, 15)
    }
}
/**SPI control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SPI1:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u16;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u16 = 0;
}
