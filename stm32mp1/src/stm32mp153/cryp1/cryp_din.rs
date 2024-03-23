#[doc = "Register `CRYP_DIN` reader"]
pub type R = crate::R<CRYP_DINrs>;
#[doc = "Register `CRYP_DIN` writer"]
pub type W = crate::W<CRYP_DINrs>;
#[doc = "Field `DATAIN` reader - DATAIN"]
pub type DATAIN_R = crate::FieldReader<u32>;
#[doc = "Field `DATAIN` writer - DATAIN"]
pub type DATAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATAIN"]
    #[inline(always)]
    pub fn datain(&self) -> DATAIN_R {
        DATAIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATAIN"]
    #[inline(always)]
    #[must_use]
    pub fn datain(&mut self) -> DATAIN_W<CRYP_DINrs> {
        DATAIN_W::new(self, 0)
    }
}
#[doc = "The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_din::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_din::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_DINrs;
impl crate::RegisterSpec for CRYP_DINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_din::R`](R) reader structure"]
impl crate::Readable for CRYP_DINrs {}
#[doc = "`write(|w| ..)` method takes [`cryp_din::W`](W) writer structure"]
impl crate::Writable for CRYP_DINrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_DIN to value 0"]
impl crate::Resettable for CRYP_DINrs {
    const RESET_VALUE: u32 = 0;
}
