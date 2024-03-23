#[doc = "Register `PCROP1ASR` reader"]
pub type R = crate::R<PCROP1ASRrs>;
#[doc = "Register `PCROP1ASR` writer"]
pub type W = crate::W<PCROP1ASRrs>;
#[doc = "Field `PCROP1A_STRT` reader - PCROP1A area start offset"]
pub type PCROP1A_STRT_R = crate::FieldReader;
#[doc = "Field `PCROP1A_STRT` writer - PCROP1A area start offset"]
pub type PCROP1A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PCROP1A area start offset"]
    #[inline(always)]
    pub fn pcrop1a_strt(&self) -> PCROP1A_STRT_R {
        PCROP1A_STRT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PCROP1A area start offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1a_strt(&mut self) -> PCROP1A_STRT_W<PCROP1ASRrs> {
        PCROP1A_STRT_W::new(self, 0)
    }
}
#[doc = "Flash PCROP zone A Start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1asr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1asr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP1ASRrs;
impl crate::RegisterSpec for PCROP1ASRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1asr::R`](R) reader structure"]
impl crate::Readable for PCROP1ASRrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop1asr::W`](W) writer structure"]
impl crate::Writable for PCROP1ASRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP1ASR to value 0xffff_ffff"]
impl crate::Resettable for PCROP1ASRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
