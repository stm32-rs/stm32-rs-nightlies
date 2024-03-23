#[doc = "Register `SECWM1R_PRG` reader"]
pub type R = crate::R<SECWM1R_PRGrs>;
#[doc = "Register `SECWM1R_PRG` writer"]
pub type W = crate::W<SECWM1R_PRGrs>;
#[doc = "Field `SECWM1_STRT` reader - Bank1 security WM area 1 start sector"]
pub type SECWM1_STRT_R = crate::FieldReader;
#[doc = "Field `SECWM1_STRT` writer - Bank1 security WM area 1 start sector"]
pub type SECWM1_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECWM1_END` reader - Bank1 security WM area 1 end sector"]
pub type SECWM1_END_R = crate::FieldReader;
#[doc = "Field `SECWM1_END` writer - Bank1 security WM area 1 end sector"]
pub type SECWM1_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Bank1 security WM area 1 start sector"]
    #[inline(always)]
    pub fn secwm1_strt(&self) -> SECWM1_STRT_R {
        SECWM1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank1 security WM area 1 end sector"]
    #[inline(always)]
    pub fn secwm1_end(&self) -> SECWM1_END_R {
        SECWM1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank1 security WM area 1 start sector"]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_strt(&mut self) -> SECWM1_STRT_W<SECWM1R_PRGrs> {
        SECWM1_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Bank1 security WM area 1 end sector"]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_end(&mut self) -> SECWM1_END_W<SECWM1R_PRGrs> {
        SECWM1_END_W::new(self, 16)
    }
}
#[doc = "FLASH security watermark for Bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm1r_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secwm1r_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECWM1R_PRGrs;
impl crate::RegisterSpec for SECWM1R_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secwm1r_prg::R`](R) reader structure"]
impl crate::Readable for SECWM1R_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`secwm1r_prg::W`](W) writer structure"]
impl crate::Writable for SECWM1R_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECWM1R_PRG to value 0"]
impl crate::Resettable for SECWM1R_PRGrs {
    const RESET_VALUE: u32 = 0;
}
