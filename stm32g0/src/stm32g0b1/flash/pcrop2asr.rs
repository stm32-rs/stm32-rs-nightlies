#[doc = "Register `PCROP2ASR` reader"]
pub type R = crate::R<PCROP2ASRrs>;
#[doc = "Register `PCROP2ASR` writer"]
pub type W = crate::W<PCROP2ASRrs>;
#[doc = "Field `PCROP2A_STRT` reader - PCROP2A area start offset, bank2"]
pub type PCROP2A_STRT_R = crate::FieldReader<u16>;
#[doc = "Field `PCROP2A_STRT` writer - PCROP2A area start offset, bank2"]
pub type PCROP2A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - PCROP2A area start offset, bank2"]
    #[inline(always)]
    pub fn pcrop2a_strt(&self) -> PCROP2A_STRT_R {
        PCROP2A_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PCROP2A area start offset, bank2"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2a_strt(&mut self) -> PCROP2A_STRT_W<PCROP2ASRrs> {
        PCROP2A_STRT_W::new(self, 0)
    }
}
#[doc = "Flash PCROP2 area A start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2asr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2asr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP2ASRrs;
impl crate::RegisterSpec for PCROP2ASRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop2asr::R`](R) reader structure"]
impl crate::Readable for PCROP2ASRrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop2asr::W`](W) writer structure"]
impl crate::Writable for PCROP2ASRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP2ASR to value 0xffff_ffff"]
impl crate::Resettable for PCROP2ASRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
