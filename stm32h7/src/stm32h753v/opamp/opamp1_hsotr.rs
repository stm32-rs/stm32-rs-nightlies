#[doc = "Register `OPAMP1_HSOTR` reader"]
pub type R = crate::R<OPAMP1_HSOTRrs>;
#[doc = "Register `OPAMP1_HSOTR` writer"]
pub type W = crate::W<OPAMP1_HSOTRrs>;
#[doc = "Field `TRIMLPOFFSETN` reader - Trim for NMOS differential pairs"]
pub type TRIMLPOFFSETN_R = crate::FieldReader;
#[doc = "Field `TRIMLPOFFSETN` writer - Trim for NMOS differential pairs"]
pub type TRIMLPOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMLPOFFSETP` reader - Trim for PMOS differential pairs"]
pub type TRIMLPOFFSETP_R = crate::FieldReader;
#[doc = "Field `TRIMLPOFFSETP` writer - Trim for PMOS differential pairs"]
pub type TRIMLPOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetn(&self) -> TRIMLPOFFSETN_R {
        TRIMLPOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetp(&self) -> TRIMLPOFFSETP_R {
        TRIMLPOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimlpoffsetn(&mut self) -> TRIMLPOFFSETN_W<OPAMP1_HSOTRrs> {
        TRIMLPOFFSETN_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimlpoffsetp(&mut self) -> TRIMLPOFFSETP_W<OPAMP1_HSOTRrs> {
        TRIMLPOFFSETP_W::new(self, 8)
    }
}
#[doc = "OPAMP1 offset trimming register in low-power mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp1_hsotr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp1_hsotr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP1_HSOTRrs;
impl crate::RegisterSpec for OPAMP1_HSOTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp1_hsotr::R`](R) reader structure"]
impl crate::Readable for OPAMP1_HSOTRrs {}
#[doc = "`write(|w| ..)` method takes [`opamp1_hsotr::W`](W) writer structure"]
impl crate::Writable for OPAMP1_HSOTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP1_HSOTR to value 0"]
impl crate::Resettable for OPAMP1_HSOTRrs {
    const RESET_VALUE: u32 = 0;
}
