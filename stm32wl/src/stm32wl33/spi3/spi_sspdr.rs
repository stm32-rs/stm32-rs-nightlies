///Register `SPI_SSPDR` reader
pub type R = crate::R<SPI_SSPDRrs>;
///Register `SPI_SSPDR` writer
pub type W = crate::W<SPI_SSPDRrs>;
///Field `DR` reader - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 18.5.8: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used.
pub type DR_R = crate::FieldReader<u16>;
///Field `DR` writer - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 18.5.8: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used.
pub type DR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 18.5.8: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used.
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SSPDR").field("dr", &self.dr()).finish()
    }
}
impl W {
    ///Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 18.5.8: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used.
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, SPI_SSPDRrs> {
        DR_W::new(self, 0)
    }
}
/**SPI_SSPDR register

You can [`read`](crate::Reg::read) this register and get [`spi_sspdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPDR)*/
pub struct SPI_SSPDRrs;
impl crate::RegisterSpec for SPI_SSPDRrs {
    type Ux = u32;
}
///`read()` method returns [`spi_sspdr::R`](R) reader structure
impl crate::Readable for SPI_SSPDRrs {}
///`write(|w| ..)` method takes [`spi_sspdr::W`](W) writer structure
impl crate::Writable for SPI_SSPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_SSPDR to value 0
impl crate::Resettable for SPI_SSPDRrs {}
