///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**Regular oversampling Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE {
    ///0: Regular Oversampling disabled
    Disabled = 0,
    ///1: Regular Oversampling enabled
    Enabled = 1,
}
impl From<ROVSE> for bool {
    #[inline(always)]
    fn from(variant: ROVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVSE` reader - Regular oversampling Enable
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
    ///Regular Oversampling disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE::Disabled
    }
    ///Regular Oversampling enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE::Enabled
    }
}
///Field `ROVSE` writer - Regular oversampling Enable
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG, ROVSE>;
impl<'a, REG> ROVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular Oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Disabled)
    }
    ///Regular Oversampling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Enabled)
    }
}
/**Injected oversampling Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE {
    ///0: Injected Oversampling disabled
    Disabled = 0,
    ///1: Injected Oversampling enabled
    Enabled = 1,
}
impl From<JOVSE> for bool {
    #[inline(always)]
    fn from(variant: JOVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `JOVSE` reader - Injected oversampling Enable
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
    ///Injected Oversampling disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE::Disabled
    }
    ///Injected Oversampling enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE::Enabled
    }
}
///Field `JOVSE` writer - Injected oversampling Enable
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG, JOVSE>;
impl<'a, REG> JOVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Injected Oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Disabled)
    }
    ///Injected Oversampling enabled
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
    ///0: 2x
    Ratio2 = 0,
    ///1: 4x
    Ratio4 = 1,
    ///2: 8x
    Ratio8 = 2,
    ///3: 16x
    Ratio16 = 3,
    ///4: 32x
    Ratio32 = 4,
    ///5: 64x
    Ratio64 = 5,
    ///6: 128x
    Ratio128 = 6,
    ///7: 256x
    Ratio256 = 7,
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
            0 => OVSR::Ratio2,
            1 => OVSR::Ratio4,
            2 => OVSR::Ratio8,
            3 => OVSR::Ratio16,
            4 => OVSR::Ratio32,
            5 => OVSR::Ratio64,
            6 => OVSR::Ratio128,
            7 => OVSR::Ratio256,
            _ => unreachable!(),
        }
    }
    ///2x
    #[inline(always)]
    pub fn is_ratio2(&self) -> bool {
        *self == OVSR::Ratio2
    }
    ///4x
    #[inline(always)]
    pub fn is_ratio4(&self) -> bool {
        *self == OVSR::Ratio4
    }
    ///8x
    #[inline(always)]
    pub fn is_ratio8(&self) -> bool {
        *self == OVSR::Ratio8
    }
    ///16x
    #[inline(always)]
    pub fn is_ratio16(&self) -> bool {
        *self == OVSR::Ratio16
    }
    ///32x
    #[inline(always)]
    pub fn is_ratio32(&self) -> bool {
        *self == OVSR::Ratio32
    }
    ///64x
    #[inline(always)]
    pub fn is_ratio64(&self) -> bool {
        *self == OVSR::Ratio64
    }
    ///128x
    #[inline(always)]
    pub fn is_ratio128(&self) -> bool {
        *self == OVSR::Ratio128
    }
    ///256x
    #[inline(always)]
    pub fn is_ratio256(&self) -> bool {
        *self == OVSR::Ratio256
    }
}
///Field `OVSR` writer - Oversampling ratio
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OVSR, crate::Safe>;
impl<'a, REG> OVSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2x
    #[inline(always)]
    pub fn ratio2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio2)
    }
    ///4x
    #[inline(always)]
    pub fn ratio4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio4)
    }
    ///8x
    #[inline(always)]
    pub fn ratio8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio8)
    }
    ///16x
    #[inline(always)]
    pub fn ratio16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio16)
    }
    ///32x
    #[inline(always)]
    pub fn ratio32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio32)
    }
    ///64x
    #[inline(always)]
    pub fn ratio64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio64)
    }
    ///128x
    #[inline(always)]
    pub fn ratio128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio128)
    }
    ///256x
    #[inline(always)]
    pub fn ratio256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio256)
    }
}
/**Oversampling shift

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS {
    ///0: No Shift
    NoShift = 0,
    ///1: Shift 1-bit
    Shift1bit = 1,
    ///2: Shift 2-bit
    Shift2bit = 2,
    ///3: Shift 3-bit
    Shift3bit = 3,
    ///4: Shift 4-bit
    Shift4bit = 4,
    ///5: Shift 5-bit
    Shift5bit = 5,
    ///6: Shift 6-bit
    Shift6bit = 6,
    ///7: Shift 7-bit
    Shift7bit = 7,
    ///8: Shift 8-bit
    Shift8bit = 8,
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
            1 => Some(OVSS::Shift1bit),
            2 => Some(OVSS::Shift2bit),
            3 => Some(OVSS::Shift3bit),
            4 => Some(OVSS::Shift4bit),
            5 => Some(OVSS::Shift5bit),
            6 => Some(OVSS::Shift6bit),
            7 => Some(OVSS::Shift7bit),
            8 => Some(OVSS::Shift8bit),
            _ => None,
        }
    }
    ///No Shift
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS::NoShift
    }
    ///Shift 1-bit
    #[inline(always)]
    pub fn is_shift1bit(&self) -> bool {
        *self == OVSS::Shift1bit
    }
    ///Shift 2-bit
    #[inline(always)]
    pub fn is_shift2bit(&self) -> bool {
        *self == OVSS::Shift2bit
    }
    ///Shift 3-bit
    #[inline(always)]
    pub fn is_shift3bit(&self) -> bool {
        *self == OVSS::Shift3bit
    }
    ///Shift 4-bit
    #[inline(always)]
    pub fn is_shift4bit(&self) -> bool {
        *self == OVSS::Shift4bit
    }
    ///Shift 5-bit
    #[inline(always)]
    pub fn is_shift5bit(&self) -> bool {
        *self == OVSS::Shift5bit
    }
    ///Shift 6-bit
    #[inline(always)]
    pub fn is_shift6bit(&self) -> bool {
        *self == OVSS::Shift6bit
    }
    ///Shift 7-bit
    #[inline(always)]
    pub fn is_shift7bit(&self) -> bool {
        *self == OVSS::Shift7bit
    }
    ///Shift 8-bit
    #[inline(always)]
    pub fn is_shift8bit(&self) -> bool {
        *self == OVSS::Shift8bit
    }
}
///Field `OVSS` writer - Oversampling shift
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSS>;
impl<'a, REG> OVSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No Shift
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::NoShift)
    }
    ///Shift 1-bit
    #[inline(always)]
    pub fn shift1bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift1bit)
    }
    ///Shift 2-bit
    #[inline(always)]
    pub fn shift2bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift2bit)
    }
    ///Shift 3-bit
    #[inline(always)]
    pub fn shift3bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift3bit)
    }
    ///Shift 4-bit
    #[inline(always)]
    pub fn shift4bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift4bit)
    }
    ///Shift 5-bit
    #[inline(always)]
    pub fn shift5bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift5bit)
    }
    ///Shift 6-bit
    #[inline(always)]
    pub fn shift6bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift6bit)
    }
    ///Shift 7-bit
    #[inline(always)]
    pub fn shift7bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift7bit)
    }
    ///Shift 8-bit
    #[inline(always)]
    pub fn shift8bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift8bit)
    }
}
/**Triggered Regular oversampling

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS {
    ///0: All oversampled conversions for a channel are done consecutively following a trigger
    All = 0,
    ///1: Each oversampled conversion for a channel needs a new trigger
    Single = 1,
}
impl From<TROVS> for bool {
    #[inline(always)]
    fn from(variant: TROVS) -> Self {
        variant as u8 != 0
    }
}
///Field `TROVS` reader - Triggered Regular oversampling
pub type TROVS_R = crate::BitReader<TROVS>;
impl TROVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TROVS {
        match self.bits {
            false => TROVS::All,
            true => TROVS::Single,
        }
    }
    ///All oversampled conversions for a channel are done consecutively following a trigger
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == TROVS::All
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TROVS::Single
    }
}
///Field `TROVS` writer - Triggered Regular oversampling
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG, TROVS>;
impl<'a, REG> TROVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All oversampled conversions for a channel are done consecutively following a trigger
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::All)
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Single)
    }
}
/**Regular oversampling mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM {
    ///0: When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)
    ContinuedMode = 0,
    ///1: When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)
    ResumedMode = 1,
}
impl From<ROVSM> for bool {
    #[inline(always)]
    fn from(variant: ROVSM) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVSM` reader - Regular oversampling mode
pub type ROVSM_R = crate::BitReader<ROVSM>;
impl ROVSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROVSM {
        match self.bits {
            false => ROVSM::ContinuedMode,
            true => ROVSM::ResumedMode,
        }
    }
    ///When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)
    #[inline(always)]
    pub fn is_continued_mode(&self) -> bool {
        *self == ROVSM::ContinuedMode
    }
    ///When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)
    #[inline(always)]
    pub fn is_resumed_mode(&self) -> bool {
        *self == ROVSM::ResumedMode
    }
}
///Field `ROVSM` writer - Regular oversampling mode
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG, ROVSM>;
impl<'a, REG> ROVSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)
    #[inline(always)]
    pub fn continued_mode(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::ContinuedMode)
    }
    ///When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)
    #[inline(always)]
    pub fn resumed_mode(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::ResumedMode)
    }
}
/**Software trigger bit for sampling time control trigger mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG {
    ///0: Software trigger starts the conversion for sampling time control trigger mode
    Conversion = 0,
    ///1: Software trigger starts the sampling for sampling time control trigger mode
    Sampling = 1,
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
            false => SWTRIG::Conversion,
            true => SWTRIG::Sampling,
        }
    }
    ///Software trigger starts the conversion for sampling time control trigger mode
    #[inline(always)]
    pub fn is_conversion(&self) -> bool {
        *self == SWTRIG::Conversion
    }
    ///Software trigger starts the sampling for sampling time control trigger mode
    #[inline(always)]
    pub fn is_sampling(&self) -> bool {
        *self == SWTRIG::Sampling
    }
}
///Field `SWTRIG` writer - Software trigger bit for sampling time control trigger mode
pub type SWTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG>;
impl<'a, REG> SWTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger starts the conversion for sampling time control trigger mode
    #[inline(always)]
    pub fn conversion(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG::Conversion)
    }
    ///Software trigger starts the sampling for sampling time control trigger mode
    #[inline(always)]
    pub fn sampling(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG::Sampling)
    }
}
/**Bulb sampling mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BULB {
    ///0: Bulb sampling mode disabled
    Disabled = 0,
    ///1: Bulb sampling mode enabled. The sampling period starts just after the previous end of conversion
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
    ///Bulb sampling mode enabled. The sampling period starts just after the previous end of conversion
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
    ///Bulb sampling mode enabled. The sampling period starts just after the previous end of conversion
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
    ///Bit 0 - Regular oversampling Enable
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Injected oversampling Enable
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
    ///Bit 9 - Triggered Regular oversampling
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Regular oversampling mode
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
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
            .field("rovse", &self.rovse())
            .field("jovse", &self.jovse())
            .field("ovsr", &self.ovsr())
            .field("ovss", &self.ovss())
            .field("trovs", &self.trovs())
            .field("rovsm", &self.rovsm())
            .field("swtrig", &self.swtrig())
            .field("bulb", &self.bulb())
            .field("smptrig", &self.smptrig())
            .finish()
    }
}
impl W {
    ///Bit 0 - Regular oversampling Enable
    #[inline(always)]
    pub fn rovse(&mut self) -> ROVSE_W<'_, CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    ///Bit 1 - Injected oversampling Enable
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
    ///Bit 9 - Triggered Regular oversampling
    #[inline(always)]
    pub fn trovs(&mut self) -> TROVS_W<'_, CFGR2rs> {
        TROVS_W::new(self, 9)
    }
    ///Bit 10 - Regular oversampling mode
    #[inline(always)]
    pub fn rovsm(&mut self) -> ROVSM_W<'_, CFGR2rs> {
        ROVSM_W::new(self, 10)
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
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#ADC1:CFGR2)*/
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
