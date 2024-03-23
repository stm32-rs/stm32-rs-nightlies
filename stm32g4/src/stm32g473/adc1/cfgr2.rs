#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Regular Oversampling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE {
    #[doc = "0: Regular oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Regular oversampling enabled"]
    Enabled = 1,
}
impl From<ROVSE> for bool {
    #[inline(always)]
    fn from(variant: ROVSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVSE` reader - Regular Oversampling Enable"]
pub type ROVSE_R = crate::BitReader<ROVSE>;
impl ROVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROVSE {
        match self.bits {
            false => ROVSE::Disabled,
            true => ROVSE::Enabled,
        }
    }
    #[doc = "Regular oversampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE::Disabled
    }
    #[doc = "Regular oversampling enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE::Enabled
    }
}
#[doc = "Field `ROVSE` writer - Regular Oversampling Enable"]
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG, ROVSE>;
impl<'a, REG> ROVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Disabled)
    }
    #[doc = "Regular oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Enabled)
    }
}
#[doc = "Injected Oversampling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE {
    #[doc = "0: Injected oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Injected oversampling enabled"]
    Enabled = 1,
}
impl From<JOVSE> for bool {
    #[inline(always)]
    fn from(variant: JOVSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JOVSE` reader - Injected Oversampling Enable"]
pub type JOVSE_R = crate::BitReader<JOVSE>;
impl JOVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JOVSE {
        match self.bits {
            false => JOVSE::Disabled,
            true => JOVSE::Enabled,
        }
    }
    #[doc = "Injected oversampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE::Disabled
    }
    #[doc = "Injected oversampling enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE::Enabled
    }
}
#[doc = "Field `JOVSE` writer - Injected Oversampling Enable"]
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG, JOVSE>;
impl<'a, REG> JOVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Disabled)
    }
    #[doc = "Injected oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Enabled)
    }
}
#[doc = "Oversampling ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR {
    #[doc = "0: Oversampling ratio of 2"]
    Os2 = 0,
    #[doc = "1: Oversampling ratio of 4"]
    Os4 = 1,
    #[doc = "2: Oversampling ratio of 8"]
    Os8 = 2,
    #[doc = "3: Oversampling ratio of 16"]
    Os16 = 3,
    #[doc = "4: Oversampling ratio of 32"]
    Os32 = 4,
    #[doc = "5: Oversampling ratio of 64"]
    Os64 = 5,
    #[doc = "6: Oversampling ratio of 128"]
    Os128 = 6,
    #[doc = "7: Oversampling ratio of 256"]
    Os256 = 7,
}
impl From<OVSR> for u8 {
    #[inline(always)]
    fn from(variant: OVSR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSR {
    type Ux = u8;
}
#[doc = "Field `OVSR` reader - Oversampling ratio"]
pub type OVSR_R = crate::FieldReader<OVSR>;
impl OVSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVSR {
        match self.bits {
            0 => OVSR::Os2,
            1 => OVSR::Os4,
            2 => OVSR::Os8,
            3 => OVSR::Os16,
            4 => OVSR::Os32,
            5 => OVSR::Os64,
            6 => OVSR::Os128,
            7 => OVSR::Os256,
            _ => unreachable!(),
        }
    }
    #[doc = "Oversampling ratio of 2"]
    #[inline(always)]
    pub fn is_os2(&self) -> bool {
        *self == OVSR::Os2
    }
    #[doc = "Oversampling ratio of 4"]
    #[inline(always)]
    pub fn is_os4(&self) -> bool {
        *self == OVSR::Os4
    }
    #[doc = "Oversampling ratio of 8"]
    #[inline(always)]
    pub fn is_os8(&self) -> bool {
        *self == OVSR::Os8
    }
    #[doc = "Oversampling ratio of 16"]
    #[inline(always)]
    pub fn is_os16(&self) -> bool {
        *self == OVSR::Os16
    }
    #[doc = "Oversampling ratio of 32"]
    #[inline(always)]
    pub fn is_os32(&self) -> bool {
        *self == OVSR::Os32
    }
    #[doc = "Oversampling ratio of 64"]
    #[inline(always)]
    pub fn is_os64(&self) -> bool {
        *self == OVSR::Os64
    }
    #[doc = "Oversampling ratio of 128"]
    #[inline(always)]
    pub fn is_os128(&self) -> bool {
        *self == OVSR::Os128
    }
    #[doc = "Oversampling ratio of 256"]
    #[inline(always)]
    pub fn is_os256(&self) -> bool {
        *self == OVSR::Os256
    }
}
#[doc = "Field `OVSR` writer - Oversampling ratio"]
pub type OVSR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OVSR>;
impl<'a, REG> OVSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Oversampling ratio of 2"]
    #[inline(always)]
    pub fn os2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os2)
    }
    #[doc = "Oversampling ratio of 4"]
    #[inline(always)]
    pub fn os4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os4)
    }
    #[doc = "Oversampling ratio of 8"]
    #[inline(always)]
    pub fn os8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os8)
    }
    #[doc = "Oversampling ratio of 16"]
    #[inline(always)]
    pub fn os16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os16)
    }
    #[doc = "Oversampling ratio of 32"]
    #[inline(always)]
    pub fn os32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os32)
    }
    #[doc = "Oversampling ratio of 64"]
    #[inline(always)]
    pub fn os64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os64)
    }
    #[doc = "Oversampling ratio of 128"]
    #[inline(always)]
    pub fn os128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os128)
    }
    #[doc = "Oversampling ratio of 256"]
    #[inline(always)]
    pub fn os256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os256)
    }
}
#[doc = "Oversampling shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS {
    #[doc = "0: No right shift applied to oversampling result"]
    NoShift = 0,
    #[doc = "1: Shift oversampling result right by 1 bit"]
    Shift1 = 1,
    #[doc = "2: Shift oversampling result right by 2 bits"]
    Shift2 = 2,
    #[doc = "3: Shift oversampling result right by 3 bits"]
    Shift3 = 3,
    #[doc = "4: Shift oversampling result right by 4 bits"]
    Shift4 = 4,
    #[doc = "5: Shift oversampling result right by 5 bits"]
    Shift5 = 5,
    #[doc = "6: Shift oversampling result right by 6 bits"]
    Shift6 = 6,
    #[doc = "7: Shift oversampling result right by 7 bits"]
    Shift7 = 7,
    #[doc = "8: Shift oversampling result right by 8 bits"]
    Shift8 = 8,
}
impl From<OVSS> for u8 {
    #[inline(always)]
    fn from(variant: OVSS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSS {
    type Ux = u8;
}
#[doc = "Field `OVSS` reader - Oversampling shift"]
pub type OVSS_R = crate::FieldReader<OVSS>;
impl OVSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVSS> {
        match self.bits {
            0 => Some(OVSS::NoShift),
            1 => Some(OVSS::Shift1),
            2 => Some(OVSS::Shift2),
            3 => Some(OVSS::Shift3),
            4 => Some(OVSS::Shift4),
            5 => Some(OVSS::Shift5),
            6 => Some(OVSS::Shift6),
            7 => Some(OVSS::Shift7),
            8 => Some(OVSS::Shift8),
            _ => None,
        }
    }
    #[doc = "No right shift applied to oversampling result"]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS::NoShift
    }
    #[doc = "Shift oversampling result right by 1 bit"]
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        *self == OVSS::Shift1
    }
    #[doc = "Shift oversampling result right by 2 bits"]
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        *self == OVSS::Shift2
    }
    #[doc = "Shift oversampling result right by 3 bits"]
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        *self == OVSS::Shift3
    }
    #[doc = "Shift oversampling result right by 4 bits"]
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        *self == OVSS::Shift4
    }
    #[doc = "Shift oversampling result right by 5 bits"]
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        *self == OVSS::Shift5
    }
    #[doc = "Shift oversampling result right by 6 bits"]
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        *self == OVSS::Shift6
    }
    #[doc = "Shift oversampling result right by 7 bits"]
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        *self == OVSS::Shift7
    }
    #[doc = "Shift oversampling result right by 8 bits"]
    #[inline(always)]
    pub fn is_shift8(&self) -> bool {
        *self == OVSS::Shift8
    }
}
#[doc = "Field `OVSS` writer - Oversampling shift"]
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSS>;
impl<'a, REG> OVSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No right shift applied to oversampling result"]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::NoShift)
    }
    #[doc = "Shift oversampling result right by 1 bit"]
    #[inline(always)]
    pub fn shift1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift1)
    }
    #[doc = "Shift oversampling result right by 2 bits"]
    #[inline(always)]
    pub fn shift2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift2)
    }
    #[doc = "Shift oversampling result right by 3 bits"]
    #[inline(always)]
    pub fn shift3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift3)
    }
    #[doc = "Shift oversampling result right by 4 bits"]
    #[inline(always)]
    pub fn shift4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift4)
    }
    #[doc = "Shift oversampling result right by 5 bits"]
    #[inline(always)]
    pub fn shift5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift5)
    }
    #[doc = "Shift oversampling result right by 6 bits"]
    #[inline(always)]
    pub fn shift6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift6)
    }
    #[doc = "Shift oversampling result right by 7 bits"]
    #[inline(always)]
    pub fn shift7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift7)
    }
    #[doc = "Shift oversampling result right by 8 bits"]
    #[inline(always)]
    pub fn shift8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift8)
    }
}
#[doc = "Triggered Regular Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS {
    #[doc = "0: All oversampled conversions for a channel are run following a trigger"]
    Automatic = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a new trigger"]
    Triggered = 1,
}
impl From<TROVS> for bool {
    #[inline(always)]
    fn from(variant: TROVS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TROVS` reader - Triggered Regular Oversampling"]
pub type TROVS_R = crate::BitReader<TROVS>;
impl TROVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TROVS {
        match self.bits {
            false => TROVS::Automatic,
            true => TROVS::Triggered,
        }
    }
    #[doc = "All oversampled conversions for a channel are run following a trigger"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TROVS::Automatic
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TROVS::Triggered
    }
}
#[doc = "Field `TROVS` writer - Triggered Regular Oversampling"]
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG, TROVS>;
impl<'a, REG> TROVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All oversampled conversions for a channel are run following a trigger"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Automatic)
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Triggered)
    }
}
#[doc = "Regular Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM {
    #[doc = "0: Oversampling is temporary stopped and continued after injection sequence"]
    Continued = 0,
    #[doc = "1: Oversampling is aborted and resumed from start after injection sequence"]
    Resumed = 1,
}
impl From<ROVSM> for bool {
    #[inline(always)]
    fn from(variant: ROVSM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVSM` reader - Regular Oversampling mode"]
pub type ROVSM_R = crate::BitReader<ROVSM>;
impl ROVSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROVSM {
        match self.bits {
            false => ROVSM::Continued,
            true => ROVSM::Resumed,
        }
    }
    #[doc = "Oversampling is temporary stopped and continued after injection sequence"]
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        *self == ROVSM::Continued
    }
    #[doc = "Oversampling is aborted and resumed from start after injection sequence"]
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == ROVSM::Resumed
    }
}
#[doc = "Field `ROVSM` writer - Regular Oversampling mode"]
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG, ROVSM>;
impl<'a, REG> ROVSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampling is temporary stopped and continued after injection sequence"]
    #[inline(always)]
    pub fn continued(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Continued)
    }
    #[doc = "Oversampling is aborted and resumed from start after injection sequence"]
    #[inline(always)]
    pub fn resumed(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Resumed)
    }
}
#[doc = "Gain compensation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCOMP {
    #[doc = "0: Regular ADC operating mode"]
    Disabled = 0,
    #[doc = "1: Gain compensation enabled and applies to all channels"]
    Enabled = 1,
}
impl From<GCOMP> for bool {
    #[inline(always)]
    fn from(variant: GCOMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCOMP` reader - Gain compensation mode"]
pub type GCOMP_R = crate::BitReader<GCOMP>;
impl GCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GCOMP {
        match self.bits {
            false => GCOMP::Disabled,
            true => GCOMP::Enabled,
        }
    }
    #[doc = "Regular ADC operating mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCOMP::Disabled
    }
    #[doc = "Gain compensation enabled and applies to all channels"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCOMP::Enabled
    }
}
#[doc = "Field `GCOMP` writer - Gain compensation mode"]
pub type GCOMP_W<'a, REG> = crate::BitWriter<'a, REG, GCOMP>;
impl<'a, REG> GCOMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular ADC operating mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCOMP::Disabled)
    }
    #[doc = "Gain compensation enabled and applies to all channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCOMP::Enabled)
    }
}
#[doc = "Software trigger bit for sampling time control trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG {
    #[doc = "0: End sampling period and start conversion"]
    Disabled = 0,
    #[doc = "1: Start sampling period"]
    Enabled = 1,
}
impl From<SWTRIG> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRIG` reader - Software trigger bit for sampling time control trigger mode"]
pub type SWTRIG_R = crate::BitReader<SWTRIG>;
impl SWTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWTRIG {
        match self.bits {
            false => SWTRIG::Disabled,
            true => SWTRIG::Enabled,
        }
    }
    #[doc = "End sampling period and start conversion"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWTRIG::Disabled
    }
    #[doc = "Start sampling period"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWTRIG::Enabled
    }
}
#[doc = "Field `SWTRIG` writer - Software trigger bit for sampling time control trigger mode"]
pub type SWTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG>;
impl<'a, REG> SWTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End sampling period and start conversion"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG::Disabled)
    }
    #[doc = "Start sampling period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG::Enabled)
    }
}
#[doc = "Bulb sampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BULB {
    #[doc = "0: Bulb sampling mode disabled"]
    Disabled = 0,
    #[doc = "1: Bulb sampling mode enabled. Immediately start sampling after last conversion finishes."]
    Enabled = 1,
}
impl From<BULB> for bool {
    #[inline(always)]
    fn from(variant: BULB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BULB` reader - Bulb sampling mode"]
pub type BULB_R = crate::BitReader<BULB>;
impl BULB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BULB {
        match self.bits {
            false => BULB::Disabled,
            true => BULB::Enabled,
        }
    }
    #[doc = "Bulb sampling mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BULB::Disabled
    }
    #[doc = "Bulb sampling mode enabled. Immediately start sampling after last conversion finishes."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BULB::Enabled
    }
}
#[doc = "Field `BULB` writer - Bulb sampling mode"]
pub type BULB_W<'a, REG> = crate::BitWriter<'a, REG, BULB>;
impl<'a, REG> BULB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bulb sampling mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BULB::Disabled)
    }
    #[doc = "Bulb sampling mode enabled. Immediately start sampling after last conversion finishes."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BULB::Enabled)
    }
}
#[doc = "Sampling time control trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPTRIG {
    #[doc = "0: Sampling time control trigger mode disabled"]
    Disabled = 0,
    #[doc = "1: Sampling time control trigger mode enabled"]
    Enabled = 1,
}
impl From<SMPTRIG> for bool {
    #[inline(always)]
    fn from(variant: SMPTRIG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPTRIG` reader - Sampling time control trigger mode"]
pub type SMPTRIG_R = crate::BitReader<SMPTRIG>;
impl SMPTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPTRIG {
        match self.bits {
            false => SMPTRIG::Disabled,
            true => SMPTRIG::Enabled,
        }
    }
    #[doc = "Sampling time control trigger mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMPTRIG::Disabled
    }
    #[doc = "Sampling time control trigger mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMPTRIG::Enabled
    }
}
#[doc = "Field `SMPTRIG` writer - Sampling time control trigger mode"]
pub type SMPTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SMPTRIG>;
impl<'a, REG> SMPTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time control trigger mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPTRIG::Disabled)
    }
    #[doc = "Sampling time control trigger mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPTRIG::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Regular Oversampling Enable"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Injected Oversampling Enable"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Gain compensation mode"]
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 25 - Software trigger bit for sampling time control trigger mode"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bulb sampling mode"]
    #[inline(always)]
    pub fn bulb(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Sampling time control trigger mode"]
    #[inline(always)]
    pub fn smptrig(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Regular Oversampling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Injected Oversampling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn trovs(&mut self) -> TROVS_W<CFGR2rs> {
        TROVS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
    #[doc = "Bit 16 - Gain compensation mode"]
    #[inline(always)]
    #[must_use]
    pub fn gcomp(&mut self) -> GCOMP_W<CFGR2rs> {
        GCOMP_W::new(self, 16)
    }
    #[doc = "Bit 25 - Software trigger bit for sampling time control trigger mode"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SWTRIG_W<CFGR2rs> {
        SWTRIG_W::new(self, 25)
    }
    #[doc = "Bit 26 - Bulb sampling mode"]
    #[inline(always)]
    #[must_use]
    pub fn bulb(&mut self) -> BULB_W<CFGR2rs> {
        BULB_W::new(self, 26)
    }
    #[doc = "Bit 27 - Sampling time control trigger mode"]
    #[inline(always)]
    #[must_use]
    pub fn smptrig(&mut self) -> SMPTRIG_W<CFGR2rs> {
        SMPTRIG_W::new(self, 27)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
