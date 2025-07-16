///Register `DOUTR` reader
pub type R = crate::R<DOUTRrs>;
///Field `DOUT` reader - Output data word This read-only bitfield fetches a 32-bit output buffer. A four-fold sequential read of this bitfield, upon the computation completion (CCF flag set), virtually reads a complete 16-byte block of output data from the AES peripheral. Before reaching the output buffer, the data produced by the AES core are handled by the data swap block according to the DATATYPE\[1:0\] bitfield. Data weights from the first to the fourth read operation are: \[127:96\], \[95:64\], \[63:32\], and \[31:0\].
pub type DOUT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Output data word This read-only bitfield fetches a 32-bit output buffer. A four-fold sequential read of this bitfield, upon the computation completion (CCF flag set), virtually reads a complete 16-byte block of output data from the AES peripheral. Before reaching the output buffer, the data produced by the AES core are handled by the data swap block according to the DATATYPE\[1:0\] bitfield. Data weights from the first to the fourth read operation are: \[127:96\], \[95:64\], \[63:32\], and \[31:0\].
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
/**AES data output register

You can [`read`](crate::Reg::read) this register and get [`doutr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#AES:DOUTR)*/
pub struct DOUTRrs;
impl crate::RegisterSpec for DOUTRrs {
    type Ux = u32;
}
///`read()` method returns [`doutr::R`](R) reader structure
impl crate::Readable for DOUTRrs {}
///`reset()` method sets DOUTR to value 0
impl crate::Resettable for DOUTRrs {}
