///Register `PCROP1ASR` reader
pub type R = crate::R<PCROP1ASRrs>;
///Register `PCROP1ASR` writer
pub type W = crate::W<PCROP1ASRrs>;
///Field `PCROP1A_STRT` reader - Bank 1 PCROPQ area start offset
pub type PCROP1A_STRT_R = crate::FieldReader<u16>;
///Field `PCROP1A_STRT` writer - Bank 1 PCROPQ area start offset
pub type PCROP1A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Bank 1 PCROPQ area start offset
    #[inline(always)]
    pub fn pcrop1a_strt(&self) -> PCROP1A_STRT_R {
        PCROP1A_STRT_R::new((self.bits & 0x01ff) as u16)
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
    ///Bits 0:8 - Bank 1 PCROPQ area start offset
    #[inline(always)]
    pub fn pcrop1a_strt(&mut self) -> PCROP1A_STRT_W<'_, PCROP1ASRrs> {
        PCROP1A_STRT_W::new(self, 0)
    }
}
/**Flash Bank 1 PCROP Start address zone A register

You can [`read`](crate::Reg::read) this register and get [`pcrop1asr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1asr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:PCROP1ASR)*/
pub struct PCROP1ASRrs;
impl crate::RegisterSpec for PCROP1ASRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1asr::R`](R) reader structure
impl crate::Readable for PCROP1ASRrs {}
///`write(|w| ..)` method takes [`pcrop1asr::W`](W) writer structure
impl crate::Writable for PCROP1ASRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP1ASR to value 0xffff_fe00
impl crate::Resettable for PCROP1ASRrs {
    const RESET_VALUE: u32 = 0xffff_fe00;
}
