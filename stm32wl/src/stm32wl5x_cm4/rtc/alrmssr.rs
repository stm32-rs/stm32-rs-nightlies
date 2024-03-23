#[doc = "Register `ALRM%sSSR` reader"]
pub type R = crate::R<ALRMSSRrs>;
#[doc = "Register `ALRM%sSSR` writer"]
pub type W = crate::W<ALRMSSRrs>;
#[doc = "Field `SS` reader - Sub seconds value"]
pub type SS_R = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Sub seconds value"]
pub type SS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 15, u16>;
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit"]
pub type MASKSS_R = crate::FieldReader;
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit"]
pub type MASKSS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Clear synchronous counter on alarm (Binary mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCLR {
    #[doc = "0: The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is free-running"]
    FreeRunning = 0,
    #[doc = "1: The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR → SS\\[31:0\\]
value and is automatically reloaded with 0xFFFF FFFF when reaching RTC_ALRMABINR → SS\\[31:0\\]"]
    Alrmbinr = 1,
}
impl From<SSCLR> for bool {
    #[inline(always)]
    fn from(variant: SSCLR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCLR` reader - Clear synchronous counter on alarm (Binary mode only)"]
pub type SSCLR_R = crate::BitReader<SSCLR>;
impl SSCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCLR {
        match self.bits {
            false => SSCLR::FreeRunning,
            true => SSCLR::Alrmbinr,
        }
    }
    #[doc = "The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is free-running"]
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        *self == SSCLR::FreeRunning
    }
    #[doc = "The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR → SS\\[31:0\\]
value and is automatically reloaded with 0xFFFF FFFF when reaching RTC_ALRMABINR → SS\\[31:0\\]"]
    #[inline(always)]
    pub fn is_alrmbinr(&self) -> bool {
        *self == SSCLR::Alrmbinr
    }
}
#[doc = "Field `SSCLR` writer - Clear synchronous counter on alarm (Binary mode only)"]
pub type SSCLR_W<'a, REG> = crate::BitWriter<'a, REG, SSCLR>;
impl<'a, REG> SSCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is free-running"]
    #[inline(always)]
    pub fn free_running(self) -> &'a mut crate::W<REG> {
        self.variant(SSCLR::FreeRunning)
    }
    #[doc = "The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR → SS\\[31:0\\]
value and is automatically reloaded with 0xFFFF FFFF when reaching RTC_ALRMABINR → SS\\[31:0\\]"]
    #[inline(always)]
    pub fn alrmbinr(self) -> &'a mut crate::W<REG> {
        self.variant(SSCLR::Alrmbinr)
    }
}
impl R {
    #[doc = "Bits 0:14 - Sub seconds value"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:29 - Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Clear synchronous counter on alarm (Binary mode only)"]
    #[inline(always)]
    pub fn ssclr(&self) -> SSCLR_R {
        SSCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub seconds value"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<ALRMSSRrs> {
        SS_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MASKSS_W<ALRMSSRrs> {
        MASKSS_W::new(self, 24)
    }
    #[doc = "Bit 31 - Clear synchronous counter on alarm (Binary mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn ssclr(&mut self) -> SSCLR_W<ALRMSSRrs> {
        SSCLR_W::new(self, 31)
    }
}
#[doc = "Alarm %s sub-second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMSSRrs;
impl crate::RegisterSpec for ALRMSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmssr::R`](R) reader structure"]
impl crate::Readable for ALRMSSRrs {}
#[doc = "`write(|w| ..)` method takes [`alrmssr::W`](W) writer structure"]
impl crate::Writable for ALRMSSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRM%sSSR to value 0"]
impl crate::Resettable for ALRMSSRrs {
    const RESET_VALUE: u32 = 0;
}
