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
#[doc = "FIFO counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifocnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
