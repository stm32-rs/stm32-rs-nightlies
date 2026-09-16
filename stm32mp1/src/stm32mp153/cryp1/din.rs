///Register `DIN` reader
pub type R = crate::R<DINrs>;
///Register `DIN` writer
pub type W = crate::W<DINrs>;
///Field `DATAIN` reader - DATAIN
pub type DATAIN_R = crate::FieldReader<u32>;
///Field `DATAIN` writer - DATAIN
pub type DATAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DATAIN
    #[inline(always)]
    pub fn datain(&self) -> DATAIN_R {
        DATAIN_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN")
            .field("datain", &self.datain())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DATAIN
    #[inline(always)]
    pub fn datain(&mut self) -> DATAIN_W<'_, DINrs> {
        DATAIN_W::new(self, 0)
    }
}
/**The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.

You can [`read`](crate::Reg::read) this register and get [`din::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:DIN)*/
pub struct DINrs;
impl crate::RegisterSpec for DINrs {
    type Ux = u32;
}
///`read()` method returns [`din::R`](R) reader structure
impl crate::Readable for DINrs {}
///`write(|w| ..)` method takes [`din::W`](W) writer structure
impl crate::Writable for DINrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIN to value 0
impl crate::Resettable for DINrs {}
