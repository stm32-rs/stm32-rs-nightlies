#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "ADC ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYR {
    #[doc = "0: ADC is not ready to start conversion"]
    NotReady = 0,
    #[doc = "1: ADC is ready to start conversion"]
    Ready = 1,
}
impl From<ADRDYR> for bool {
    #[inline(always)]
    fn from(variant: ADRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` reader - ADC ready"]
pub type ADRDY_R = crate::BitReader<ADRDYR>;
impl ADRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADRDYR {
        match self.bits {
            false => ADRDYR::NotReady,
            true => ADRDYR::Ready,
        }
    }
    #[doc = "ADC is not ready to start conversion"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDYR::NotReady
    }
    #[doc = "ADC is ready to start conversion"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDYR::Ready
    }
}
#[doc = "ADC ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYW {
    #[doc = "1: Clear ADC is ready to start conversion flag"]
    Clear = 1,
}
impl From<ADRDYW> for bool {
    #[inline(always)]
    fn from(variant: ADRDYW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` writer - ADC ready"]
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG, ADRDYW>;
impl<'a, REG> ADRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear ADC is ready to start conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYW::Clear)
    }
}
#[doc = "End of sampling flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPR {
    #[doc = "0: End of sampling phase no yet reached"]
    NotEnded = 0,
    #[doc = "1: End of sampling phase reached"]
    Ended = 1,
}
impl From<EOSMPR> for bool {
    #[inline(always)]
    fn from(variant: EOSMPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` reader - End of sampling flag"]
pub type EOSMP_R = crate::BitReader<EOSMPR>;
impl EOSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPR {
        match self.bits {
            false => EOSMPR::NotEnded,
            true => EOSMPR::Ended,
        }
    }
    #[doc = "End of sampling phase no yet reached"]
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMPR::NotEnded
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMPR::Ended
    }
}
#[doc = "End of sampling flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPW {
    #[doc = "1: Clear end of sampling phase reached flag"]
    Clear = 1,
}
impl From<EOSMPW> for bool {
    #[inline(always)]
    fn from(variant: EOSMPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` writer - End of sampling flag"]
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPW>;
impl<'a, REG> EOSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear end of sampling phase reached flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPW::Clear)
    }
}
#[doc = "End of conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR {
    #[doc = "0: Regular conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Regular conversion complete"]
    Complete = 1,
}
impl From<EOCR> for bool {
    #[inline(always)]
    fn from(variant: EOCR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - End of conversion flag"]
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
    #[doc = "Regular conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR::NotComplete
    }
    #[doc = "Regular conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR::Complete
    }
}
#[doc = "End of conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW {
    #[doc = "1: Clear regular conversion complete flag"]
    Clear = 1,
}
impl From<EOCW> for bool {
    #[inline(always)]
    fn from(variant: EOCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - End of conversion flag"]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG, EOCW>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear regular conversion complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCW::Clear)
    }
}
#[doc = "End of regular sequence flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR {
    #[doc = "0: Regular sequence is not complete"]
    NotComplete = 0,
    #[doc = "1: Regular sequence complete"]
    Complete = 1,
}
impl From<EOSR> for bool {
    #[inline(always)]
    fn from(variant: EOSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` reader - End of regular sequence flag"]
pub type EOS_R = crate::BitReader<EOSR>;
impl EOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSR {
        match self.bits {
            false => EOSR::NotComplete,
            true => EOSR::Complete,
        }
    }
    #[doc = "Regular sequence is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOSR::NotComplete
    }
    #[doc = "Regular sequence complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOSR::Complete
    }
}
#[doc = "End of regular sequence flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSW {
    #[doc = "1: Clear regular sequence complete flag"]
    Clear = 1,
}
impl From<EOSW> for bool {
    #[inline(always)]
    fn from(variant: EOSW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` writer - End of regular sequence flag"]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG, EOSW>;
impl<'a, REG> EOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear regular sequence complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSW::Clear)
    }
}
#[doc = "ADC overrun\n\nValue on reset: 0"]
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
#[doc = "Field `OVR` reader - ADC overrun"]
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
#[doc = "ADC overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW {
    #[doc = "1: Clear overrun occurred flag"]
    Clear = 1,
}
impl From<OVRW> for bool {
    #[inline(always)]
    fn from(variant: OVRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` writer - ADC overrun"]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG, OVRW>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear overrun occurred flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRW::Clear)
    }
}
#[doc = "Injected channel end of conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCR {
    #[doc = "0: Injected conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Injected conversion complete"]
    Complete = 1,
}
impl From<JEOCR> for bool {
    #[inline(always)]
    fn from(variant: JEOCR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC` reader - Injected channel end of conversion flag"]
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
    #[doc = "Injected conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOCR::NotComplete
    }
    #[doc = "Injected conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOCR::Complete
    }
}
#[doc = "Injected channel end of conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCW {
    #[doc = "1: Clear injected conversion complete flag"]
    Clear = 1,
}
impl From<JEOCW> for bool {
    #[inline(always)]
    fn from(variant: JEOCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC` writer - Injected channel end of conversion flag"]
pub type JEOC_W<'a, REG> = crate::BitWriter<'a, REG, JEOCW>;
impl<'a, REG> JEOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear injected conversion complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCW::Clear)
    }
}
#[doc = "Injected channel end of sequence flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSR {
    #[doc = "0: Injected sequence is not complete"]
    NotComplete = 0,
    #[doc = "1: Injected sequence complete"]
    Complete = 1,
}
impl From<JEOSR> for bool {
    #[inline(always)]
    fn from(variant: JEOSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOS` reader - Injected channel end of sequence flag"]
pub type JEOS_R = crate::BitReader<JEOSR>;
impl JEOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOSR {
        match self.bits {
            false => JEOSR::NotComplete,
            true => JEOSR::Complete,
        }
    }
    #[doc = "Injected sequence is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOSR::NotComplete
    }
    #[doc = "Injected sequence complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOSR::Complete
    }
}
#[doc = "Injected channel end of sequence flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSW {
    #[doc = "1: Clear Injected sequence complete flag"]
    Clear = 1,
}
impl From<JEOSW> for bool {
    #[inline(always)]
    fn from(variant: JEOSW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOS` writer - Injected channel end of sequence flag"]
pub type JEOS_W<'a, REG> = crate::BitWriter<'a, REG, JEOSW>;
impl<'a, REG> JEOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Injected sequence complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JEOSW::Clear)
    }
}
#[doc = "Analog watchdog 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1R {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<AWD1R> for bool {
    #[inline(always)]
    fn from(variant: AWD1R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` reader - Analog watchdog 1 flag"]
pub type AWD1_R = crate::BitReader<AWD1R>;
impl AWD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1R {
        match self.bits {
            false => AWD1R::NoEvent,
            true => AWD1R::Event,
        }
    }
    #[doc = "No analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1R::NoEvent
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1R::Event
    }
}
#[doc = "Analog watchdog 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1W {
    #[doc = "1: Clear analog watchdog event occurred flag"]
    Clear = 1,
}
impl From<AWD1W> for bool {
    #[inline(always)]
    fn from(variant: AWD1W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` writer - Analog watchdog 1 flag"]
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG, AWD1W>;
impl<'a, REG> AWD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear analog watchdog event occurred flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1W::Clear)
    }
}
#[doc = "Field `AWD2` reader - Analog watchdog 2 flag"]
pub use AWD1_R as AWD2_R;
#[doc = "Field `AWD3` reader - Analog watchdog 3 flag"]
pub use AWD1_R as AWD3_R;
#[doc = "Field `AWD2` writer - Analog watchdog 2 flag"]
pub use AWD1_W as AWD2_W;
#[doc = "Field `AWD3` writer - Analog watchdog 3 flag"]
pub use AWD1_W as AWD3_W;
#[doc = "Injected context queue overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFR {
    #[doc = "0: No injected context queue overflow has occurred"]
    NoOverflow = 0,
    #[doc = "1: Injected context queue overflow has occurred"]
    Overflow = 1,
}
impl From<JQOVFR> for bool {
    #[inline(always)]
    fn from(variant: JQOVFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQOVF` reader - Injected context queue overflow"]
pub type JQOVF_R = crate::BitReader<JQOVFR>;
impl JQOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JQOVFR {
        match self.bits {
            false => JQOVFR::NoOverflow,
            true => JQOVFR::Overflow,
        }
    }
    #[doc = "No injected context queue overflow has occurred"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVFR::NoOverflow
    }
    #[doc = "Injected context queue overflow has occurred"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVFR::Overflow
    }
}
#[doc = "Injected context queue overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFW {
    #[doc = "1: Clear injected context queue overflow flag"]
    Clear = 1,
}
impl From<JQOVFW> for bool {
    #[inline(always)]
    fn from(variant: JQOVFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQOVF` writer - Injected context queue overflow"]
pub type JQOVF_W<'a, REG> = crate::BitWriter<'a, REG, JQOVFW>;
impl<'a, REG> JQOVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear injected context queue overflow flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVFW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready"]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag"]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence flag"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Injected channel end of conversion flag"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Injected channel end of sequence flag"]
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected context queue overflow"]
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready"]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<ISRrs> {
        ADRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of sampling flag"]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<ISRrs> {
        EOSMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<ISRrs> {
        EOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of regular sequence flag"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<ISRrs> {
        EOS_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<ISRrs> {
        OVR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Injected channel end of conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JEOC_W<ISRrs> {
        JEOC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Injected channel end of sequence flag"]
    #[inline(always)]
    #[must_use]
    pub fn jeos(&mut self) -> JEOS_W<ISRrs> {
        JEOS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag"]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<ISRrs> {
        AWD1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag"]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<ISRrs> {
        AWD2_W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag"]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<ISRrs> {
        AWD3_W::new(self, 9)
    }
    #[doc = "Bit 10 - Injected context queue overflow"]
    #[inline(always)]
    #[must_use]
    pub fn jqovf(&mut self) -> JQOVF_W<ISRrs> {
        JQOVF_W::new(self, 10)
    }
}
#[doc = "ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
