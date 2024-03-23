#[doc = "Register `SPI_IPIDR` reader"]
pub type R = crate::R<SPI_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "SPI/I2S identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_IPIDRrs;
impl crate::RegisterSpec for SPI_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ipidr::R`](R) reader structure"]
impl crate::Readable for SPI_IPIDRrs {}
#[doc = "`reset()` method sets SPI_IPIDR to value 0x0013_0022"]
impl crate::Resettable for SPI_IPIDRrs {
    const RESET_VALUE: u32 = 0x0013_0022;
}
