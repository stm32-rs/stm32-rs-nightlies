#[doc = "Register `TIMEOUTR` reader"]
pub type R = crate::R<TIMEOUTRrs>;
#[doc = "Register `TIMEOUTR` writer"]
pub type W = crate::W<TIMEOUTRrs>;
#[doc = "Field `TIMEOUTA` reader - TIMEOUTA"]
pub type TIMEOUTA_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTA` writer - TIMEOUTA"]
pub type TIMEOUTA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "TIDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIDLE {
    #[doc = "0: TIMEOUTA is used to detect SCL low timeout"]
    Disabled = 0,
    #[doc = "1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    Enabled = 1,
}
impl From<TIDLE> for bool {
    #[inline(always)]
    fn from(variant: TIDLE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIDLE` reader - TIDLE"]
pub type TIDLE_R = crate::BitReader<TIDLE>;
impl TIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIDLE {
        match self.bits {
            false => TIDLE::Disabled,
            true => TIDLE::Enabled,
        }
    }
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIDLE::Disabled
    }
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIDLE::Enabled
    }
}
#[doc = "Field `TIDLE` writer - TIDLE"]
pub type TIDLE_W<'a, REG> = crate::BitWriter<'a, REG, TIDLE>;
impl<'a, REG> TIDLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIDLE::Disabled)
    }
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIDLE::Enabled)
    }
}
#[doc = "TIMOUTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTEN {
    #[doc = "0: SCL timeout detection is disabled"]
    Disabled = 0,
    #[doc = "1: SCL timeout detection is enabled"]
    Enabled = 1,
}
impl From<TIMOUTEN> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUTEN` reader - TIMOUTEN"]
pub type TIMOUTEN_R = crate::BitReader<TIMOUTEN>;
impl TIMOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMOUTEN {
        match self.bits {
            false => TIMOUTEN::Disabled,
            true => TIMOUTEN::Enabled,
        }
    }
    #[doc = "SCL timeout detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMOUTEN::Disabled
    }
    #[doc = "SCL timeout detection is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMOUTEN::Enabled
    }
}
#[doc = "Field `TIMOUTEN` writer - TIMOUTEN"]
pub type TIMOUTEN_W<'a, REG> = crate::BitWriter<'a, REG, TIMOUTEN>;
impl<'a, REG> TIMOUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCL timeout detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUTEN::Disabled)
    }
    #[doc = "SCL timeout detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUTEN::Enabled)
    }
}
#[doc = "Field `TIMEOUTB` reader - TIMEOUTB"]
pub type TIMEOUTB_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTB` writer - TIMEOUTB"]
pub type TIMEOUTB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "TEXTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTEN {
    #[doc = "0: Extended clock timeout detection is disabled"]
    Disabled = 0,
    #[doc = "1: Extended clock timeout detection is enabled"]
    Enabled = 1,
}
impl From<TEXTEN> for bool {
    #[inline(always)]
    fn from(variant: TEXTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXTEN` reader - TEXTEN"]
pub type TEXTEN_R = crate::BitReader<TEXTEN>;
impl TEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEXTEN {
        match self.bits {
            false => TEXTEN::Disabled,
            true => TEXTEN::Enabled,
        }
    }
    #[doc = "Extended clock timeout detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEXTEN::Disabled
    }
    #[doc = "Extended clock timeout detection is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEXTEN::Enabled
    }
}
#[doc = "Field `TEXTEN` writer - TEXTEN"]
pub type TEXTEN_W<'a, REG> = crate::BitWriter<'a, REG, TEXTEN>;
impl<'a, REG> TEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Extended clock timeout detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTEN::Disabled)
    }
    #[doc = "Extended clock timeout detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTEN::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:11 - TIMEOUTA"]
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - TIDLE"]
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - TIMOUTEN"]
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - TIMEOUTB"]
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - TEXTEN"]
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - TIMEOUTA"]
    #[inline(always)]
    #[must_use]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<TIMEOUTRrs> {
        TIMEOUTA_W::new(self, 0)
    }
    #[doc = "Bit 12 - TIDLE"]
    #[inline(always)]
    #[must_use]
    pub fn tidle(&mut self) -> TIDLE_W<TIMEOUTRrs> {
        TIDLE_W::new(self, 12)
    }
    #[doc = "Bit 15 - TIMOUTEN"]
    #[inline(always)]
    #[must_use]
    pub fn timouten(&mut self) -> TIMOUTEN_W<TIMEOUTRrs> {
        TIMOUTEN_W::new(self, 15)
    }
    #[doc = "Bits 16:27 - TIMEOUTB"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<TIMEOUTRrs> {
        TIMEOUTB_W::new(self, 16)
    }
    #[doc = "Bit 31 - TEXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn texten(&mut self) -> TEXTEN_W<TIMEOUTRrs> {
        TEXTEN_W::new(self, 31)
    }
}
#[doc = "Timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeoutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeoutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
