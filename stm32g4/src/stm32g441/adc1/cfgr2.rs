///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**Regular Oversampling Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE {
    ///0: Regular oversampling disabled
    Disabled = 0,
    ///1: Regular oversampling enabled
    Enabled = 1,
}
impl From<ROVSE> for bool {
    #[inline(always)]
    fn from(variant: ROVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVSE` reader - Regular Oversampling Enable
pub type ROVSE_R = crate::BitReader<ROVSE>;
impl ROVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROVSE {
        match self.bits {
            false => ROVSE::Disabled,
            true => ROVSE::Enabled,
        }
    }
    ///Regular oversampling disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE::Disabled
    }
    ///Regular oversampling enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE::Enabled
    }
}
///Field `ROVSE` writer - Regular Oversampling Enable
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG, ROVSE>;
impl<'a, REG> ROVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Disabled)
    }
    ///Regular oversampling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Enabled)
    }
}
/**Injected Oversampling Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE {
    ///0: Injected oversampling disabled
    Disabled = 0,
    ///1: Injected oversampling enabled
    Enabled = 1,
}
impl From<JOVSE> for bool {
    #[inline(always)]
    fn from(variant: JOVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `JOVSE` reader - Injected Oversampling Enable
pub type JOVSE_R = crate::BitReader<JOVSE>;
impl JOVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JOVSE {
        match self.bits {
            false => JOVSE::Disabled,
            true => JOVSE::Enabled,
        }
    }
    ///Injected oversampling disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE::Disabled
    }
    ///Injected oversampling enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE::Enabled
    }
}
///Field `JOVSE` writer - Injected Oversampling Enable
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG, JOVSE>;
impl<'a, REG> JOVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Injected oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Disabled)
    }
    ///Injected oversampling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Enabled)
    }
}
/**Oversampling ratio

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR {
    ///0: Oversampling ratio of 2
    Os2 = 0,
    ///1: Oversampling ratio of 4
    Os4 = 1,
    ///2: Oversampling ratio of 8
    Os8 = 2,
    ///3: Oversampling ratio of 16
    Os16 = 3,
    ///4: Oversampling ratio of 32
    Os32 = 4,
    ///5: Oversampling ratio of 64
    Os64 = 5,
    ///6: Oversampling ratio of 128
    Os128 = 6,
    ///7: Oversampling ratio of 256
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
impl crate::IsEnum for OVSR {}
///Field `OVSR` reader - Oversampling ratio
pub type OVSR_R = crate::FieldReader<OVSR>;
impl OVSR_R {
    ///Get enumerated values variant
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
    ///Oversampling ratio of 2
    #[inline(always)]
    pub fn is_os2(&self) -> bool {
        *self == OVSR::Os2
    }
    ///Oversampling ratio of 4
    #[inline(always)]
    pub fn is_os4(&self) -> bool {
        *self == OVSR::Os4
    }
    ///Oversampling ratio of 8
    #[inline(always)]
    pub fn is_os8(&self) -> bool {
        *self == OVSR::Os8
    }
    ///Oversampling ratio of 16
    #[inline(always)]
    pub fn is_os16(&self) -> bool {
        *self == OVSR::Os16
    }
    ///Oversampling ratio of 32
    #[inline(always)]
    pub fn is_os32(&self) -> bool {
        *self == OVSR::Os32
    }
    ///Oversampling ratio of 64
    #[inline(always)]
    pub fn is_os64(&self) -> bool {
        *self == OVSR::Os64
    }
    ///Oversampling ratio of 128
    #[inline(always)]
    pub fn is_os128(&self) -> bool {
        *self == OVSR::Os128
    }
    ///Oversampling ratio of 256
    #[inline(always)]
    pub fn is_os256(&self) -> bool {
        *self == OVSR::Os256
    }
}
///Field `OVSR` writer - Oversampling ratio
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OVSR, crate::Safe>;
impl<'a, REG> OVSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Oversampling ratio of 2
    #[inline(always)]
    pub fn os2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os2)
    }
    ///Oversampling ratio of 4
    #[inline(always)]
    pub fn os4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os4)
    }
    ///Oversampling ratio of 8
    #[inline(always)]
    pub fn os8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os8)
    }
    ///Oversampling ratio of 16
    #[inline(always)]
    pub fn os16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os16)
    }
    ///Oversampling ratio of 32
    #[inline(always)]
    pub fn os32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os32)
    }
    ///Oversampling ratio of 64
    #[inline(always)]
    pub fn os64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os64)
    }
    ///Oversampling ratio of 128
    #[inline(always)]
    pub fn os128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os128)
    }
    ///Oversampling ratio of 256
    #[inline(always)]
    pub fn os256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Os256)
    }
}
/**Oversampling shift

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS {
    ///0: No right shift applied to oversampling result
    NoShift = 0,
    ///1: Shift oversampling result right by 1 bit
    Shift1 = 1,
    ///2: Shift oversampling result right by 2 bits
    Shift2 = 2,
    ///3: Shift oversampling result right by 3 bits
    Shift3 = 3,
    ///4: Shift oversampling result right by 4 bits
    Shift4 = 4,
    ///5: Shift oversampling result right by 5 bits
    Shift5 = 5,
    ///6: Shift oversampling result right by 6 bits
    Shift6 = 6,
    ///7: Shift oversampling result right by 7 bits
    Shift7 = 7,
    ///8: Shift oversampling result right by 8 bits
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
impl crate::IsEnum for OVSS {}
///Field `OVSS` reader - Oversampling shift
pub type OVSS_R = crate::FieldReader<OVSS>;
impl OVSS_R {
    ///Get enumerated values variant
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
    ///No right shift applied to oversampling result
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS::NoShift
    }
    ///Shift oversampling result right by 1 bit
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        *self == OVSS::Shift1
    }
    ///Shift oversampling result right by 2 bits
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        *self == OVSS::Shift2
    }
    ///Shift oversampling result right by 3 bits
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        *self == OVSS::Shift3
    }
    ///Shift oversampling result right by 4 bits
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        *self == OVSS::Shift4
    }
    ///Shift oversampling result right by 5 bits
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        *self == OVSS::Shift5
    }
    ///Shift oversampling result right by 6 bits
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        *self == OVSS::Shift6
    }
    ///Shift oversampling result right by 7 bits
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        *self == OVSS::Shift7
    }
    ///Shift oversampling result right by 8 bits
    #[inline(always)]
    pub fn is_shift8(&self) -> bool {
        *self == OVSS::Shift8
    }
}
///Field `OVSS` writer - Oversampling shift
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSS>;
impl<'a, REG> OVSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No right shift applied to oversampling result
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::NoShift)
    }
    ///Shift oversampling result right by 1 bit
    #[inline(always)]
    pub fn shift1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift1)
    }
    ///Shift oversampling result right by 2 bits
    #[inline(always)]
    pub fn shift2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift2)
    }
    ///Shift oversampling result right by 3 bits
    #[inline(always)]
    pub fn shift3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift3)
    }
    ///Shift oversampling result right by 4 bits
    #[inline(always)]
    pub fn shift4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift4)
    }
    ///Shift oversampling result right by 5 bits
    #[inline(always)]
    pub fn shift5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift5)
    }
    ///Shift oversampling result right by 6 bits
    #[inline(always)]
    pub fn shift6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift6)
    }
    ///Shift oversampling result right by 7 bits
    #[inline(always)]
    pub fn shift7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift7)
    }
    ///Shift oversampling result right by 8 bits
    #[inline(always)]
    pub fn shift8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift8)
    }
}
/**Triggered Regular Oversampling

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS {
    ///0: All oversampled conversions for a channel are run following a trigger
    Automatic = 0,
    ///1: Each oversampled conversion for a channel needs a new trigger
    Triggered = 1,
}
impl From<TROVS> for bool {
    #[inline(always)]
    fn from(variant: TROVS) -> Self {
        variant as u8 != 0
    }
}
///Field `TROVS` reader - Triggered Regular Oversampling
pub type TROVS_R = crate::BitReader<TROVS>;
impl TROVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TROVS {
        match self.bits {
            false => TROVS::Automatic,
            true => TROVS::Triggered,
        }
    }
    ///All oversampled conversions for a channel are run following a trigger
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TROVS::Automatic
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TROVS::Triggered
    }
}
///Field `TROVS` writer - Triggered Regular Oversampling
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG, TROVS>;
impl<'a, REG> TROVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All oversampled conversions for a channel are run following a trigger
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Automatic)
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Triggered)
    }
}
/**Regular Oversampling mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM {
    ///0: Oversampling is temporary stopped and continued after injection sequence
    Continued = 0,
    ///1: Oversampling is aborted and resumed from start after injection sequence
    Resumed = 1,
}
impl From<ROVSM> for bool {
    #[inline(always)]
    fn from(variant: ROVSM) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVSM` reader - Regular Oversampling mode
pub type ROVSM_R = crate::BitReader<ROVSM>;
impl ROVSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROVSM {
        match self.bits {
            false => ROVSM::Continued,
            true => ROVSM::Resumed,
        }
    }
    ///Oversampling is temporary stopped and continued after injection sequence
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        *self == ROVSM::Continued
    }
    ///Oversampling is aborted and resumed from start after injection sequence
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == ROVSM::Resumed
    }
}
///Field `ROVSM` writer - Regular Oversampling mode
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG, ROVSM>;
impl<'a, REG> ROVSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oversampling is temporary stopped and continued after injection sequence
    #[inline(always)]
    pub fn continued(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Continued)
    }
    ///Oversampling is aborted and resumed from start after injection sequence
    #[inline(always)]
    pub fn resumed(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Resumed)
    }
}
/**Gain compensation mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCOMP {
    ///0: Regular ADC operating mode
    Disabled = 0,
    ///1: Gain compensation enabled and applies to all channels
    Enabled = 1,
}
impl From<GCOMP> for bool {
    #[inline(always)]
    fn from(variant: GCOMP) -> Self {
        variant as u8 != 0
    }
}
///Field `GCOMP` reader - Gain compensation mode
pub type GCOMP_R = crate::BitReader<GCOMP>;
impl GCOMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GCOMP {
        match self.bits {
            false => GCOMP::Disabled,
            true => GCOMP::Enabled,
        }
    }
    ///Regular ADC operating mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCOMP::Disabled
    }
    ///Gain compensation enabled and applies to all channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCOMP::Enabled
    }
}
///Field `GCOMP` writer - Gain compensation mode
pub type GCOMP_W<'a, REG> = crate::BitWriter<'a, REG, GCOMP>;
impl<'a, REG> GCOMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular ADC operating mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCOMP::Disabled)
    }
    ///Gain compensation enabled and applies to all channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCOMP::Enabled)
    }
}
/**Software trigger bit for sampling time control trigger mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG {
    ///0: End sampling period and start conversion
    Disabled = 0,
    ///1: Start sampling period
    Enabled = 1,
}
impl From<SWTRIG> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `SWTRIG` reader - Software trigger bit for sampling time control trigger mode
pub type SWTRIG_R = crate::BitReader<SWTRIG>;
impl SWTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWTRIG {
        match self.bits {
            false => SWTRIG::Disabled,
            true => SWTRIG::Enabled,
        }
    }
    ///End sampling period and start conversion
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWTRIG::Disabled
    }
    ///Start sampling period
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWTRIG::Enabled
    }
}
///Field `SWTRIG` writer - Software trigger bit for sampling time control trigger mode
pub type SWTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG>;
impl<'a, REG> SWTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End sampling period and start conversion
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG::Disabled)
    }
    ///Start sampling period
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG::Enabled)
    }
}
/**Bulb sampling mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BULB {
    ///0: Bulb sampling mode disabled
    Disabled = 0,
    ///1: Bulb sampling mode enabled. Immediately start sampling after last conversion finishes.
    Enabled = 1,
}
impl From<BULB> for bool {
    #[inline(always)]
    fn from(variant: BULB) -> Self {
        variant as u8 != 0
    }
}
///Field `BULB` reader - Bulb sampling mode
pub type BULB_R = crate::BitReader<BULB>;
impl BULB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BULB {
        match self.bits {
            false => BULB::Disabled,
            true => BULB::Enabled,
        }
    }
    ///Bulb sampling mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BULB::Disabled
    }
    ///Bulb sampling mode enabled. Immediately start sampling after last conversion finishes.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BULB::Enabled
    }
}
///Field `BULB` writer - Bulb sampling mode
pub type BULB_W<'a, REG> = crate::BitWriter<'a, REG, BULB>;
impl<'a, REG> BULB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bulb sampling mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BULB::Disabled)
    }
    ///Bulb sampling mode enabled. Immediately start sampling after last conversion finishes.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BULB::Enabled)
    }
}
/**Sampling time control trigger mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPTRIG {
    ///0: Sampling time control trigger mode disabled
    Disabled = 0,
    ///1: Sampling time control trigger mode enabled
    Enabled = 1,
}
impl From<SMPTRIG> for bool {
    #[inline(always)]
    fn from(variant: SMPTRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPTRIG` reader - Sampling time control trigger mode
pub type SMPTRIG_R = crate::BitReader<SMPTRIG>;
impl SMPTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPTRIG {
        match self.bits {
            false => SMPTRIG::Disabled,
            true => SMPTRIG::Enabled,
        }
    }
    ///Sampling time control trigger mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMPTRIG::Disabled
    }
    ///Sampling time control trigger mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMPTRIG::Enabled
    }
}
///Field `SMPTRIG` writer - Sampling time control trigger mode
pub type SMPTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SMPTRIG>;
impl<'a, REG> SMPTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time control trigger mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPTRIG::Disabled)
    }
    ///Sampling time control trigger mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPTRIG::Enabled)
    }
}
impl R {
    ///Bit 0 - Regular Oversampling Enable
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Injected Oversampling Enable
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered Regular Oversampling
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Regular Oversampling mode
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - Gain compensation mode
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 25 - Software trigger bit for sampling time control trigger mode
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Bulb sampling mode
    #[inline(always)]
    pub fn bulb(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Sampling time control trigger mode
    #[inline(always)]
    pub fn smptrig(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("smptrig", &self.smptrig())
            .field("bulb", &self.bulb())
            .field("swtrig", &self.swtrig())
            .field("gcomp", &self.gcomp())
            .field("rovsm", &self.rovsm())
            .field("trovs", &self.trovs())
            .field("ovss", &self.ovss())
            .field("ovsr", &self.ovsr())
            .field("jovse", &self.jovse())
            .field("rovse", &self.rovse())
            .finish()
    }
}
impl W {
    ///Bit 0 - Regular Oversampling Enable
    #[inline(always)]
    pub fn rovse(&mut self) -> ROVSE_W<'_, CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    ///Bit 1 - Injected Oversampling Enable
    #[inline(always)]
    pub fn jovse(&mut self) -> JOVSE_W<'_, CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W<'_, CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W<'_, CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - Triggered Regular Oversampling
    #[inline(always)]
    pub fn trovs(&mut self) -> TROVS_W<'_, CFGR2rs> {
        TROVS_W::new(self, 9)
    }
    ///Bit 10 - Regular Oversampling mode
    #[inline(always)]
    pub fn rovsm(&mut self) -> ROVSM_W<'_, CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
    ///Bit 16 - Gain compensation mode
    #[inline(always)]
    pub fn gcomp(&mut self) -> GCOMP_W<'_, CFGR2rs> {
        GCOMP_W::new(self, 16)
    }
    ///Bit 25 - Software trigger bit for sampling time control trigger mode
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W<'_, CFGR2rs> {
        SWTRIG_W::new(self, 25)
    }
    ///Bit 26 - Bulb sampling mode
    #[inline(always)]
    pub fn bulb(&mut self) -> BULB_W<'_, CFGR2rs> {
        BULB_W::new(self, 26)
    }
    ///Bit 27 - Sampling time control trigger mode
    #[inline(always)]
    pub fn smptrig(&mut self) -> SMPTRIG_W<'_, CFGR2rs> {
        SMPTRIG_W::new(self, 27)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#ADC1:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
