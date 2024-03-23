#[doc = "Register `RXCRC` reader"]
pub type R = crate::R<RXCRCrs>;
#[doc = "Field `RXCRC` reader - CRC register for receiver"]
pub type RXCRC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
#[doc = "Receiver CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCRCrs;
impl crate::RegisterSpec for RXCRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrc::R`](R) reader structure"]
impl crate::Readable for RXCRCrs {}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RXCRCrs {
    const RESET_VALUE: u32 = 0;
}
