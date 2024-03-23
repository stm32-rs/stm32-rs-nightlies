#[doc = "Register `STGENC_CNTCVL` reader"]
pub type R = crate::R<STGENC_CNTCVLrs>;
#[doc = "Register `STGENC_CNTCVL` writer"]
pub type W = crate::W<STGENC_CNTCVLrs>;
#[doc = "Field `CNTCVL_L_32` reader - CNTCVL_L_32"]
pub type CNTCVL_L_32_R = crate::FieldReader<u32>;
#[doc = "Field `CNTCVL_L_32` writer - CNTCVL_L_32"]
pub type CNTCVL_L_32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CNTCVL_L_32"]
    #[inline(always)]
    pub fn cntcvl_l_32(&self) -> CNTCVL_L_32_R {
        CNTCVL_L_32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTCVL_L_32"]
    #[inline(always)]
    #[must_use]
    pub fn cntcvl_l_32(&mut self) -> CNTCVL_L_32_W<STGENC_CNTCVLrs> {
        CNTCVL_L_32_W::new(self, 0)
    }
}
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenc_cntcvl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stgenc_cntcvl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENC_CNTCVLrs;
impl crate::RegisterSpec for STGENC_CNTCVLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenc_cntcvl::R`](R) reader structure"]
impl crate::Readable for STGENC_CNTCVLrs {}
#[doc = "`write(|w| ..)` method takes [`stgenc_cntcvl::W`](W) writer structure"]
impl crate::Writable for STGENC_CNTCVLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STGENC_CNTCVL to value 0"]
impl crate::Resettable for STGENC_CNTCVLrs {
    const RESET_VALUE: u32 = 0;
}
