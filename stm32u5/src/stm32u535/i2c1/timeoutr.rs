#[doc = "Register `TIMEOUTR` reader"]
pub type R = crate::R<TIMEOUTRrs>;
#[doc = "Register `TIMEOUTR` writer"]
pub type W = crate::W<TIMEOUTRrs>;
#[doc = "Field `TIMEOUTA` reader - Bus timeout A"]
pub type TIMEOUTA_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTA` writer - Bus timeout A"]
pub type TIMEOUTA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TIDLE` reader - Idle clock timeout detection"]
pub type TIDLE_R = crate::BitReader;
#[doc = "Field `TIDLE` writer - Idle clock timeout detection"]
pub type TIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMOUTEN` reader - Clock timeout enable"]
pub type TIMOUTEN_R = crate::BitReader;
#[doc = "Field `TIMOUTEN` writer - Clock timeout enable"]
pub type TIMOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTB` reader - Bus timeout B"]
pub type TIMEOUTB_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTB` writer - Bus timeout B"]
pub type TIMEOUTB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TEXTEN` reader - Extended clock timeout enable"]
pub type TEXTEN_R = crate::BitReader;
#[doc = "Field `TEXTEN` writer - Extended clock timeout enable"]
pub type TEXTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Bus timeout A"]
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Idle clock timeout detection"]
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Bus timeout B"]
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bus timeout A"]
    #[inline(always)]
    #[must_use]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<TIMEOUTRrs> {
        TIMEOUTA_W::new(self, 0)
    }
    #[doc = "Bit 12 - Idle clock timeout detection"]
    #[inline(always)]
    #[must_use]
    pub fn tidle(&mut self) -> TIDLE_W<TIMEOUTRrs> {
        TIDLE_W::new(self, 12)
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn timouten(&mut self) -> TIMOUTEN_W<TIMEOUTRrs> {
        TIMOUTEN_W::new(self, 15)
    }
    #[doc = "Bits 16:27 - Bus timeout B"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<TIMEOUTRrs> {
        TIMEOUTB_W::new(self, 16)
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn texten(&mut self) -> TEXTEN_W<TIMEOUTRrs> {
        TEXTEN_W::new(self, 31)
    }
}
#[doc = "Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeoutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeoutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUTRrs;
impl crate::RegisterSpec for TIMEOUTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeoutr::R`](R) reader structure"]
impl crate::Readable for TIMEOUTRrs {}
#[doc = "`write(|w| ..)` method takes [`timeoutr::W`](W) writer structure"]
impl crate::Writable for TIMEOUTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUTR to value 0"]
impl crate::Resettable for TIMEOUTRrs {
    const RESET_VALUE: u32 = 0;
}
