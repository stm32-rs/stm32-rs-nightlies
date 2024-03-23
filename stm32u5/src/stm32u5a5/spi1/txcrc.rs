#[doc = "Register `TXCRC` reader"]
pub type R = crate::R<TXCRCrs>;
#[doc = "Field `TXCRC` reader - CRC register for transmitter"]
pub type TXCRC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CRC register for transmitter"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
#[doc = "Transmitter CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCRCrs;
impl crate::RegisterSpec for TXCRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcrc::R`](R) reader structure"]
impl crate::Readable for TXCRCrs {}
#[doc = "`reset()` method sets TXCRC to value 0"]
impl crate::Resettable for TXCRCrs {
    const RESET_VALUE: u32 = 0;
}
