#[doc = "Register `SECWM1R1` reader"]
pub type R = crate::R<SECWM1R1rs>;
#[doc = "Register `SECWM1R1` writer"]
pub type W = crate::W<SECWM1R1rs>;
#[doc = "Field `SECWM1_PSTRT` reader - SECWM1_PSTRT"]
pub type SECWM1_PSTRT_R = crate::FieldReader;
#[doc = "Field `SECWM1_PSTRT` writer - SECWM1_PSTRT"]
pub type SECWM1_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECWM1_PEND` reader - SECWM1_PEND"]
pub type SECWM1_PEND_R = crate::FieldReader;
#[doc = "Field `SECWM1_PEND` writer - SECWM1_PEND"]
pub type SECWM1_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - SECWM1_PSTRT"]
    #[inline(always)]
    pub fn secwm1_pstrt(&self) -> SECWM1_PSTRT_R {
        SECWM1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - SECWM1_PEND"]
    #[inline(always)]
    pub fn secwm1_pend(&self) -> SECWM1_PEND_R {
        SECWM1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SECWM1_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_pstrt(&mut self) -> SECWM1_PSTRT_W<SECWM1R1rs> {
        SECWM1_PSTRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - SECWM1_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_pend(&mut self) -> SECWM1_PEND_W<SECWM1R1rs> {
        SECWM1_PEND_W::new(self, 16)
    }
}
#[doc = "Flash bank 1 secure watermak1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm1r1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secwm1r1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECWM1R1rs;
impl crate::RegisterSpec for SECWM1R1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secwm1r1::R`](R) reader structure"]
impl crate::Readable for SECWM1R1rs {}
#[doc = "`write(|w| ..)` method takes [`secwm1r1::W`](W) writer structure"]
impl crate::Writable for SECWM1R1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECWM1R1 to value 0xff00_ff00"]
impl crate::Resettable for SECWM1R1rs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
