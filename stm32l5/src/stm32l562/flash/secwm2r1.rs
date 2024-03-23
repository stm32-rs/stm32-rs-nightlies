#[doc = "Register `SECWM2R1` reader"]
pub type R = crate::R<SECWM2R1rs>;
#[doc = "Register `SECWM2R1` writer"]
pub type W = crate::W<SECWM2R1rs>;
#[doc = "Field `SECWM2_PSTRT` reader - SECWM2_PSTRT"]
pub type SECWM2_PSTRT_R = crate::FieldReader;
#[doc = "Field `SECWM2_PSTRT` writer - SECWM2_PSTRT"]
pub type SECWM2_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECWM2_PEND` reader - SECWM2_PEND"]
pub type SECWM2_PEND_R = crate::FieldReader;
#[doc = "Field `SECWM2_PEND` writer - SECWM2_PEND"]
pub type SECWM2_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - SECWM2_PSTRT"]
    #[inline(always)]
    pub fn secwm2_pstrt(&self) -> SECWM2_PSTRT_R {
        SECWM2_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - SECWM2_PEND"]
    #[inline(always)]
    pub fn secwm2_pend(&self) -> SECWM2_PEND_R {
        SECWM2_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SECWM2_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn secwm2_pstrt(&mut self) -> SECWM2_PSTRT_W<SECWM2R1rs> {
        SECWM2_PSTRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - SECWM2_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn secwm2_pend(&mut self) -> SECWM2_PEND_W<SECWM2R1rs> {
        SECWM2_PEND_W::new(self, 16)
    }
}
#[doc = "Flash secure watermak2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm2r1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secwm2r1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECWM2R1rs;
impl crate::RegisterSpec for SECWM2R1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secwm2r1::R`](R) reader structure"]
impl crate::Readable for SECWM2R1rs {}
#[doc = "`write(|w| ..)` method takes [`secwm2r1::W`](W) writer structure"]
impl crate::Writable for SECWM2R1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECWM2R1 to value 0xff00_ff00"]
impl crate::Resettable for SECWM2R1rs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
