///Register `DINR` writer
pub type W = crate::W<DINRrs>;
///Field `DIN` writer - Input data word A four-fold sequential write to this bitfield during the input phase results in writing a complete 128-bit block of input data to the SAES peripheral. From the first to the fourth write, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\[1:0\] bitfield, then written into the AES core 128-bit input buffer. The data signification of the input data block depends on the SAES operating mode: - Mode 1 (encryption): plaintext - Mode 2 (key derivation): the bitfield is not used (SAES_KEYRx registers used for input if KEYSEL=0) - Mode 3 (decryption): ciphertext The data swap operation is described in on page 1149.
pub type DIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<DINRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Input data word A four-fold sequential write to this bitfield during the input phase results in writing a complete 128-bit block of input data to the SAES peripheral. From the first to the fourth write, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\[1:0\] bitfield, then written into the AES core 128-bit input buffer. The data signification of the input data block depends on the SAES operating mode: - Mode 1 (encryption): plaintext - Mode 2 (key derivation): the bitfield is not used (SAES_KEYRx registers used for input if KEYSEL=0) - Mode 3 (decryption): ciphertext The data swap operation is described in on page 1149.
    #[inline(always)]
    pub fn din(&mut self) -> DIN_W<'_, DINRrs> {
        DIN_W::new(self, 0)
    }
}
/**SAES data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SAES:DINR)*/
pub struct DINRrs;
impl crate::RegisterSpec for DINRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dinr::W`](W) writer structure
impl crate::Writable for DINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DINR to value 0
impl crate::Resettable for DINRrs {}
