#[doc = "Register `SPI_SR` reader"]
pub type R = crate::R<SPI_SRrs>;
#[doc = "Field `RXP` reader - Rx-Packet available RXP flag is changed by hardware. It monitors number of overall data currently available at RxFIFO if SPI is enabled. It has to be checked once a data packet is completely read out from RxFIFO."]
pub type RXP_R = crate::BitReader;
#[doc = "Field `TXP` reader - Tx-Packet space available TXP flag is changed by hardware. It monitors overall space currently available at TxFIFO no matter if SPI is enabled or not. It has to be checked once a complete data packet is stored at TxFIFO."]
pub type TXP_R = crate::BitReader;
#[doc = "Field `DXP` reader - duplex packet DXP flag is set whenever both TXP and RXP flags are set regardless SPI mode."]
pub type DXP_R = crate::BitReader;
#[doc = "Field `EOT` reader - end of transfer EOT is set by hardware as soon as a full transfer is complete, that is when TSIZE number of data have been transmitted and/or received on the SPI. EOT is cleared by software write 1 to EOTC bit at SPI_IFCR. EOT flag triggers an interrupt if EOTIE bit is set. If DXP flag is used until TXTF flag is set and DXPIE is cleared, EOT can be used to download the last packets contained into RxFIFO in one-shot. In master, EOT event terminates the data transaction and handles SS output optionally. When CRC is applied, the EOT event is extended over the CRC frame transaction. To restart the internal state machine properly, SPI is strongly suggested to be disabled and re-enabled before next transaction starts despite its setting is not changed."]
pub type EOT_R = crate::BitReader;
#[doc = "Field `TXTF` reader - transmission transfer filled TXTF is set by hardware as soon as all of the data packets in a transfer have been submitted for transmission by application software or DMA, that is when TSIZE number of data have been pushed into the TxFIFO. This bit is cleared by software write 1 to TXTFC bit at SPI_IFCR TXTF flag triggers an interrupt if TXTFIE bit is set. TXTF setting clears the TXPIE and DXPIE masks so to off-load application software from calculating when to disable TXP and DXP interrupts."]
pub type TXTF_R = crate::BitReader;
#[doc = "Field `UDR` reader - underrun at slave transmission mode This bit is cleared by writing 1 to UDRC bit at SPI_IFCR Note: UDR flag applies to Slave mode only"]
pub type UDR_R = crate::BitReader;
#[doc = "Field `OVR` reader - overrun This bit is cleared by writing 1 to OVRC bit at SPI_IFCR"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `CRCE` reader - CRC error This bit is cleared by writing 1 to CRCEC bit at SPI_IFCR"]
pub type CRCE_R = crate::BitReader;
#[doc = "Field `TIFRE` reader - TI frame format error This bit is cleared by writing 1 to TIFREC bit at SPI_IFCR"]
pub type TIFRE_R = crate::BitReader;
#[doc = "Field `MODF` reader - mode fault This bit is cleared by writing 1 to MODFC bit at SPI_IFCR"]
pub type MODF_R = crate::BitReader;
#[doc = "Field `SUSP` reader - suspension status In Master mode, SUSP is set by hardware either as soon as the current frame is completed after CSUSP request is done or at master automatic suspend receive mode (MASRX bit is set at SPI_CR1 register) on RxFIFO full condition. SUSP generates an interrupt when EOTIE is set. This bit has to be cleared prior SPI is disabled by writing 1 to SUSPC bit at SPI_IFCR."]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `TXC` reader - TxFIFO transmission complete The flag behavior depends on TSIZE setting. When TSIZE=0 the TXC is changed by hardware exclusively and it raises each time the TxFIFO becomes empty and there is no activity on the bus. If TSIZE &lt;>0 there is no specific reason to monitor TXC as it just copies the EOT flag value including its software clearing. The TXC generates an interrupt when EOTIE is set."]
pub type TXC_R = crate::BitReader;
#[doc = "Field `RXPLVL` reader - RxFIFO packing level When RXWNE=0 and data size is set up to 16-bit, the value gives number of remaining data frames persisting at RxFIFO. Note: (*): Optional value when data size is set up to 8-bit only. When data size is greater than 16-bit, these bits are always read as 00. In that consequence, the single data frame received at the FIFO cannot be detected neither by RWNE nor by RXPLVL bits if data size is set from 17 to 24 bits. The user then must apply other methods like TSIZE>0 or FTHLV=0."]
pub type RXPLVL_R = crate::FieldReader;
#[doc = "Field `RXWNE` reader - RxFIFO word not empty Note: This bit value does not depend on DSIZE setting and keeps together with RXPLVL\\[1:0\\]
information about RxFIFO occupancy by residual data."]
pub type RXWNE_R = crate::BitReader;
#[doc = "Field `CTSIZE` reader - number of data frames remaining in current TSIZE session The value is not quite reliable when traffic is ongoing on bus or during autonomous operation at low-power mode. Note: CTSIZE\\[15:0\\]
bits are not available at instances with limited set of features"]
pub type CTSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Rx-Packet available RXP flag is changed by hardware. It monitors number of overall data currently available at RxFIFO if SPI is enabled. It has to be checked once a data packet is completely read out from RxFIFO."]
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx-Packet space available TXP flag is changed by hardware. It monitors overall space currently available at TxFIFO no matter if SPI is enabled or not. It has to be checked once a complete data packet is stored at TxFIFO."]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - duplex packet DXP flag is set whenever both TXP and RXP flags are set regardless SPI mode."]
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - end of transfer EOT is set by hardware as soon as a full transfer is complete, that is when TSIZE number of data have been transmitted and/or received on the SPI. EOT is cleared by software write 1 to EOTC bit at SPI_IFCR. EOT flag triggers an interrupt if EOTIE bit is set. If DXP flag is used until TXTF flag is set and DXPIE is cleared, EOT can be used to download the last packets contained into RxFIFO in one-shot. In master, EOT event terminates the data transaction and handles SS output optionally. When CRC is applied, the EOT event is extended over the CRC frame transaction. To restart the internal state machine properly, SPI is strongly suggested to be disabled and re-enabled before next transaction starts despite its setting is not changed."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - transmission transfer filled TXTF is set by hardware as soon as all of the data packets in a transfer have been submitted for transmission by application software or DMA, that is when TSIZE number of data have been pushed into the TxFIFO. This bit is cleared by software write 1 to TXTFC bit at SPI_IFCR TXTF flag triggers an interrupt if TXTFIE bit is set. TXTF setting clears the TXPIE and DXPIE masks so to off-load application software from calculating when to disable TXP and DXP interrupts."]
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - underrun at slave transmission mode This bit is cleared by writing 1 to UDRC bit at SPI_IFCR Note: UDR flag applies to Slave mode only"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - overrun This bit is cleared by writing 1 to OVRC bit at SPI_IFCR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC error This bit is cleared by writing 1 to CRCEC bit at SPI_IFCR"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TI frame format error This bit is cleared by writing 1 to TIFREC bit at SPI_IFCR"]
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - mode fault This bit is cleared by writing 1 to MODFC bit at SPI_IFCR"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - suspension status In Master mode, SUSP is set by hardware either as soon as the current frame is completed after CSUSP request is done or at master automatic suspend receive mode (MASRX bit is set at SPI_CR1 register) on RxFIFO full condition. SUSP generates an interrupt when EOTIE is set. This bit has to be cleared prior SPI is disabled by writing 1 to SUSPC bit at SPI_IFCR."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TxFIFO transmission complete The flag behavior depends on TSIZE setting. When TSIZE=0 the TXC is changed by hardware exclusively and it raises each time the TxFIFO becomes empty and there is no activity on the bus. If TSIZE &lt;>0 there is no specific reason to monitor TXC as it just copies the EOT flag value including its software clearing. The TXC generates an interrupt when EOTIE is set."]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - RxFIFO packing level When RXWNE=0 and data size is set up to 16-bit, the value gives number of remaining data frames persisting at RxFIFO. Note: (*): Optional value when data size is set up to 8-bit only. When data size is greater than 16-bit, these bits are always read as 00. In that consequence, the single data frame received at the FIFO cannot be detected neither by RWNE nor by RXPLVL bits if data size is set from 17 to 24 bits. The user then must apply other methods like TSIZE>0 or FTHLV=0."]
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RxFIFO word not empty Note: This bit value does not depend on DSIZE setting and keeps together with RXPLVL\\[1:0\\]
information about RxFIFO occupancy by residual data."]
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - number of data frames remaining in current TSIZE session The value is not quite reliable when traffic is ongoing on bus or during autonomous operation at low-power mode. Note: CTSIZE\\[15:0\\]
bits are not available at instances with limited set of features"]
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SRrs;
impl crate::RegisterSpec for SPI_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_sr::R`](R) reader structure"]
impl crate::Readable for SPI_SRrs {}
#[doc = "`reset()` method sets SPI_SR to value 0x1002"]
impl crate::Resettable for SPI_SRrs {
    const RESET_VALUE: u32 = 0x1002;
}
