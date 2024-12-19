///Register `SPI_SR` reader
pub type R = crate::R<SPI_SRrs>;
///Register `SPI_SR` writer
pub type W = crate::W<SPI_SRrs>;
///Field `RXNE` reader - Receive buffer not empty
pub type RXNE_R = crate::BitReader;
///Field `TXE` reader - Transmit buffer empty
pub type TXE_R = crate::BitReader;
///Field `CRCERR` reader - CRC error flag Note: This flag is set by hardware and cleared by software writing 0.
pub type CRCERR_R = crate::BitReader;
///Field `CRCERR` writer - CRC error flag Note: This flag is set by hardware and cleared by software writing 0.
pub type CRCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODF` reader - Mode fault This flag is set by hardware and reset by a software sequence. Refer to Section1: Mode fault (MODF) on page1964 for the software sequence.
pub type MODF_R = crate::BitReader;
///Field `OVR` reader - Overrun flag This flag is set by hardware and reset by a software sequence.
pub type OVR_R = crate::BitReader;
///Field `BSY` reader - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to Section133.4.10: SPI status flags and Procedure for disabling the SPI on page1954.
pub type BSY_R = crate::BitReader;
///Field `FRE` reader - Frame format error This flag is used for SPI in TI slave mode. Refer to Section133.4.11: SPI error flags. This flag is set by hardware and reset when SPI_SR is read by software.
pub type FRE_R = crate::BitReader;
///Field `FRLVL` reader - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in SPI receive-only mode while CRC calculation is enabled.
pub type FRLVL_R = crate::FieldReader;
///Field `FTLVL` reader - FIFO transmission level These bits are set and cleared by hardware.
pub type FTLVL_R = crate::FieldReader;
impl R {
    ///Bit 0 - Receive buffer not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit buffer empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0.
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mode fault This flag is set by hardware and reset by a software sequence. Refer to Section1: Mode fault (MODF) on page1964 for the software sequence.
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overrun flag This flag is set by hardware and reset by a software sequence.
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to Section133.4.10: SPI status flags and Procedure for disabling the SPI on page1954.
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Frame format error This flag is used for SPI in TI slave mode. Refer to Section133.4.11: SPI error flags. This flag is set by hardware and reset when SPI_SR is read by software.
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in SPI receive-only mode while CRC calculation is enabled.
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - FIFO transmission level These bits are set and cleared by hardware.
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SR")
            .field("rxne", &self.rxne())
            .field("txe", &self.txe())
            .field("crcerr", &self.crcerr())
            .field("modf", &self.modf())
            .field("ovr", &self.ovr())
            .field("bsy", &self.bsy())
            .field("fre", &self.fre())
            .field("frlvl", &self.frlvl())
            .field("ftlvl", &self.ftlvl())
            .finish()
    }
}
impl W {
    ///Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0.
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W<SPI_SRrs> {
        CRCERR_W::new(self, 4)
    }
}
/**SPI status register

You can [`read`](crate::Reg::read) this register and get [`spi_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SPI1:SPI_SR)*/
pub struct SPI_SRrs;
impl crate::RegisterSpec for SPI_SRrs {
    type Ux = u16;
}
///`read()` method returns [`spi_sr::R`](R) reader structure
impl crate::Readable for SPI_SRrs {}
///`write(|w| ..)` method takes [`spi_sr::W`](W) writer structure
impl crate::Writable for SPI_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets SPI_SR to value 0x02
impl crate::Resettable for SPI_SRrs {
    const RESET_VALUE: u16 = 0x02;
}
