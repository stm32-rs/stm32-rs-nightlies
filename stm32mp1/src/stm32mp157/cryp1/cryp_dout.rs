#[doc = "Register `CRYP_DOUT` reader"]
pub type R = crate::R<CRYP_DOUTrs>;
#[doc = "Field `DATAOUT` reader - DATAOUT"]
pub type DATAOUT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DATAOUT"]
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new(self.bits)
    }
}
#[doc = "The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_dout::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_DOUTrs;
impl crate::RegisterSpec for CRYP_DOUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_dout::R`](R) reader structure"]
impl crate::Readable for CRYP_DOUTrs {}
#[doc = "`reset()` method sets CRYP_DOUT to value 0"]
impl crate::Resettable for CRYP_DOUTrs {
    const RESET_VALUE: u32 = 0;
}
