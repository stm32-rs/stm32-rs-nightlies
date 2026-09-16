///Register `DINR` reader
pub type R = crate::R<DINRrs>;
///Register `DINR` writer
pub type W = crate::W<DINRrs>;
///Field `DIN` reader - Data input A four-fold sequential write to this bitfield during the Input phase results in pushing a complete 16-byte block into the CRYP input FIFO. From the first to the fourth write, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Input FIFO can receive up to two 16-byte blocks of plaintext (when encrypting) or ciphertext (when decrypting). If EN bit is set in CRYP_CR register, when at least four 32-bit words have been pushed into the input FIFO, and when at least four 32-bit words are free in the output FIFO, the CRYP automatically starts an encryption or decryption process, setting the BUSY bit. Reading this register pops data off the input FIFO (oldest value is returned). The data present in the input FIFO are returned only if CRYPEN is cleared (undefined value is returned otherwise). Following one or more reads the FIFO must be flushed (setting the FFLUSH bit) prior to processing new data.
pub type DIN_R = crate::FieldReader<u32>;
///Field `DIN` writer - Data input A four-fold sequential write to this bitfield during the Input phase results in pushing a complete 16-byte block into the CRYP input FIFO. From the first to the fourth write, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Input FIFO can receive up to two 16-byte blocks of plaintext (when encrypting) or ciphertext (when decrypting). If EN bit is set in CRYP_CR register, when at least four 32-bit words have been pushed into the input FIFO, and when at least four 32-bit words are free in the output FIFO, the CRYP automatically starts an encryption or decryption process, setting the BUSY bit. Reading this register pops data off the input FIFO (oldest value is returned). The data present in the input FIFO are returned only if CRYPEN is cleared (undefined value is returned otherwise). Following one or more reads the FIFO must be flushed (setting the FFLUSH bit) prior to processing new data.
pub type DIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data input A four-fold sequential write to this bitfield during the Input phase results in pushing a complete 16-byte block into the CRYP input FIFO. From the first to the fourth write, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Input FIFO can receive up to two 16-byte blocks of plaintext (when encrypting) or ciphertext (when decrypting). If EN bit is set in CRYP_CR register, when at least four 32-bit words have been pushed into the input FIFO, and when at least four 32-bit words are free in the output FIFO, the CRYP automatically starts an encryption or decryption process, setting the BUSY bit. Reading this register pops data off the input FIFO (oldest value is returned). The data present in the input FIFO are returned only if CRYPEN is cleared (undefined value is returned otherwise). Following one or more reads the FIFO must be flushed (setting the FFLUSH bit) prior to processing new data.
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR").field("din", &self.din()).finish()
    }
}
impl W {
    ///Bits 0:31 - Data input A four-fold sequential write to this bitfield during the Input phase results in pushing a complete 16-byte block into the CRYP input FIFO. From the first to the fourth write, the corresponding data weights are \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. Input FIFO can receive up to two 16-byte blocks of plaintext (when encrypting) or ciphertext (when decrypting). If EN bit is set in CRYP_CR register, when at least four 32-bit words have been pushed into the input FIFO, and when at least four 32-bit words are free in the output FIFO, the CRYP automatically starts an encryption or decryption process, setting the BUSY bit. Reading this register pops data off the input FIFO (oldest value is returned). The data present in the input FIFO are returned only if CRYPEN is cleared (undefined value is returned otherwise). Following one or more reads the FIFO must be flushed (setting the FFLUSH bit) prior to processing new data.
    #[inline(always)]
    pub fn din(&mut self) -> DIN_W<'_, DINRrs> {
        DIN_W::new(self, 0)
    }
}
/**CRYP data input register

You can [`read`](crate::Reg::read) this register and get [`dinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:DINR)*/
pub struct DINRrs;
impl crate::RegisterSpec for DINRrs {
    type Ux = u32;
}
///`read()` method returns [`dinr::R`](R) reader structure
impl crate::Readable for DINRrs {}
///`write(|w| ..)` method takes [`dinr::W`](W) writer structure
impl crate::Writable for DINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DINR to value 0
impl crate::Resettable for DINRrs {}
