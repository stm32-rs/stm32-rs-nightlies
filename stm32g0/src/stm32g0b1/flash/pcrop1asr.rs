///Register `PCROP1ASR` reader
pub type R = crate::R<PCROP1ASRrs>;
///Field `PCROP1A_STRT` reader - PCROP1A area start offset
pub type PCROP1A_STRT_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PCROP1A area start offset
    #[inline(always)]
    pub fn pcrop1a_strt(&self) -> PCROP1A_STRT_R {
        PCROP1A_STRT_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1ASR")
            .field("pcrop1a_strt", &self.pcrop1a_strt())
            .finish()
    }
}
/**Flash PCROP zone A Start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1asr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#FLASH:PCROP1ASR)*/
pub struct PCROP1ASRrs;
impl crate::RegisterSpec for PCROP1ASRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1asr::R`](R) reader structure
impl crate::Readable for PCROP1ASRrs {}
///`reset()` method sets PCROP1ASR to value 0xf000_0000
impl crate::Resettable for PCROP1ASRrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}
