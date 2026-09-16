///Register `DOUT` reader
pub type R = crate::R<DOUTrs>;
///Field `DATAOUT` reader - DATAOUT
pub type DATAOUT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - DATAOUT
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT")
            .field("dataout", &self.dataout())
            .finish()
    }
}
/**The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.

You can [`read`](crate::Reg::read) this register and get [`dout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:DOUT)*/
pub struct DOUTrs;
impl crate::RegisterSpec for DOUTrs {
    type Ux = u32;
}
///`read()` method returns [`dout::R`](R) reader structure
impl crate::Readable for DOUTrs {}
///`reset()` method sets DOUT to value 0
impl crate::Resettable for DOUTrs {}
