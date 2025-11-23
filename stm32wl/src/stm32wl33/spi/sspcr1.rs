///Register `SSPCR1` reader
pub type R = crate::R<SSPCR1rs>;
///Register `SSPCR1` writer
pub type W = crate::W<SSPCR1rs>;
///Field `CPHA` reader - Clock phase - 0: The first clock transition is the first data capture edge - 1: The second clock transition is the first data capture edge
pub type CPHA_R = crate::BitReader;
///Field `CPHA` writer - Clock phase - 0: The first clock transition is the first data capture edge - 1: The second clock transition is the first data capture edge
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPOL` reader - Clock polarity - 0: CK to 0 when idle - 1: CK to 1 when idle
pub type CPOL_R = crate::BitReader;
///Field `CPOL` writer - Clock polarity - 0: CK to 0 when idle - 1: CK to 1 when idle
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTR` reader - Master selection - 0: Slave configuration - 1: Master configuration
pub type MSTR_R = crate::BitReader;
///Field `MSTR` writer - Master selection - 0: Slave configuration - 1: Master configuration
pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR` reader - Baud rate control - 000: fPCLK/2 - 001: fPCLK/4 - 010: fPCLK/8 - 011: fPCLK/16 - 100: fPCLK/32 - 101: fPCLK/64 - 110: fPCLK/128 - 111: fPCLK/256
pub type BR_R = crate::FieldReader;
///Field `BR` writer - Baud rate control - 000: fPCLK/2 - 001: fPCLK/4 - 010: fPCLK/8 - 011: fPCLK/16 - 100: fPCLK/32 - 101: fPCLK/64 - 110: fPCLK/128 - 111: fPCLK/256
pub type BR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPE` reader - SPI enable - 0: Peripheral disabled - 1: Peripheral enabled
pub type SPE_R = crate::BitReader;
///Field `SPE` writer - SPI enable - 0: Peripheral disabled - 1: Peripheral enabled
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSBFIRST` reader - Frame format - 0: data is transmitted / received with the MSB first - 1: data is transmitted / received with the LSB first
pub type LSBFIRST_R = crate::BitReader;
///Field `LSBFIRST` writer - Frame format - 0: data is transmitted / received with the MSB first - 1: data is transmitted / received with the LSB first
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSI` reader - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored.
pub type SSI_R = crate::BitReader;
///Field `SSI` writer - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored.
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSM` reader - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. - 0: Software slave management disabled - 1: Software slave management enabled
pub type SSM_R = crate::BitReader;
///Field `SSM` writer - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. - 0: Software slave management disabled - 1: Software slave management enabled
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXONLY` reader - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. - 0: Full duplex (Transmit and receive) - 1: Output disabled (Receive-only mode)
pub type RXONLY_R = crate::BitReader;
///Field `RXONLY` writer - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. - 0: Full duplex (Transmit and receive) - 1: Output disabled (Receive-only mode)
pub type RXONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCL` reader - CRC length This bit is set and cleared by software to select the CRC length. - 0: 8-bit CRC length - 1: 16-bit CRC length
pub type CRCL_R = crate::BitReader;
///Field `CRCL` writer - CRC length This bit is set and cleared by software to select the CRC length. - 0: 8-bit CRC length - 1: 16-bit CRC length
pub type CRCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCNEXT` reader - Transmit CRC next - 0: Next transmit value is from Tx buffer - 1: Next transmit value is from Tx CRC register
pub type CRCNEXT_R = crate::BitReader;
///Field `CRCNEXT` writer - Transmit CRC next - 0: Next transmit value is from Tx buffer - 1: Next transmit value is from Tx CRC register
pub type CRCNEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - Hardware CRC calculation enable - 0: CRC calculation disabled - 1: CRC calculation Enabled
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - Hardware CRC calculation enable - 0: CRC calculation disabled - 1: CRC calculation Enabled
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIDIOE` reader - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode - 0: Output disabled (receive-only mode) - 1: Output enabled (transmit-only mode)
pub type BIDIOE_R = crate::BitReader;
///Field `BIDIOE` writer - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode - 0: Output disabled (receive-only mode) - 1: Output enabled (transmit-only mode)
pub type BIDIOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIDIMODE` reader - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. - 0: 2-line unidirectional data mode selected - 1: 1-line bidirectional data mode selected
pub type BIDIMODE_R = crate::BitReader;
///Field `BIDIMODE` writer - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. - 0: 2-line unidirectional data mode selected - 1: 1-line bidirectional data mode selected
pub type BIDIMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clock phase - 0: The first clock transition is the first data capture edge - 1: The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock polarity - 0: CK to 0 when idle - 1: CK to 1 when idle
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master selection - 0: Slave configuration - 1: Master configuration
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - Baud rate control - 000: fPCLK/2 - 001: fPCLK/4 - 010: fPCLK/8 - 011: fPCLK/16 - 100: fPCLK/32 - 101: fPCLK/64 - 110: fPCLK/128 - 111: fPCLK/256
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - SPI enable - 0: Peripheral disabled - 1: Peripheral enabled
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Frame format - 0: data is transmitted / received with the MSB first - 1: data is transmitted / received with the LSB first
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored.
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. - 0: Software slave management disabled - 1: Software slave management enabled
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. - 0: Full duplex (Transmit and receive) - 1: Output disabled (Receive-only mode)
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. - 0: 8-bit CRC length - 1: 16-bit CRC length
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Transmit CRC next - 0: Next transmit value is from Tx buffer - 1: Next transmit value is from Tx CRC register
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Hardware CRC calculation enable - 0: CRC calculation disabled - 1: CRC calculation Enabled
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode - 0: Output disabled (receive-only mode) - 1: Output enabled (transmit-only mode)
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. - 0: 2-line unidirectional data mode selected - 1: 1-line bidirectional data mode selected
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPCR1")
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
    ///Bit 0 - Clock phase - 0: The first clock transition is the first data capture edge - 1: The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<'_, SSPCR1rs> {
        CPHA_W::new(self, 0)
    }
    ///Bit 1 - Clock polarity - 0: CK to 0 when idle - 1: CK to 1 when idle
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<'_, SSPCR1rs> {
        CPOL_W::new(self, 1)
    }
    ///Bit 2 - Master selection - 0: Slave configuration - 1: Master configuration
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<'_, SSPCR1rs> {
        MSTR_W::new(self, 2)
    }
    ///Bits 3:5 - Baud rate control - 000: fPCLK/2 - 001: fPCLK/4 - 010: fPCLK/8 - 011: fPCLK/16 - 100: fPCLK/32 - 101: fPCLK/64 - 110: fPCLK/128 - 111: fPCLK/256
    #[inline(always)]
    pub fn br(&mut self) -> BR_W<'_, SSPCR1rs> {
        BR_W::new(self, 3)
    }
    ///Bit 6 - SPI enable - 0: Peripheral disabled - 1: Peripheral enabled
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<'_, SSPCR1rs> {
        SPE_W::new(self, 6)
    }
    ///Bit 7 - Frame format - 0: data is transmitted / received with the MSB first - 1: data is transmitted / received with the LSB first
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<'_, SSPCR1rs> {
        LSBFIRST_W::new(self, 7)
    }
    ///Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored.
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<'_, SSPCR1rs> {
        SSI_W::new(self, 8)
    }
    ///Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. - 0: Software slave management disabled - 1: Software slave management enabled
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W<'_, SSPCR1rs> {
        SSM_W::new(self, 9)
    }
    ///Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. - 0: Full duplex (Transmit and receive) - 1: Output disabled (Receive-only mode)
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W<'_, SSPCR1rs> {
        RXONLY_W::new(self, 10)
    }
    ///Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. - 0: 8-bit CRC length - 1: 16-bit CRC length
    #[inline(always)]
    pub fn crcl(&mut self) -> CRCL_W<'_, SSPCR1rs> {
        CRCL_W::new(self, 11)
    }
    ///Bit 12 - Transmit CRC next - 0: Next transmit value is from Tx buffer - 1: Next transmit value is from Tx CRC register
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W<'_, SSPCR1rs> {
        CRCNEXT_W::new(self, 12)
    }
    ///Bit 13 - Hardware CRC calculation enable - 0: CRC calculation disabled - 1: CRC calculation Enabled
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, SSPCR1rs> {
        CRCEN_W::new(self, 13)
    }
    ///Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode - 0: Output disabled (receive-only mode) - 1: Output enabled (transmit-only mode)
    #[inline(always)]
    pub fn bidioe(&mut self) -> BIDIOE_W<'_, SSPCR1rs> {
        BIDIOE_W::new(self, 14)
    }
    ///Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. - 0: 2-line unidirectional data mode selected - 1: 1-line bidirectional data mode selected
    #[inline(always)]
    pub fn bidimode(&mut self) -> BIDIMODE_W<'_, SSPCR1rs> {
        BIDIMODE_W::new(self, 15)
    }
}
/**SPI_SSPCR1 register

You can [`read`](crate::Reg::read) this register and get [`sspcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI:SSPCR1)*/
pub struct SSPCR1rs;
impl crate::RegisterSpec for SSPCR1rs {
    type Ux = u32;
}
///`read()` method returns [`sspcr1::R`](R) reader structure
impl crate::Readable for SSPCR1rs {}
///`write(|w| ..)` method takes [`sspcr1::W`](W) writer structure
impl crate::Writable for SSPCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSPCR1 to value 0
impl crate::Resettable for SSPCR1rs {}
