#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "ADRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYR {
    #[doc = "0: ADC not yet ready to start conversion"]
    NotReady = 0,
    #[doc = "1: ADC ready to start conversion"]
    Ready = 1,
}
impl From<ADRDYR> for bool {
    #[inline(always)]
    fn from(variant: ADRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` reader - ADRDY"]
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
    #[doc = "ADC not yet ready to start conversion"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDYR::NotReady
    }
    #[doc = "ADC ready to start conversion"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDYR::Ready
    }
}
#[doc = "ADRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYW {
    #[doc = "1: Clear the ADC ready flag"]
    Clear = 1,
}
impl From<ADRDYW> for bool {
    #[inline(always)]
    fn from(variant: ADRDYW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` writer - ADRDY"]
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG, ADRDYW>;
impl<'a, REG> ADRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the ADC ready flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYW::Clear)
    }
}
#[doc = "EOSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPR {
    #[doc = "0: Not at the end of the samplings phase"]
    NotAtEnd = 0,
    #[doc = "1: End of sampling phase reached"]
    AtEnd = 1,
}
impl From<EOSMPR> for bool {
    #[inline(always)]
    fn from(variant: EOSMPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` reader - EOSMP"]
pub type EOSMP_R = crate::BitReader<EOSMPR>;
impl EOSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPR {
        match self.bits {
            false => EOSMPR::NotAtEnd,
            true => EOSMPR::AtEnd,
        }
    }
    #[doc = "Not at the end of the samplings phase"]
    #[inline(always)]
    pub fn is_not_at_end(&self) -> bool {
        *self == EOSMPR::NotAtEnd
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        *self == EOSMPR::AtEnd
    }
}
#[doc = "EOSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPW {
    #[doc = "1: Clear the sampling phase flag"]
    Clear = 1,
}
impl From<EOSMPW> for bool {
    #[inline(always)]
    fn from(variant: EOSMPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` writer - EOSMP"]
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPW>;
impl<'a, REG> EOSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the sampling phase flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPW::Clear)
    }
}
#[doc = "EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR {
    #[doc = "0: Channel conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Channel conversion complete"]
    Complete = 1,
}
impl From<EOCR> for bool {
    #[inline(always)]
    fn from(variant: EOCR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - EOC"]
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
    #[doc = "Channel conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR::NotComplete
    }
    #[doc = "Channel conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR::Complete
    }
}
#[doc = "EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW {
    #[doc = "1: Clear the channel conversion flag"]
    Clear = 1,
}
impl From<EOCW> for bool {
    #[inline(always)]
    fn from(variant: EOCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - EOC"]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG, EOCW>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the channel conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCW::Clear)
    }
}
#[doc = "EOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR {
    #[doc = "0: Conversion sequence is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion sequence complete"]
    Complete = 1,
}
impl From<EOSR> for bool {
    #[inline(always)]
    fn from(variant: EOSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` reader - EOS"]
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
    #[doc = "Conversion sequence is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOSR::NotComplete
    }
    #[doc = "Conversion sequence complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOSR::Complete
    }
}
#[doc = "EOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSW {
    #[doc = "1: Clear the conversion sequence flag"]
    Clear = 1,
}
impl From<EOSW> for bool {
    #[inline(always)]
    fn from(variant: EOSW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` writer - EOS"]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG, EOSW>;
impl<'a, REG> EOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the conversion sequence flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSW::Clear)
    }
}
#[doc = "OVR\n\nValue on reset: 0"]
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
#[doc = "Field `OVR` reader - OVR"]
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
#[doc = "OVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW {
    #[doc = "1: Clear the overrun flag"]
    Clear = 1,
}
impl From<OVRW> for bool {
    #[inline(always)]
    fn from(variant: OVRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` writer - OVR"]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG, OVRW>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the overrun flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRW::Clear)
    }
}
#[doc = "AWD1\n\nValue on reset: 0"]
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
#[doc = "Field `AWD1` reader - AWD1"]
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
#[doc = "AWD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1W {
    #[doc = "1: Clear the analog watchdog event flag"]
    Clear = 1,
}
impl From<AWD1W> for bool {
    #[inline(always)]
    fn from(variant: AWD1W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` writer - AWD1"]
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG, AWD1W>;
impl<'a, REG> AWD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the analog watchdog event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1W::Clear)
    }
}
#[doc = "Field `AWD2` reader - AWD2"]
pub use AWD1_R as AWD2_R;
#[doc = "Field `AWD3` reader - AWD3"]
pub use AWD1_R as AWD3_R;
#[doc = "Field `AWD2` writer - AWD2"]
pub use AWD1_W as AWD2_W;
#[doc = "Field `AWD3` writer - AWD3"]
pub use AWD1_W as AWD3_W;
#[doc = "EOCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALR {
    #[doc = "0: Calibration is not complete"]
    NotComplete = 0,
    #[doc = "1: Calibration complete"]
    Complete = 1,
}
impl From<EOCALR> for bool {
    #[inline(always)]
    fn from(variant: EOCALR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCAL` reader - EOCAL"]
pub type EOCAL_R = crate::BitReader<EOCALR>;
impl EOCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOCALR {
        match self.bits {
            false => EOCALR::NotComplete,
            true => EOCALR::Complete,
        }
    }
    #[doc = "Calibration is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCALR::NotComplete
    }
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCALR::Complete
    }
}
#[doc = "EOCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALW {
    #[doc = "1: Clear the calibration flag"]
    Clear = 1,
}
impl From<EOCALW> for bool {
    #[inline(always)]
    fn from(variant: EOCALW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCAL` writer - EOCAL"]
pub type EOCAL_W<'a, REG> = crate::BitWriter<'a, REG, EOCALW>;
impl<'a, REG> EOCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the calibration flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCALW::Clear)
    }
}
#[doc = "CCRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRDYR {
    #[doc = "0: Channel configuration update not applied"]
    NotComplete = 0,
    #[doc = "1: Channel configuration update is applied"]
    Complete = 1,
}
impl From<CCRDYR> for bool {
    #[inline(always)]
    fn from(variant: CCRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRDY` reader - CCRDY"]
pub type CCRDY_R = crate::BitReader<CCRDYR>;
impl CCRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCRDYR {
        match self.bits {
            false => CCRDYR::NotComplete,
            true => CCRDYR::Complete,
        }
    }
    #[doc = "Channel configuration update not applied"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CCRDYR::NotComplete
    }
    #[doc = "Channel configuration update is applied"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCRDYR::Complete
    }
}
#[doc = "CCRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRDYW {
    #[doc = "1: Clear the channel configuration flag"]
    Clear = 1,
}
impl From<CCRDYW> for bool {
    #[inline(always)]
    fn from(variant: CCRDYW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRDY` writer - CCRDY"]
pub type CCRDY_W<'a, REG> = crate::BitWriter<'a, REG, CCRDYW>;
impl<'a, REG> CCRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the channel configuration flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCRDYW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - EOCAL"]
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - CCRDY"]
    #[inline(always)]
    pub fn ccrdy(&self) -> CCRDY_R {
        CCRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<ISRrs> {
        ADRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<ISRrs> {
        EOSMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<ISRrs> {
        EOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<ISRrs> {
        EOS_W::new(self, 3)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<ISRrs> {
        OVR_W::new(self, 4)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<ISRrs> {
        AWD1_W::new(self, 7)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<ISRrs> {
        AWD2_W::new(self, 8)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<ISRrs> {
        AWD3_W::new(self, 9)
    }
    #[doc = "Bit 11 - EOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EOCAL_W<ISRrs> {
        EOCAL_W::new(self, 11)
    }
    #[doc = "Bit 13 - CCRDY"]
    #[inline(always)]
    #[must_use]
    pub fn ccrdy(&mut self) -> CCRDY_W<ISRrs> {
        CCRDY_W::new(self, 13)
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
