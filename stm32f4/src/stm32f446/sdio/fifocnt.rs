#[doc = "Register `FIFOCNT` reader"]
pub type R = crate::R<FIFOCNTrs>;
#[doc = "Field `FIFOCOUNT` reader - Remaining number of words to be written to or read from the FIFO"]
pub type FIFOCOUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Remaining number of words to be written to or read from the FIFO"]
    #[inline(always)]
    pub fn fifocount(&self) -> FIFOCOUNT_R {
        FIFOCOUNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifocnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOCNTrs;
impl crate::RegisterSpec for FIFOCNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifocnt::R`](R) reader structure"]
impl crate::Readable for FIFOCNTrs {}
#[doc = "`reset()` method sets FIFOCNT to value 0"]
impl crate::Resettable for FIFOCNTrs {
    const RESET_VALUE: u32 = 0;
}
