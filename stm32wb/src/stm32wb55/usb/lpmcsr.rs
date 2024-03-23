#[doc = "Register `LPMCSR` reader"]
pub type R = crate::R<LPMCSRrs>;
#[doc = "Register `LPMCSR` writer"]
pub type W = crate::W<LPMCSRrs>;
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub type LPMEN_R = crate::BitReader;
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub type LPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMACK` reader - LPM Token acknowledge enable"]
pub type LPMACK_R = crate::BitReader;
#[doc = "Field `LPMACK` writer - LPM Token acknowledge enable"]
pub type LPMACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMWAKE` reader - RemoteWake value"]
pub type REMWAKE_R = crate::BitReader;
#[doc = "Field `REMWAKE` writer - RemoteWake value"]
pub type REMWAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BESL` reader - BESL value"]
pub type BESL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM Token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RemoteWake value"]
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - BESL value"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<LPMCSRrs> {
        LPMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPM Token acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmack(&mut self) -> LPMACK_W<LPMCSRrs> {
        LPMACK_W::new(self, 1)
    }
    #[doc = "Bit 3 - RemoteWake value"]
    #[inline(always)]
    #[must_use]
    pub fn remwake(&mut self) -> REMWAKE_W<LPMCSRrs> {
        REMWAKE_W::new(self, 3)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMCSRrs;
impl crate::RegisterSpec for LPMCSRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lpmcsr::R`](R) reader structure"]
impl crate::Readable for LPMCSRrs {}
#[doc = "`write(|w| ..)` method takes [`lpmcsr::W`](W) writer structure"]
impl crate::Writable for LPMCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets LPMCSR to value 0"]
impl crate::Resettable for LPMCSRrs {
    const RESET_VALUE: u16 = 0;
}
