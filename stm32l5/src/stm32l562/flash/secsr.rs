#[doc = "Register `SECSR` reader"]
pub type R = crate::R<SECSRrs>;
#[doc = "Register `SECSR` writer"]
pub type W = crate::W<SECSRrs>;
#[doc = "Field `SECEOP` reader - SECEOP"]
pub type SECEOP_R = crate::BitReader;
#[doc = "Field `SECEOP` writer - SECEOP"]
pub type SECEOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECOPERR` reader - SECOPERR"]
pub type SECOPERR_R = crate::BitReader;
#[doc = "Field `SECOPERR` writer - SECOPERR"]
pub type SECOPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECPROGERR` reader - SECPROGERR"]
pub type SECPROGERR_R = crate::BitReader;
#[doc = "Field `SECPROGERR` writer - SECPROGERR"]
pub type SECPROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECWRPERR` reader - SECWRPERR"]
pub type SECWRPERR_R = crate::BitReader;
#[doc = "Field `SECWRPERR` writer - SECWRPERR"]
pub type SECWRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECPGAERR` reader - SECPGAERR"]
pub type SECPGAERR_R = crate::BitReader;
#[doc = "Field `SECPGAERR` writer - SECPGAERR"]
pub type SECPGAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECSIZERR` reader - SECSIZERR"]
pub type SECSIZERR_R = crate::BitReader;
#[doc = "Field `SECSIZERR` writer - SECSIZERR"]
pub type SECSIZERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECPGSERR` reader - SECPGSERR"]
pub type SECPGSERR_R = crate::BitReader;
#[doc = "Field `SECPGSERR` writer - SECPGSERR"]
pub type SECPGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECRDERR` reader - Secure read protection error"]
pub type SECRDERR_R = crate::BitReader;
#[doc = "Field `SECRDERR` writer - Secure read protection error"]
pub type SECRDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECBSY` reader - SECBusy"]
pub type SECBSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SECEOP"]
    #[inline(always)]
    pub fn seceop(&self) -> SECEOP_R {
        SECEOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SECOPERR"]
    #[inline(always)]
    pub fn secoperr(&self) -> SECOPERR_R {
        SECOPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SECPROGERR"]
    #[inline(always)]
    pub fn secprogerr(&self) -> SECPROGERR_R {
        SECPROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SECWRPERR"]
    #[inline(always)]
    pub fn secwrperr(&self) -> SECWRPERR_R {
        SECWRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SECPGAERR"]
    #[inline(always)]
    pub fn secpgaerr(&self) -> SECPGAERR_R {
        SECPGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SECSIZERR"]
    #[inline(always)]
    pub fn secsizerr(&self) -> SECSIZERR_R {
        SECSIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SECPGSERR"]
    #[inline(always)]
    pub fn secpgserr(&self) -> SECPGSERR_R {
        SECPGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - Secure read protection error"]
    #[inline(always)]
    pub fn secrderr(&self) -> SECRDERR_R {
        SECRDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SECBusy"]
    #[inline(always)]
    pub fn secbsy(&self) -> SECBSY_R {
        SECBSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SECEOP"]
    #[inline(always)]
    #[must_use]
    pub fn seceop(&mut self) -> SECEOP_W<SECSRrs> {
        SECEOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - SECOPERR"]
    #[inline(always)]
    #[must_use]
    pub fn secoperr(&mut self) -> SECOPERR_W<SECSRrs> {
        SECOPERR_W::new(self, 1)
    }
    #[doc = "Bit 3 - SECPROGERR"]
    #[inline(always)]
    #[must_use]
    pub fn secprogerr(&mut self) -> SECPROGERR_W<SECSRrs> {
        SECPROGERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - SECWRPERR"]
    #[inline(always)]
    #[must_use]
    pub fn secwrperr(&mut self) -> SECWRPERR_W<SECSRrs> {
        SECWRPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - SECPGAERR"]
    #[inline(always)]
    #[must_use]
    pub fn secpgaerr(&mut self) -> SECPGAERR_W<SECSRrs> {
        SECPGAERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - SECSIZERR"]
    #[inline(always)]
    #[must_use]
    pub fn secsizerr(&mut self) -> SECSIZERR_W<SECSRrs> {
        SECSIZERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - SECPGSERR"]
    #[inline(always)]
    #[must_use]
    pub fn secpgserr(&mut self) -> SECPGSERR_W<SECSRrs> {
        SECPGSERR_W::new(self, 7)
    }
    #[doc = "Bit 14 - Secure read protection error"]
    #[inline(always)]
    #[must_use]
    pub fn secrderr(&mut self) -> SECRDERR_W<SECSRrs> {
        SECRDERR_W::new(self, 14)
    }
}
#[doc = "Flash status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECSRrs;
impl crate::RegisterSpec for SECSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secsr::R`](R) reader structure"]
impl crate::Readable for SECSRrs {}
#[doc = "`write(|w| ..)` method takes [`secsr::W`](W) writer structure"]
impl crate::Writable for SECSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECSR to value 0"]
impl crate::Resettable for SECSRrs {
    const RESET_VALUE: u32 = 0;
}
