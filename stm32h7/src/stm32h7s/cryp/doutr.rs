///Register `DOUTR` reader
pub type R = crate::R<DOUTRrs>;
///Field `DOUT` reader - Data output A four-fold sequential read to this bitfield during the output phase results in retrieving a complete 16-byte block from the CRYP output FIFO. From the first to the fourth read, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Output FIFO can store up to two 16-byte blocks of plaintext (when decrypting) or ciphertext (when encrypting). When the output FIFO is empty a read returns an undefined value. Writes are ignored.
pub type DOUT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Data output A four-fold sequential read to this bitfield during the output phase results in retrieving a complete 16-byte block from the CRYP output FIFO. From the first to the fourth read, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Output FIFO can store up to two 16-byte blocks of plaintext (when decrypting) or ciphertext (when encrypting). When the output FIFO is empty a read returns an undefined value. Writes are ignored.
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR").field("dout", &self.dout()).finish()
    }
}
/**CRYP data output register

You can [`read`](crate::Reg::read) this register and get [`doutr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:DOUTR)*/
pub struct DOUTRrs;
impl crate::RegisterSpec for DOUTRrs {
    type Ux = u32;
}
///`read()` method returns [`doutr::R`](R) reader structure
impl crate::Readable for DOUTRrs {}
///`reset()` method sets DOUTR to value 0
impl crate::Resettable for DOUTRrs {}
