#[doc = "Register `LPMCSR` reader"]
pub type R = crate::R<LPMCSRrs>;
#[doc = "Register `LPMCSR` writer"]
pub type W = crate::W<LPMCSRrs>;
#[doc = "LPM support enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMEN {
    #[doc = "0: enable the LPM support within the USB device"]
    Disabled = 0,
    #[doc = "1: no LPM transactions are handled"]
    Enabled = 1,
}
impl From<LPMEN> for bool {
    #[inline(always)]
    fn from(variant: LPMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub type LPMEN_R = crate::BitReader<LPMEN>;
impl LPMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPMEN {
        match self.bits {
            false => LPMEN::Disabled,
            true => LPMEN::Enabled,
        }
    }
    #[doc = "enable the LPM support within the USB device"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPMEN::Disabled
    }
    #[doc = "no LPM transactions are handled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPMEN::Enabled
    }
}
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub type LPMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPMEN>;
impl<'a, REG> LPMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable the LPM support within the USB device"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPMEN::Disabled)
    }
    #[doc = "no LPM transactions are handled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPMEN::Enabled)
    }
}
#[doc = "LPM Token acknowledge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMACK {
    #[doc = "0: the valid LPM Token will be NYET"]
    Nyet = 0,
    #[doc = "1: the valid LPM Token will be ACK"]
    Ack = 1,
}
impl From<LPMACK> for bool {
    #[inline(always)]
    fn from(variant: LPMACK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMACK` reader - LPM Token acknowledge enable"]
pub type LPMACK_R = crate::BitReader<LPMACK>;
impl LPMACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPMACK {
        match self.bits {
            false => LPMACK::Nyet,
            true => LPMACK::Ack,
        }
    }
    #[doc = "the valid LPM Token will be NYET"]
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        *self == LPMACK::Nyet
    }
    #[doc = "the valid LPM Token will be ACK"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LPMACK::Ack
    }
}
#[doc = "Field `LPMACK` writer - LPM Token acknowledge enable"]
pub type LPMACK_W<'a, REG> = crate::BitWriter<'a, REG, LPMACK>;
impl<'a, REG> LPMACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the valid LPM Token will be NYET"]
    #[inline(always)]
    pub fn nyet(self) -> &'a mut crate::W<REG> {
        self.variant(LPMACK::Nyet)
    }
    #[doc = "the valid LPM Token will be ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(LPMACK::Ack)
    }
}
#[doc = "Field `REMWAKE` reader - bRemoteWake value"]
pub type REMWAKE_R = crate::BitReader;
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
    #[doc = "Bit 3 - bRemoteWake value"]
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
}
#[doc = "LPM control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMCSRrs;
impl crate::RegisterSpec for LPMCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmcsr::R`](R) reader structure"]
impl crate::Readable for LPMCSRrs {}
#[doc = "`write(|w| ..)` method takes [`lpmcsr::W`](W) writer structure"]
impl crate::Writable for LPMCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPMCSR to value 0"]
impl crate::Resettable for LPMCSRrs {
    const RESET_VALUE: u32 = 0;
}
