#[doc = "Register `TXDR` writer"]
pub type W = crate::W<TXDRrs>;
#[doc = "Field `TXDR` writer - transmit data register The register serves as an interface with TxFIFO. A write to it accesses TxFIFO. Note: In SPI mode, data is always right-aligned. Alignment of data at I2S mode depends on DATLEN and DATFMT setting. Unused bits are ignored when writing to the register, and read as zero when the register is read. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is written by single access. halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be written by single access. word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be written by single access. Write access of this register less than the configured data size is forbidden."]
pub type TXDR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - transmit data register The register serves as an interface with TxFIFO. A write to it accesses TxFIFO. Note: In SPI mode, data is always right-aligned. Alignment of data at I2S mode depends on DATLEN and DATFMT setting. Unused bits are ignored when writing to the register, and read as zero when the register is read. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is written by single access. halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be written by single access. word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be written by single access. Write access of this register less than the configured data size is forbidden."]
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TXDR_W<TXDRrs> {
        TXDR_W::new(self, 0)
    }
}
#[doc = "SPI/I2S transmit data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDRrs;
impl crate::RegisterSpec for TXDRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TXDRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TXDRrs {
    const RESET_VALUE: u32 = 0;
}
