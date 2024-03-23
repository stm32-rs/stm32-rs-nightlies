#[doc = "Register `DINR` writer"]
pub type W = crate::W<DINRrs>;
#[doc = "Field `DIN` writer - Input data word A four-fold sequential write to this bitfield during the input phase results in writing a complete 128-bit block of input data to the SAES peripheral. From the first to the fourth write, the corresponding data weights are \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield, then written into the AES core 128-bit input buffer. The data signification of the input data block depends on the SAES operating mode: - Mode 1 (encryption): plaintext - Mode 2 (key derivation): the bitfield is not used (SAES_KEYRx registers used for input if KEYSEL=0) - Mode 3 (decryption): ciphertext The data swap operation is described in on page 1149."]
pub type DIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input data word A four-fold sequential write to this bitfield during the input phase results in writing a complete 128-bit block of input data to the SAES peripheral. From the first to the fourth write, the corresponding data weights are \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield, then written into the AES core 128-bit input buffer. The data signification of the input data block depends on the SAES operating mode: - Mode 1 (encryption): plaintext - Mode 2 (key derivation): the bitfield is not used (SAES_KEYRx registers used for input if KEYSEL=0) - Mode 3 (decryption): ciphertext The data swap operation is described in on page 1149."]
    #[inline(always)]
    #[must_use]
    pub fn din(&mut self) -> DIN_W<DINRrs> {
        DIN_W::new(self, 0)
    }
}
#[doc = "SAES data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINRrs;
impl crate::RegisterSpec for DINRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dinr::W`](W) writer structure"]
impl crate::Writable for DINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINR to value 0"]
impl crate::Resettable for DINRrs {
    const RESET_VALUE: u32 = 0;
}
