///Register `AES_DINR` writer
pub type W = crate::W<AES_DINRrs>;
/**Field `DIN` writer - Data input A four-fold sequential write to this bitfield during the Input phase results in writing a complete 16-bytes block of input data to the AES peripheral. From the first to the fourth write, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\[1:0\]
bitfield, then written into the AES core 16-bytes input buffer. Reads return zero.*/
pub type DIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<AES_DINRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /**Bits 0:31 - Data input A four-fold sequential write to this bitfield during the Input phase results in writing a complete 16-bytes block of input data to the AES peripheral. From the first to the fourth write, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\[1:0\]
    bitfield, then written into the AES core 16-bytes input buffer. Reads return zero.*/
    #[inline(always)]
    pub fn din(&mut self) -> DIN_W<AES_DINRrs> {
        DIN_W::new(self, 0)
    }
}
/**AES data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_dinr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_DINR)*/
pub struct AES_DINRrs;
impl crate::RegisterSpec for AES_DINRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`aes_dinr::W`](W) writer structure
impl crate::Writable for AES_DINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_DINR to value 0
impl crate::Resettable for AES_DINRrs {
    const RESET_VALUE: u32 = 0;
}
