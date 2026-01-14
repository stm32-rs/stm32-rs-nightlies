///Register `PCROP2ASR` reader
pub type R = crate::R<PCROP2ASRrs>;
///Register `PCROP2ASR` writer
pub type W = crate::W<PCROP2ASRrs>;
///Field `PCROP2A_STRT` reader - PCROP2A area start offset, bank2
pub type PCROP2A_STRT_R = crate::FieldReader<u16>;
///Field `PCROP2A_STRT` writer - PCROP2A area start offset, bank2
pub type PCROP2A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - PCROP2A area start offset, bank2
    #[inline(always)]
    pub fn pcrop2a_strt(&self) -> PCROP2A_STRT_R {
        PCROP2A_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP2ASR")
            .field("pcrop2a_strt", &self.pcrop2a_strt())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - PCROP2A area start offset, bank2
    #[inline(always)]
    pub fn pcrop2a_strt(&mut self) -> PCROP2A_STRT_W<'_, PCROP2ASRrs> {
        PCROP2A_STRT_W::new(self, 0)
    }
}
/**Flash PCROP2 area A start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop2asr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2asr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#FLASH:PCROP2ASR)*/
pub struct PCROP2ASRrs;
impl crate::RegisterSpec for PCROP2ASRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop2asr::R`](R) reader structure
impl crate::Readable for PCROP2ASRrs {}
///`write(|w| ..)` method takes [`pcrop2asr::W`](W) writer structure
impl crate::Writable for PCROP2ASRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP2ASR to value 0
impl crate::Resettable for PCROP2ASRrs {}
