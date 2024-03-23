#[doc = "Register `STGENC_CNTFID0` reader"]
pub type R = crate::R<STGENC_CNTFID0rs>;
#[doc = "Register `STGENC_CNTFID0` writer"]
pub type W = crate::W<STGENC_CNTFID0rs>;
#[doc = "Field `FREQ` reader - FREQ"]
pub type FREQ_R = crate::FieldReader<u32>;
#[doc = "Field `FREQ` writer - FREQ"]
pub type FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FREQ"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<STGENC_CNTFID0rs> {
        FREQ_W::new(self, 0)
    }
}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenc_cntfid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stgenc_cntfid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENC_CNTFID0rs;
impl crate::RegisterSpec for STGENC_CNTFID0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenc_cntfid0::R`](R) reader structure"]
impl crate::Readable for STGENC_CNTFID0rs {}
#[doc = "`write(|w| ..)` method takes [`stgenc_cntfid0::W`](W) writer structure"]
impl crate::Writable for STGENC_CNTFID0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STGENC_CNTFID0 to value 0"]
impl crate::Resettable for STGENC_CNTFID0rs {
    const RESET_VALUE: u32 = 0;
}
