///Register `SPI_IPIDR` reader
pub type R = crate::R<SPI_IPIDRrs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_IPIDR").field("id", &self.id()).finish()
    }
}
/**SPI/I2S identification register

You can [`read`](crate::Reg::read) this register and get [`spi_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI_IPIDR)*/
pub struct SPI_IPIDRrs;
impl crate::RegisterSpec for SPI_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`spi_ipidr::R`](R) reader structure
impl crate::Readable for SPI_IPIDRrs {}
///`reset()` method sets SPI_IPIDR to value 0x0013_0022
impl crate::Resettable for SPI_IPIDRrs {
    const RESET_VALUE: u32 = 0x0013_0022;
}
