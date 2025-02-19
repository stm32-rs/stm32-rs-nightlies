///Register `PCROP1ASR` reader
pub type R = crate::R<PCROP1ASRrs>;
///Register `PCROP1ASR` writer
pub type W = crate::W<PCROP1ASRrs>;
///Field `PCROP1A_STRT` reader - PCROP1A area start offset Contains the offset of the first subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type PCROP1A_STRT_R = crate::FieldReader;
///Field `PCROP1A_STRT` writer - PCROP1A area start offset Contains the offset of the first subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type PCROP1A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - PCROP1A area start offset Contains the offset of the first subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
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
impl W {
    ///Bits 0:7 - PCROP1A area start offset Contains the offset of the first subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn pcrop1a_strt(&mut self) -> PCROP1A_STRT_W<PCROP1ASRrs> {
        PCROP1A_STRT_W::new(self, 0)
    }
}
/**FLASH PCROP area A start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1asr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1asr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#FLASH:PCROP1ASR)*/
pub struct PCROP1ASRrs;
impl crate::RegisterSpec for PCROP1ASRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1asr::R`](R) reader structure
impl crate::Readable for PCROP1ASRrs {}
///`write(|w| ..)` method takes [`pcrop1asr::W`](W) writer structure
impl crate::Writable for PCROP1ASRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PCROP1ASR to value 0
impl crate::Resettable for PCROP1ASRrs {
    const RESET_VALUE: u32 = 0;
}
