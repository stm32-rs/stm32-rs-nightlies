#[doc = "Register `SPI_VERR` reader"]
pub type R = crate::R<SPI_VERRrs>;
#[doc = "Field `MINREV` reader - MINREV"]
pub type MINREV_R = crate::FieldReader;
#[doc = "Field `MAJREV` reader - MAJREV"]
pub type MAJREV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - MINREV"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MAJREV"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SPI/I2S version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_verr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_VERRrs;
impl crate::RegisterSpec for SPI_VERRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_verr::R`](R) reader structure"]
impl crate::Readable for SPI_VERRrs {}
#[doc = "`reset()` method sets SPI_VERR to value 0x11"]
impl crate::Resettable for SPI_VERRrs {
    const RESET_VALUE: u32 = 0x11;
}
