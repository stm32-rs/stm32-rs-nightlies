///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
/**ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYR {
    ///0: ADC is not ready to start conversion
    NotReady = 0,
    ///1: ADC is ready to start conversion
    Ready = 1,
}
impl From<ADRDYR> for bool {
    #[inline(always)]
    fn from(variant: ADRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` reader - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
pub type ADRDY_R = crate::BitReader<ADRDYR>;
impl ADRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADRDYR {
        match self.bits {
            false => ADRDYR::NotReady,
            true => ADRDYR::Ready,
        }
    }
    ///ADC is not ready to start conversion
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDYR::NotReady
    }
    ///ADC is ready to start conversion
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDYR::Ready
    }
}
/**ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYW {
    ///1: Clear ADC is ready to start conversion flag
    Clear = 1,
}
impl From<ADRDYW> for bool {
    #[inline(always)]
    fn from(variant: ADRDYW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` writer - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
pub type ADRDY_W<'a, REG> = crate::BitWriter1C<'a, REG, ADRDYW>;
impl<'a, REG> ADRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear ADC is ready to start conversion flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYW::Clear)
    }
}
/**End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPR {
    ///0: End of sampling phase no yet reached
    NotEnded = 0,
    ///1: End of sampling phase reached
    Ended = 1,
}
impl From<EOSMPR> for bool {
    #[inline(always)]
    fn from(variant: EOSMPR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` reader - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase.
pub type EOSMP_R = crate::BitReader<EOSMPR>;
impl EOSMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPR {
        match self.bits {
            false => EOSMPR::NotEnded,
            true => EOSMPR::Ended,
        }
    }
    ///End of sampling phase no yet reached
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMPR::NotEnded
    }
    ///End of sampling phase reached
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMPR::Ended
    }
}
/**End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPW {
    ///1: Clear end of sampling phase reached flag
    Clear = 1,
}
impl From<EOSMPW> for bool {
    #[inline(always)]
    fn from(variant: EOSMPW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` writer - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase.
pub type EOSMP_W<'a, REG> = crate::BitWriter1C<'a, REG, EOSMPW>;
impl<'a, REG> EOSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear end of sampling phase reached flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPW::Clear)
    }
}
/**End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR {
    ///0: Regular conversion is not complete
    NotComplete = 0,
    ///1: Regular conversion complete
    Complete = 1,
}
impl From<EOCR> for bool {
    #[inline(always)]
    fn from(variant: EOCR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` reader - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register
pub type EOC_R = crate::BitReader<EOCR>;
impl EOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCR {
        match self.bits {
            false => EOCR::NotComplete,
            true => EOCR::Complete,
        }
    }
    ///Regular conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR::NotComplete
    }
    ///Regular conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR::Complete
    }
}
/**End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW {
    ///1: Clear regular conversion complete flag
    Clear = 1,
}
impl From<EOCW> for bool {
    #[inline(always)]
    fn from(variant: EOCW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` writer - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register
pub type EOC_W<'a, REG> = crate::BitWriter1C<'a, REG, EOCW>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear regular conversion complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCW::Clear)
    }
}
/**End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR {
    ///0: Regular sequence is not complete
    NotComplete = 0,
    ///1: Regular sequence complete
    Complete = 1,
}
impl From<EOSR> for bool {
    #[inline(always)]
    fn from(variant: EOSR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` reader - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it.
pub type EOS_R = crate::BitReader<EOSR>;
impl EOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSR {
        match self.bits {
            false => EOSR::NotComplete,
            true => EOSR::Complete,
        }
    }
    ///Regular sequence is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOSR::NotComplete
    }
    ///Regular sequence complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOSR::Complete
    }
}
/**End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSW {
    ///1: Clear regular sequence complete flag
    Clear = 1,
}
impl From<EOSW> for bool {
    #[inline(always)]
    fn from(variant: EOSW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` writer - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it.
pub type EOS_W<'a, REG> = crate::BitWriter1C<'a, REG, EOSW>;
impl<'a, REG> EOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear regular sequence complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSW::Clear)
    }
}
/**ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVRR> for bool {
    #[inline(always)]
    fn from(variant: OVRR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it.
pub type OVR_R = crate::BitReader<OVRR>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRR {
        match self.bits {
            false => OVRR::NoOverrun,
            true => OVRR::Overrun,
        }
    }
    ///No overrun occurred
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NoOverrun
    }
    ///Overrun occurred
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::Overrun
    }
}
/**ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW {
    ///1: Clear overrun occurred flag
    Clear = 1,
}
impl From<OVRW> for bool {
    #[inline(always)]
    fn from(variant: OVRW) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` writer - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it.
pub type OVR_W<'a, REG> = crate::BitWriter1C<'a, REG, OVRW>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear overrun occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRW::Clear)
    }
}
/**Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCR {
    ///0: Injected conversion is not complete
    NotComplete = 0,
    ///1: Injected conversion complete
    Complete = 1,
}
impl From<JEOCR> for bool {
    #[inline(always)]
    fn from(variant: JEOCR) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC` reader - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register
pub type JEOC_R = crate::BitReader<JEOCR>;
impl JEOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOCR {
        match self.bits {
            false => JEOCR::NotComplete,
            true => JEOCR::Complete,
        }
    }
    ///Injected conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOCR::NotComplete
    }
    ///Injected conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOCR::Complete
    }
}
/**Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCW {
    ///1: Clear injected conversion complete flag
    Clear = 1,
}
impl From<JEOCW> for bool {
    #[inline(always)]
    fn from(variant: JEOCW) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC` writer - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register
pub type JEOC_W<'a, REG> = crate::BitWriter1C<'a, REG, JEOCW>;
impl<'a, REG> JEOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear injected conversion complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCW::Clear)
    }
}
/**Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSR {
    ///0: Injected sequence is not complete
    NotComplete = 0,
    ///1: Injected sequence complete
    Complete = 1,
}
impl From<JEOSR> for bool {
    #[inline(always)]
    fn from(variant: JEOSR) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOS` reader - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it.
pub type JEOS_R = crate::BitReader<JEOSR>;
impl JEOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOSR {
        match self.bits {
            false => JEOSR::NotComplete,
            true => JEOSR::Complete,
        }
    }
    ///Injected sequence is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOSR::NotComplete
    }
    ///Injected sequence complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOSR::Complete
    }
}
/**Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSW {
    ///1: Clear Injected sequence complete flag
    Clear = 1,
}
impl From<JEOSW> for bool {
    #[inline(always)]
    fn from(variant: JEOSW) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOS` writer - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it.
pub type JEOS_W<'a, REG> = crate::BitWriter1C<'a, REG, JEOSW>;
impl<'a, REG> JEOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear Injected sequence complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JEOSW::Clear)
    }
}
/**Analog watchdog %s flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1R {
    ///0: No analog watchdog event occurred
    NoEvent = 0,
    ///1: Analog watchdog event occurred
    Event = 1,
}
impl From<AWD1R> for bool {
    #[inline(always)]
    fn from(variant: AWD1R) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD(1-3)` reader - Analog watchdog %s flag
pub type AWD_R = crate::BitReader<AWD1R>;
impl AWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1R {
        match self.bits {
            false => AWD1R::NoEvent,
            true => AWD1R::Event,
        }
    }
    ///No analog watchdog event occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1R::NoEvent
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1R::Event
    }
}
/**Analog watchdog %s flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1W {
    ///1: Clear analog watchdog event occurred flag
    Clear = 1,
}
impl From<AWD1W> for bool {
    #[inline(always)]
    fn from(variant: AWD1W) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD(1-3)` writer - Analog watchdog %s flag
pub type AWD_W<'a, REG> = crate::BitWriter1C<'a, REG, AWD1W>;
impl<'a, REG> AWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear analog watchdog event occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1W::Clear)
    }
}
/**Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFR {
    ///0: No injected context queue overflow has occurred
    NoOverflow = 0,
    ///1: Injected context queue overflow has occurred
    Overflow = 1,
}
impl From<JQOVFR> for bool {
    #[inline(always)]
    fn from(variant: JQOVFR) -> Self {
        variant as u8 != 0
    }
}
///Field `JQOVF` reader - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information.
pub type JQOVF_R = crate::BitReader<JQOVFR>;
impl JQOVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JQOVFR {
        match self.bits {
            false => JQOVFR::NoOverflow,
            true => JQOVFR::Overflow,
        }
    }
    ///No injected context queue overflow has occurred
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVFR::NoOverflow
    }
    ///Injected context queue overflow has occurred
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVFR::Overflow
    }
}
/**Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFW {
    ///1: Clear injected context queue overflow flag
    Clear = 1,
}
impl From<JQOVFW> for bool {
    #[inline(always)]
    fn from(variant: JQOVFW) -> Self {
        variant as u8 != 0
    }
}
///Field `JQOVF` writer - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information.
pub type JQOVF_W<'a, REG> = crate::BitWriter1C<'a, REG, JQOVFW>;
impl<'a, REG> JQOVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear injected context queue overflow flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVFW::Clear)
    }
}
impl R {
    ///Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase.
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Analog watchdog (1-3) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1` field.</div>
    #[inline(always)]
    pub fn awd(&self, n: u8) -> AWD_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWD_R::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog watchdog (1-3) flag
    #[inline(always)]
    pub fn awd_iter(&self) -> impl Iterator<Item = AWD_R> + '_ {
        (0..3).map(move |n| AWD_R::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information.
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("adrdy", &self.adrdy())
            .field("eosmp", &self.eosmp())
            .field("eoc", &self.eoc())
            .field("eos", &self.eos())
            .field("ovr", &self.ovr())
            .field("jeoc", &self.jeoc())
            .field("jeos", &self.jeos())
            .field("awd1", &self.awd1())
            .field("awd2", &self.awd2())
            .field("awd3", &self.awd3())
            .field("jqovf", &self.jqovf())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W<ISRrs> {
        ADRDY_W::new(self, 0)
    }
    ///Bit 1 - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase.
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W<ISRrs> {
        EOSMP_W::new(self, 1)
    }
    ///Bit 2 - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<ISRrs> {
        EOC_W::new(self, 2)
    }
    ///Bit 3 - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W<ISRrs> {
        EOS_W::new(self, 3)
    }
    ///Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<ISRrs> {
        OVR_W::new(self, 4)
    }
    ///Bit 5 - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W<ISRrs> {
        JEOC_W::new(self, 5)
    }
    ///Bit 6 - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn jeos(&mut self) -> JEOS_W<ISRrs> {
        JEOS_W::new(self, 6)
    }
    ///Analog watchdog (1-3) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1` field.</div>
    #[inline(always)]
    pub fn awd(&mut self, n: u8) -> AWD_W<ISRrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWD_W::new(self, n + 7)
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD_W<ISRrs> {
        AWD_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD_W<ISRrs> {
        AWD_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD_W<ISRrs> {
        AWD_W::new(self, 9)
    }
    ///Bit 10 - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information.
    #[inline(always)]
    pub fn jqovf(&mut self) -> JQOVF_W<ISRrs> {
        JQOVF_W::new(self, 10)
    }
}
/**ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#ADC3:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07ff;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
