#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Analog watchdog flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDR {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<AWDR> for bool {
    #[inline(always)]
    fn from(variant: AWDR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD` reader - Analog watchdog flag"]
pub type AWD_R = crate::BitReader<AWDR>;
impl AWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWDR {
        match self.bits {
            false => AWDR::NoEvent,
            true => AWDR::Event,
        }
    }
    #[doc = "No analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWDR::NoEvent
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWDR::Event
    }
}
#[doc = "Analog watchdog flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<AWDW> for bool {
    #[inline(always)]
    fn from(variant: AWDW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD` writer - Analog watchdog flag"]
pub type AWD_W<'a, REG> = crate::BitWriter0C<'a, REG, AWDW>;
impl<'a, REG> AWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AWDW::Clear)
    }
}
#[doc = "Regular channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<EOCR> for bool {
    #[inline(always)]
    fn from(variant: EOCR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - Regular channel end of conversion"]
pub type EOC_R = crate::BitReader<EOCR>;
impl EOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOCR {
        match self.bits {
            false => EOCR::NotComplete,
            true => EOCR::Complete,
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR::NotComplete
    }
    #[doc = "Conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR::Complete
    }
}
#[doc = "Regular channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<EOCW> for bool {
    #[inline(always)]
    fn from(variant: EOCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - Regular channel end of conversion"]
pub type EOC_W<'a, REG> = crate::BitWriter0C<'a, REG, EOCW>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCW::Clear)
    }
}
#[doc = "Injected channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCR {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<JEOCR> for bool {
    #[inline(always)]
    fn from(variant: JEOCR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC` reader - Injected channel end of conversion"]
pub type JEOC_R = crate::BitReader<JEOCR>;
impl JEOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOCR {
        match self.bits {
            false => JEOCR::NotComplete,
            true => JEOCR::Complete,
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOCR::NotComplete
    }
    #[doc = "Conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOCR::Complete
    }
}
#[doc = "Injected channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<JEOCW> for bool {
    #[inline(always)]
    fn from(variant: JEOCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC` writer - Injected channel end of conversion"]
pub type JEOC_W<'a, REG> = crate::BitWriter0C<'a, REG, JEOCW>;
impl<'a, REG> JEOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCW::Clear)
    }
}
#[doc = "Injected channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRTR {
    #[doc = "0: No injected channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Injected channel conversion has started"]
    Started = 1,
}
impl From<JSTRTR> for bool {
    #[inline(always)]
    fn from(variant: JSTRTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSTRT` reader - Injected channel start flag"]
pub type JSTRT_R = crate::BitReader<JSTRTR>;
impl JSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JSTRTR {
        match self.bits {
            false => JSTRTR::NotStarted,
            true => JSTRTR::Started,
        }
    }
    #[doc = "No injected channel conversion started"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRTR::NotStarted
    }
    #[doc = "Injected channel conversion has started"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRTR::Started
    }
}
#[doc = "Injected channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRTW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<JSTRTW> for bool {
    #[inline(always)]
    fn from(variant: JSTRTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSTRT` writer - Injected channel start flag"]
pub type JSTRT_W<'a, REG> = crate::BitWriter0C<'a, REG, JSTRTW>;
impl<'a, REG> JSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JSTRTW::Clear)
    }
}
#[doc = "Regular channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTR {
    #[doc = "0: No regular channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Regular channel conversion has started"]
    Started = 1,
}
impl From<STRTR> for bool {
    #[inline(always)]
    fn from(variant: STRTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` reader - Regular channel start flag"]
pub type STRT_R = crate::BitReader<STRTR>;
impl STRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STRTR {
        match self.bits {
            false => STRTR::NotStarted,
            true => STRTR::Started,
        }
    }
    #[doc = "No regular channel conversion started"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRTR::NotStarted
    }
    #[doc = "Regular channel conversion has started"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRTR::Started
    }
}
#[doc = "Regular channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<STRTW> for bool {
    #[inline(always)]
    fn from(variant: STRTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` writer - Regular channel start flag"]
pub type STRT_W<'a, REG> = crate::BitWriter0C<'a, REG, STRTW>;
impl<'a, REG> STRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(STRTW::Clear)
    }
}
#[doc = "Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVRR> for bool {
    #[inline(always)]
    fn from(variant: OVRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - Overrun"]
pub type OVR_R = crate::BitReader<OVRR>;
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRR {
        match self.bits {
            false => OVRR::NoOverrun,
            true => OVRR::Overrun,
        }
    }
    #[doc = "No overrun occurred"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NoOverrun
    }
    #[doc = "Overrun occurred"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::Overrun
    }
}
#[doc = "Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<OVRW> for bool {
    #[inline(always)]
    fn from(variant: OVRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` writer - Overrun"]
pub type OVR_W<'a, REG> = crate::BitWriter0C<'a, REG, OVRW>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    #[must_use]
    pub fn awd(&mut self) -> AWD_W<SRrs> {
        AWD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<SRrs> {
        EOC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JEOC_W<SRrs> {
        JEOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    #[must_use]
    pub fn jstrt(&mut self) -> JSTRT_W<SRrs> {
        JSTRT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<SRrs> {
        STRT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<SRrs> {
        OVR_W::new(self, 5)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
