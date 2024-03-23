#[doc = "Register `SDRTR` reader"]
pub type R = crate::R<SDRTRrs>;
#[doc = "Register `SDRTR` writer"]
pub type W = crate::W<SDRTRrs>;
#[doc = "Clear Refresh error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRE {
    #[doc = "1: Refresh Error Flag is cleared"]
    Clear = 1,
}
impl From<CRE> for bool {
    #[inline(always)]
    fn from(variant: CRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRE` writer - Clear Refresh error flag"]
pub type CRE_W<'a, REG> = crate::BitWriter<'a, REG, CRE>;
impl<'a, REG> CRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Refresh Error Flag is cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CRE::Clear)
    }
}
#[doc = "Field `COUNT` reader - Refresh Timer Count"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Refresh Timer Count"]
pub type COUNT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 13, u16>;
#[doc = "RES Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated if RE = 1"]
    Enabled = 1,
}
impl From<REIE> for bool {
    #[inline(always)]
    fn from(variant: REIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REIE` reader - RES Interrupt Enable"]
pub type REIE_R = crate::BitReader<REIE>;
impl REIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REIE {
        match self.bits {
            false => REIE::Disabled,
            true => REIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REIE::Disabled
    }
    #[doc = "Interrupt is generated if RE = 1"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REIE::Enabled
    }
}
#[doc = "Field `REIE` writer - RES Interrupt Enable"]
pub type REIE_W<'a, REG> = crate::BitWriter<'a, REG, REIE>;
impl<'a, REG> REIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REIE::Disabled)
    }
    #[doc = "Interrupt is generated if RE = 1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REIE::Enabled)
    }
}
impl R {
    #[doc = "Bits 1:13 - Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - RES Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Refresh error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cre(&mut self) -> CRE_W<SDRTRrs> {
        CRE_W::new(self, 0)
    }
    #[doc = "Bits 1:13 - Refresh Timer Count"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<SDRTRrs> {
        COUNT_W::new(self, 1)
    }
    #[doc = "Bit 14 - RES Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reie(&mut self) -> REIE_W<SDRTRrs> {
        REIE_W::new(self, 14)
    }
}
#[doc = "SDRAM Refresh Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDRTRrs;
impl crate::RegisterSpec for SDRTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrtr::R`](R) reader structure"]
impl crate::Readable for SDRTRrs {}
#[doc = "`write(|w| ..)` method takes [`sdrtr::W`](W) writer structure"]
impl crate::Writable for SDRTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDRTR to value 0"]
impl crate::Resettable for SDRTRrs {
    const RESET_VALUE: u32 = 0;
}
