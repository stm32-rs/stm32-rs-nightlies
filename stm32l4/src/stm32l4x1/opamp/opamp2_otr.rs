#[doc = "Register `OPAMP2_OTR` reader"]
pub type R = crate::R<OPAMP2_OTRrs>;
#[doc = "Register `OPAMP2_OTR` writer"]
pub type W = crate::W<OPAMP2_OTRrs>;
#[doc = "Field `TRIMOFFSETN` reader - Trim for NMOS differential pairs"]
pub type TRIMOFFSETN_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - Trim for NMOS differential pairs"]
pub type TRIMOFFSETN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETP` reader - Trim for PMOS differential pairs"]
pub type TRIMOFFSETP_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - Trim for PMOS differential pairs"]
pub type TRIMOFFSETP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<OPAMP2_OTRrs> {
        TRIMOFFSETN_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<OPAMP2_OTRrs> {
        TRIMOFFSETP_W::new(self, 8)
    }
}
#[doc = "OPAMP2 offset trimming register in normal mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_otr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_otr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP2_OTRrs;
impl crate::RegisterSpec for OPAMP2_OTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_otr::R`](R) reader structure"]
impl crate::Readable for OPAMP2_OTRrs {}
#[doc = "`write(|w| ..)` method takes [`opamp2_otr::W`](W) writer structure"]
impl crate::Writable for OPAMP2_OTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP2_OTR to value 0"]
impl crate::Resettable for OPAMP2_OTRrs {
    const RESET_VALUE: u32 = 0;
}
