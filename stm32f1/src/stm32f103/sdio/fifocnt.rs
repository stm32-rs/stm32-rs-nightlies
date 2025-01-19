///Register `FIFOCNT` reader
pub type R = crate::R<FIFOCNTrs>;
///Field `FIF0COUNT` reader - FIF0COUNT
pub type FIF0COUNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - FIF0COUNT
    #[inline(always)]
    pub fn fif0count(&self) -> FIF0COUNT_R {
        FIF0COUNT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOCNT")
            .field("fif0count", &self.fif0count())
            .finish()
    }
}
/**Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO

You can [`read`](crate::Reg::read) this register and get [`fifocnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#SDIO:FIFOCNT)*/
pub struct FIFOCNTrs;
impl crate::RegisterSpec for FIFOCNTrs {
    type Ux = u32;
}
///`read()` method returns [`fifocnt::R`](R) reader structure
impl crate::Readable for FIFOCNTrs {}
///`reset()` method sets FIFOCNT to value 0
impl crate::Resettable for FIFOCNTrs {
    const RESET_VALUE: u32 = 0;
}
