#[doc = "Register `MISR` reader"]
pub type R = crate::R<MISRrs>;
#[doc = "Register `MISR` writer"]
pub type W = crate::W<MISRrs>;
#[doc = "Field `ALRAMF` reader - Alarm A masked flag"]
pub type ALRAMF_R = crate::BitReader;
#[doc = "Field `ALRAMF` writer - Alarm A masked flag"]
pub type ALRAMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBMF` reader - Alarm B masked flag"]
pub type ALRBMF_R = crate::BitReader;
#[doc = "Field `ALRBMF` writer - Alarm B masked flag"]
pub type ALRBMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTMF` reader - Wakeup timer masked flag"]
pub type WUTMF_R = crate::BitReader;
#[doc = "Field `WUTMF` writer - Wakeup timer masked flag"]
pub type WUTMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMF` reader - Timestamp masked flag"]
pub type TSMF_R = crate::BitReader;
#[doc = "Field `TSMF` writer - Timestamp masked flag"]
pub type TSMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSOVMF` reader - Timestamp overflow masked flag"]
pub type TSOVMF_R = crate::BitReader;
#[doc = "Field `TSOVMF` writer - Timestamp overflow masked flag"]
pub type TSOVMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITSMF` reader - Internal timestamp masked flag"]
pub type ITSMF_R = crate::BitReader;
#[doc = "Field `ITSMF` writer - Internal timestamp masked flag"]
pub type ITSMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSRUMF` reader - SSR underflow masked flag"]
pub type SSRUMF_R = crate::BitReader;
#[doc = "Field `SSRUMF` writer - SSR underflow masked flag"]
pub type SSRUMF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alarm A masked flag"]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B masked flag"]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag"]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp masked flag"]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag"]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SSR underflow masked flag"]
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm A masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn alramf(&mut self) -> ALRAMF_W<MISRrs> {
        ALRAMF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm B masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrbmf(&mut self) -> ALRBMF_W<MISRrs> {
        ALRBMF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn wutmf(&mut self) -> WUTMF_W<MISRrs> {
        WUTMF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsmf(&mut self) -> TSMF_W<MISRrs> {
        TSMF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsovmf(&mut self) -> TSOVMF_W<MISRrs> {
        TSOVMF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn itsmf(&mut self) -> ITSMF_W<MISRrs> {
        ITSMF_W::new(self, 5)
    }
    #[doc = "Bit 6 - SSR underflow masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssrumf(&mut self) -> SSRUMF_W<MISRrs> {
        SSRUMF_W::new(self, 6)
    }
}
#[doc = "RTC masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MISRrs {}
#[doc = "`write(|w| ..)` method takes [`misr::W`](W) writer structure"]
impl crate::Writable for MISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}
