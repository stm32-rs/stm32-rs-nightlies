#[doc = "Register `SECWM2R_PRG` reader"]
pub type R = crate::R<SECWM2R_PRGrs>;
#[doc = "Register `SECWM2R_PRG` writer"]
pub type W = crate::W<SECWM2R_PRGrs>;
#[doc = "Field `SECWM_STRT2` reader - Bank 2 security WM area start sector"]
pub type SECWM_STRT2_R = crate::FieldReader;
#[doc = "Field `SECWM_STRT2` writer - Bank 2 security WM area start sector"]
pub type SECWM_STRT2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECWM_END2` reader - Bank 2 security WM area end sector"]
pub type SECWM_END2_R = crate::FieldReader;
#[doc = "Field `SECWM_END2` writer - Bank 2 security WM area end sector"]
pub type SECWM_END2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Bank 2 security WM area start sector"]
    #[inline(always)]
    pub fn secwm_strt2(&self) -> SECWM_STRT2_R {
        SECWM_STRT2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank 2 security WM area end sector"]
    #[inline(always)]
    pub fn secwm_end2(&self) -> SECWM_END2_R {
        SECWM_END2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank 2 security WM area start sector"]
    #[inline(always)]
    #[must_use]
    pub fn secwm_strt2(&mut self) -> SECWM_STRT2_W<SECWM2R_PRGrs> {
        SECWM_STRT2_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Bank 2 security WM area end sector"]
    #[inline(always)]
    #[must_use]
    pub fn secwm_end2(&mut self) -> SECWM_END2_W<SECWM2R_PRGrs> {
        SECWM_END2_W::new(self, 16)
    }
}
#[doc = "FLASH security watermark for Bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm2r_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secwm2r_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECWM2R_PRGrs;
impl crate::RegisterSpec for SECWM2R_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secwm2r_prg::R`](R) reader structure"]
impl crate::Readable for SECWM2R_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`secwm2r_prg::W`](W) writer structure"]
impl crate::Writable for SECWM2R_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECWM2R_PRG to value 0"]
impl crate::Resettable for SECWM2R_PRGrs {
    const RESET_VALUE: u32 = 0;
}
