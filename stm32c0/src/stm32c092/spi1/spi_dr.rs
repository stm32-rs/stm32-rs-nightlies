///Register `SPI_DR` reader
pub type R = crate::R<SPI_DRrs>;
///Register `SPI_DR` writer
pub type W = crate::W<SPI_DRrs>;
///Field `DR` reader - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 27.5.9: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used.
pub type DR_R = crate::FieldReader<u16>;
///Field `DR` writer - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 27.5.9: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used.
pub type DR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 27.5.9: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used.
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DR").field("dr", &self.dr()).finish()
    }
}
impl W {
    ///Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 27.5.9: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used.
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, SPI_DRrs> {
        DR_W::new(self, 0)
    }
}
/**SPI data register

You can [`read`](crate::Reg::read) this register and get [`spi_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#SPI1:SPI_DR)*/
pub struct SPI_DRrs;
impl crate::RegisterSpec for SPI_DRrs {
    type Ux = u16;
}
///`read()` method returns [`spi_dr::R`](R) reader structure
impl crate::Readable for SPI_DRrs {}
///`write(|w| ..)` method takes [`spi_dr::W`](W) writer structure
impl crate::Writable for SPI_DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_DR to value 0
impl crate::Resettable for SPI_DRrs {}
