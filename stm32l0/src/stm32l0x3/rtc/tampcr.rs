///Register `TAMPCR` reader
pub type R = crate::R<TAMPCRrs>;
///Register `TAMPCR` writer
pub type W = crate::W<TAMPCRrs>;
/**RTC_TAMP1 input detection enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1E {
    ///0: RTC_TAMPx input detection disabled
    Disabled = 0,
    ///1: RTC_TAMPx input detection enabled
    Enabled = 1,
}
impl From<TAMP1E> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1E` reader - RTC_TAMP1 input detection enable
pub type TAMP1E_R = crate::BitReader<TAMP1E>;
impl TAMP1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1E {
        match self.bits {
            false => TAMP1E::Disabled,
            true => TAMP1E::Enabled,
        }
    }
    ///RTC_TAMPx input detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1E::Disabled
    }
    ///RTC_TAMPx input detection enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1E::Enabled
    }
}
///Field `TAMP1E` writer - RTC_TAMP1 input detection enable
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1E>;
impl<'a, REG> TAMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC_TAMPx input detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E::Disabled)
    }
    ///RTC_TAMPx input detection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E::Enabled)
    }
}
/**Active level for RTC_TAMP1 input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1TRG {
    ///0: If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input staying low triggers a tamper detection event.
    RisingEdge = 0,
    ///1: If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input falling edge triggers a tamper detection event
    FallingEdge = 1,
}
impl From<TAMP1TRG> for bool {
    #[inline(always)]
    fn from(variant: TAMP1TRG) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1TRG` reader - Active level for RTC_TAMP1 input
pub type TAMP1TRG_R = crate::BitReader<TAMP1TRG>;
impl TAMP1TRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1TRG {
        match self.bits {
            false => TAMP1TRG::RisingEdge,
            true => TAMP1TRG::FallingEdge,
        }
    }
    ///If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input staying low triggers a tamper detection event.
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TAMP1TRG::RisingEdge
    }
    ///If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input falling edge triggers a tamper detection event
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TAMP1TRG::FallingEdge
    }
}
///Field `TAMP1TRG` writer - Active level for RTC_TAMP1 input
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1TRG>;
impl<'a, REG> TAMP1TRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input staying low triggers a tamper detection event.
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1TRG::RisingEdge)
    }
    ///If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input falling edge triggers a tamper detection event
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1TRG::FallingEdge)
    }
}
/**Tamper interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPIE {
    ///0: Tamper interrupt disabled
    Disabled = 0,
    ///1: Tamper interrupt enabled
    Enabled = 1,
}
impl From<TAMPIE> for bool {
    #[inline(always)]
    fn from(variant: TAMPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPIE` reader - Tamper interrupt enable
pub type TAMPIE_R = crate::BitReader<TAMPIE>;
impl TAMPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPIE {
        match self.bits {
            false => TAMPIE::Disabled,
            true => TAMPIE::Enabled,
        }
    }
    ///Tamper interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPIE::Disabled
    }
    ///Tamper interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPIE::Enabled
    }
}
///Field `TAMPIE` writer - Tamper interrupt enable
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG, TAMPIE>;
impl<'a, REG> TAMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPIE::Disabled)
    }
    ///Tamper interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPIE::Enabled)
    }
}
///Field `TAMP2E` reader - RTC_TAMP2 input detection enable
pub use TAMP1E_R as TAMP2E_R;
///Field `TAMP3E` reader - RTC_TAMP3 detection enable
pub use TAMP1E_R as TAMP3E_R;
///Field `TAMP2E` writer - RTC_TAMP2 input detection enable
pub use TAMP1E_W as TAMP2E_W;
///Field `TAMP3E` writer - RTC_TAMP3 detection enable
pub use TAMP1E_W as TAMP3E_W;
///Field `TAMP2TRG` reader - Active level for RTC_TAMP2 input
pub use TAMP1TRG_R as TAMP2TRG_R;
///Field `TAMP3TRG` reader - Active level for RTC_TAMP3 input
pub use TAMP1TRG_R as TAMP3TRG_R;
///Field `TAMP2TRG` writer - Active level for RTC_TAMP2 input
pub use TAMP1TRG_W as TAMP2TRG_W;
///Field `TAMP3TRG` writer - Active level for RTC_TAMP3 input
pub use TAMP1TRG_W as TAMP3TRG_W;
/**Activate timestamp on tamper detection event

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPTS {
    ///0: Tamper detection event does not cause a timestamp to be saved
    NoSave = 0,
    ///1: Save timestamp on tamper detection event
    Save = 1,
}
impl From<TAMPTS> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPTS` reader - Activate timestamp on tamper detection event
pub type TAMPTS_R = crate::BitReader<TAMPTS>;
impl TAMPTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPTS {
        match self.bits {
            false => TAMPTS::NoSave,
            true => TAMPTS::Save,
        }
    }
    ///Tamper detection event does not cause a timestamp to be saved
    #[inline(always)]
    pub fn is_no_save(&self) -> bool {
        *self == TAMPTS::NoSave
    }
    ///Save timestamp on tamper detection event
    #[inline(always)]
    pub fn is_save(&self) -> bool {
        *self == TAMPTS::Save
    }
}
///Field `TAMPTS` writer - Activate timestamp on tamper detection event
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPTS>;
impl<'a, REG> TAMPTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper detection event does not cause a timestamp to be saved
    #[inline(always)]
    pub fn no_save(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS::NoSave)
    }
    ///Save timestamp on tamper detection event
    #[inline(always)]
    pub fn save(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS::Save)
    }
}
/**Tamper sampling frequency

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFREQ {
    ///0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    Div32768 = 0,
    ///1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    Div16384 = 1,
    ///2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    Div8192 = 2,
    ///3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    Div4096 = 3,
    ///4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    Div2048 = 4,
    ///5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    Div1024 = 5,
    ///6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    Div512 = 6,
    ///7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    Div256 = 7,
}
impl From<TAMPFREQ> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFREQ {
    type Ux = u8;
}
impl crate::IsEnum for TAMPFREQ {}
///Field `TAMPFREQ` reader - Tamper sampling frequency
pub type TAMPFREQ_R = crate::FieldReader<TAMPFREQ>;
impl TAMPFREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFREQ {
        match self.bits {
            0 => TAMPFREQ::Div32768,
            1 => TAMPFREQ::Div16384,
            2 => TAMPFREQ::Div8192,
            3 => TAMPFREQ::Div4096,
            4 => TAMPFREQ::Div2048,
            5 => TAMPFREQ::Div1024,
            6 => TAMPFREQ::Div512,
            7 => TAMPFREQ::Div256,
            _ => unreachable!(),
        }
    }
    ///RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == TAMPFREQ::Div32768
    }
    ///RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == TAMPFREQ::Div16384
    }
    ///RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == TAMPFREQ::Div8192
    }
    ///RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == TAMPFREQ::Div4096
    }
    ///RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == TAMPFREQ::Div2048
    }
    ///RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == TAMPFREQ::Div1024
    }
    ///RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == TAMPFREQ::Div512
    }
    ///RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == TAMPFREQ::Div256
    }
}
///Field `TAMPFREQ` writer - Tamper sampling frequency
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TAMPFREQ, crate::Safe>;
impl<'a, REG> TAMPFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div32768)
    }
    ///RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div16384)
    }
    ///RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div8192)
    }
    ///RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div4096)
    }
    ///RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div2048)
    }
    ///RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div1024)
    }
    ///RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div512)
    }
    ///RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div256)
    }
}
/**RTC_TAMPx filter count

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFLT {
    ///0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)
    Immediate = 0,
    ///1: Tamper event is activated after 2 consecutive samples at the active level
    Samples2 = 1,
    ///2: Tamper event is activated after 4 consecutive samples at the active level
    Samples4 = 2,
    ///3: Tamper event is activated after 8 consecutive samples at the active level
    Samples8 = 3,
}
impl From<TAMPFLT> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFLT {
    type Ux = u8;
}
impl crate::IsEnum for TAMPFLT {}
///Field `TAMPFLT` reader - RTC_TAMPx filter count
pub type TAMPFLT_R = crate::FieldReader<TAMPFLT>;
impl TAMPFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFLT {
        match self.bits {
            0 => TAMPFLT::Immediate,
            1 => TAMPFLT::Samples2,
            2 => TAMPFLT::Samples4,
            3 => TAMPFLT::Samples8,
            _ => unreachable!(),
        }
    }
    ///Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == TAMPFLT::Immediate
    }
    ///Tamper event is activated after 2 consecutive samples at the active level
    #[inline(always)]
    pub fn is_samples2(&self) -> bool {
        *self == TAMPFLT::Samples2
    }
    ///Tamper event is activated after 4 consecutive samples at the active level
    #[inline(always)]
    pub fn is_samples4(&self) -> bool {
        *self == TAMPFLT::Samples4
    }
    ///Tamper event is activated after 8 consecutive samples at the active level
    #[inline(always)]
    pub fn is_samples8(&self) -> bool {
        *self == TAMPFLT::Samples8
    }
}
///Field `TAMPFLT` writer - RTC_TAMPx filter count
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TAMPFLT, crate::Safe>;
impl<'a, REG> TAMPFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Immediate)
    }
    ///Tamper event is activated after 2 consecutive samples at the active level
    #[inline(always)]
    pub fn samples2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Samples2)
    }
    ///Tamper event is activated after 4 consecutive samples at the active level
    #[inline(always)]
    pub fn samples4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Samples4)
    }
    ///Tamper event is activated after 8 consecutive samples at the active level
    #[inline(always)]
    pub fn samples8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Samples8)
    }
}
/**RTC_TAMPx precharge duration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPPRCH {
    ///0: 1 RTCCLK cycle
    Cycles1 = 0,
    ///1: 2 RTCCLK cycles
    Cycles2 = 1,
    ///2: 4 RTCCLK cycles
    Cycles4 = 2,
    ///3: 8 RTCCLK cycles
    Cycles8 = 3,
}
impl From<TAMPPRCH> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPPRCH {
    type Ux = u8;
}
impl crate::IsEnum for TAMPPRCH {}
///Field `TAMPPRCH` reader - RTC_TAMPx precharge duration
pub type TAMPPRCH_R = crate::FieldReader<TAMPPRCH>;
impl TAMPPRCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPRCH {
        match self.bits {
            0 => TAMPPRCH::Cycles1,
            1 => TAMPPRCH::Cycles2,
            2 => TAMPPRCH::Cycles4,
            3 => TAMPPRCH::Cycles8,
            _ => unreachable!(),
        }
    }
    ///1 RTCCLK cycle
    #[inline(always)]
    pub fn is_cycles1(&self) -> bool {
        *self == TAMPPRCH::Cycles1
    }
    ///2 RTCCLK cycles
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == TAMPPRCH::Cycles2
    }
    ///4 RTCCLK cycles
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == TAMPPRCH::Cycles4
    }
    ///8 RTCCLK cycles
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == TAMPPRCH::Cycles8
    }
}
///Field `TAMPPRCH` writer - RTC_TAMPx precharge duration
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TAMPPRCH, crate::Safe>;
impl<'a, REG> TAMPPRCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 RTCCLK cycle
    #[inline(always)]
    pub fn cycles1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles1)
    }
    ///2 RTCCLK cycles
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles2)
    }
    ///4 RTCCLK cycles
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles4)
    }
    ///8 RTCCLK cycles
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles8)
    }
}
/**RTC_TAMPx pull-up disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPPUDIS {
    ///0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up)
    Enabled = 0,
    ///1: Disable precharge of RTC_TAMPx pins
    Disabled = 1,
}
impl From<TAMPPUDIS> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPPUDIS` reader - RTC_TAMPx pull-up disable
pub type TAMPPUDIS_R = crate::BitReader<TAMPPUDIS>;
impl TAMPPUDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPUDIS {
        match self.bits {
            false => TAMPPUDIS::Enabled,
            true => TAMPPUDIS::Disabled,
        }
    }
    ///Precharge RTC_TAMPx pins before sampling (enable internal pull-up)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPPUDIS::Enabled
    }
    ///Disable precharge of RTC_TAMPx pins
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPPUDIS::Disabled
    }
}
///Field `TAMPPUDIS` writer - RTC_TAMPx pull-up disable
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPPUDIS>;
impl<'a, REG> TAMPPUDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Precharge RTC_TAMPx pins before sampling (enable internal pull-up)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS::Enabled)
    }
    ///Disable precharge of RTC_TAMPx pins
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS::Disabled)
    }
}
/**Tamper 1 interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1IE {
    ///0: Tamper x interrupt is disabled if TAMPIE = 0
    Disabled = 0,
    ///1: Tamper x interrupt enabled
    Enabled = 1,
}
impl From<TAMP1IE> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1IE` reader - Tamper 1 interrupt enable
pub type TAMP1IE_R = crate::BitReader<TAMP1IE>;
impl TAMP1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1IE {
        match self.bits {
            false => TAMP1IE::Disabled,
            true => TAMP1IE::Enabled,
        }
    }
    ///Tamper x interrupt is disabled if TAMPIE = 0
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1IE::Disabled
    }
    ///Tamper x interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1IE::Enabled
    }
}
///Field `TAMP1IE` writer - Tamper 1 interrupt enable
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1IE>;
impl<'a, REG> TAMP1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper x interrupt is disabled if TAMPIE = 0
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE::Disabled)
    }
    ///Tamper x interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE::Enabled)
    }
}
/**Tamper 1 no erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1NOERASE {
    ///0: Tamper x event erases the backup registers
    Erase = 0,
    ///1: Tamper x event does not erase the backup registers
    NoErase = 1,
}
impl From<TAMP1NOERASE> for bool {
    #[inline(always)]
    fn from(variant: TAMP1NOERASE) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1NOERASE` reader - Tamper 1 no erase
pub type TAMP1NOERASE_R = crate::BitReader<TAMP1NOERASE>;
impl TAMP1NOERASE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1NOERASE {
        match self.bits {
            false => TAMP1NOERASE::Erase,
            true => TAMP1NOERASE::NoErase,
        }
    }
    ///Tamper x event erases the backup registers
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == TAMP1NOERASE::Erase
    }
    ///Tamper x event does not erase the backup registers
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        *self == TAMP1NOERASE::NoErase
    }
}
///Field `TAMP1NOERASE` writer - Tamper 1 no erase
pub type TAMP1NOERASE_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1NOERASE>;
impl<'a, REG> TAMP1NOERASE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper x event erases the backup registers
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1NOERASE::Erase)
    }
    ///Tamper x event does not erase the backup registers
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1NOERASE::NoErase)
    }
}
/**Tamper 1 mask flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1MF {
    ///0: Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection
    NotMasked = 0,
    ///1: Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased.
    Masked = 1,
}
impl From<TAMP1MF> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MF) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1MF` reader - Tamper 1 mask flag
pub type TAMP1MF_R = crate::BitReader<TAMP1MF>;
impl TAMP1MF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1MF {
        match self.bits {
            false => TAMP1MF::NotMasked,
            true => TAMP1MF::Masked,
        }
    }
    ///Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TAMP1MF::NotMasked
    }
    ///Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased.
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TAMP1MF::Masked
    }
}
///Field `TAMP1MF` writer - Tamper 1 mask flag
pub type TAMP1MF_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1MF>;
impl<'a, REG> TAMP1MF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1MF::NotMasked)
    }
    ///Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased.
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1MF::Masked)
    }
}
///Field `TAMP2IE` reader - Tamper 2 interrupt enable
pub use TAMP1IE_R as TAMP2IE_R;
///Field `TAMP3IE` reader - Tamper 3 interrupt enable
pub use TAMP1IE_R as TAMP3IE_R;
///Field `TAMP2IE` writer - Tamper 2 interrupt enable
pub use TAMP1IE_W as TAMP2IE_W;
///Field `TAMP3IE` writer - Tamper 3 interrupt enable
pub use TAMP1IE_W as TAMP3IE_W;
///Field `TAMP2MF` reader - Tamper 2 mask flag
pub use TAMP1MF_R as TAMP2MF_R;
///Field `TAMP3MF` reader - Tamper 3 mask flag
pub use TAMP1MF_R as TAMP3MF_R;
///Field `TAMP2MF` writer - Tamper 2 mask flag
pub use TAMP1MF_W as TAMP2MF_W;
///Field `TAMP3MF` writer - Tamper 3 mask flag
pub use TAMP1MF_W as TAMP3MF_W;
///Field `TAMP2NOERASE` reader - Tamper 2 no erase
pub use TAMP1NOERASE_R as TAMP2NOERASE_R;
///Field `TAMP3NOERASE` reader - Tamper 3 no erase
pub use TAMP1NOERASE_R as TAMP3NOERASE_R;
///Field `TAMP2NOERASE` writer - Tamper 2 no erase
pub use TAMP1NOERASE_W as TAMP2NOERASE_W;
///Field `TAMP3NOERASE` writer - Tamper 3 no erase
pub use TAMP1NOERASE_W as TAMP3NOERASE_W;
impl R {
    ///Bit 0 - RTC_TAMP1 input detection enable
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Active level for RTC_TAMP1 input
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTC_TAMP2 input detection enable
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Active level for RTC_TAMP2 input
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RTC_TAMP3 detection enable
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Active level for RTC_TAMP3 input
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:12 - RTC_TAMPx filter count
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - RTC_TAMPx precharge duration
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - RTC_TAMPx pull-up disable
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Tamper 1 interrupt enable
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noerase(&self) -> TAMP1NOERASE_R {
        TAMP1NOERASE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Tamper 1 mask flag
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Tamper 2 interrupt enable
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noerase(&self) -> TAMP2NOERASE_R {
        TAMP2NOERASE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Tamper 2 mask flag
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Tamper 3 interrupt enable
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Tamper 3 no erase
    #[inline(always)]
    pub fn tamp3noerase(&self) -> TAMP3NOERASE_R {
        TAMP3NOERASE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Tamper 3 mask flag
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMPCR")
            .field("tamp1mf", &self.tamp1mf())
            .field("tamp2mf", &self.tamp2mf())
            .field("tamp1noerase", &self.tamp1noerase())
            .field("tamp2noerase", &self.tamp2noerase())
            .field("tamp1ie", &self.tamp1ie())
            .field("tamp2ie", &self.tamp2ie())
            .field("tamppudis", &self.tamppudis())
            .field("tampprch", &self.tampprch())
            .field("tampflt", &self.tampflt())
            .field("tampfreq", &self.tampfreq())
            .field("tampts", &self.tampts())
            .field("tamp1trg", &self.tamp1trg())
            .field("tamp2trg", &self.tamp2trg())
            .field("tamp1e", &self.tamp1e())
            .field("tamp2e", &self.tamp2e())
            .field("tampie", &self.tampie())
            .field("tamp3mf", &self.tamp3mf())
            .field("tamp3noerase", &self.tamp3noerase())
            .field("tamp3ie", &self.tamp3ie())
            .field("tamp3trg", &self.tamp3trg())
            .field("tamp3e", &self.tamp3e())
            .finish()
    }
}
impl W {
    ///Bit 0 - RTC_TAMP1 input detection enable
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<'_, TAMPCRrs> {
        TAMP1E_W::new(self, 0)
    }
    ///Bit 1 - Active level for RTC_TAMP1 input
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<'_, TAMPCRrs> {
        TAMP1TRG_W::new(self, 1)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<'_, TAMPCRrs> {
        TAMPIE_W::new(self, 2)
    }
    ///Bit 3 - RTC_TAMP2 input detection enable
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<'_, TAMPCRrs> {
        TAMP2E_W::new(self, 3)
    }
    ///Bit 4 - Active level for RTC_TAMP2 input
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<'_, TAMPCRrs> {
        TAMP2TRG_W::new(self, 4)
    }
    ///Bit 5 - RTC_TAMP3 detection enable
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W<'_, TAMPCRrs> {
        TAMP3E_W::new(self, 5)
    }
    ///Bit 6 - Active level for RTC_TAMP3 input
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<'_, TAMPCRrs> {
        TAMP3TRG_W::new(self, 6)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<'_, TAMPCRrs> {
        TAMPTS_W::new(self, 7)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<'_, TAMPCRrs> {
        TAMPFREQ_W::new(self, 8)
    }
    ///Bits 11:12 - RTC_TAMPx filter count
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<'_, TAMPCRrs> {
        TAMPFLT_W::new(self, 11)
    }
    ///Bits 13:14 - RTC_TAMPx precharge duration
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<'_, TAMPCRrs> {
        TAMPPRCH_W::new(self, 13)
    }
    ///Bit 15 - RTC_TAMPx pull-up disable
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<'_, TAMPCRrs> {
        TAMPPUDIS_W::new(self, 15)
    }
    ///Bit 16 - Tamper 1 interrupt enable
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<'_, TAMPCRrs> {
        TAMP1IE_W::new(self, 16)
    }
    ///Bit 17 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noerase(&mut self) -> TAMP1NOERASE_W<'_, TAMPCRrs> {
        TAMP1NOERASE_W::new(self, 17)
    }
    ///Bit 18 - Tamper 1 mask flag
    #[inline(always)]
    pub fn tamp1mf(&mut self) -> TAMP1MF_W<'_, TAMPCRrs> {
        TAMP1MF_W::new(self, 18)
    }
    ///Bit 19 - Tamper 2 interrupt enable
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<'_, TAMPCRrs> {
        TAMP2IE_W::new(self, 19)
    }
    ///Bit 20 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noerase(&mut self) -> TAMP2NOERASE_W<'_, TAMPCRrs> {
        TAMP2NOERASE_W::new(self, 20)
    }
    ///Bit 21 - Tamper 2 mask flag
    #[inline(always)]
    pub fn tamp2mf(&mut self) -> TAMP2MF_W<'_, TAMPCRrs> {
        TAMP2MF_W::new(self, 21)
    }
    ///Bit 22 - Tamper 3 interrupt enable
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<'_, TAMPCRrs> {
        TAMP3IE_W::new(self, 22)
    }
    ///Bit 23 - Tamper 3 no erase
    #[inline(always)]
    pub fn tamp3noerase(&mut self) -> TAMP3NOERASE_W<'_, TAMPCRrs> {
        TAMP3NOERASE_W::new(self, 23)
    }
    ///Bit 24 - Tamper 3 mask flag
    #[inline(always)]
    pub fn tamp3mf(&mut self) -> TAMP3MF_W<'_, TAMPCRrs> {
        TAMP3MF_W::new(self, 24)
    }
}
/**RTC tamper configuration register

You can [`read`](crate::Reg::read) this register and get [`tampcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#RTC:TAMPCR)*/
pub struct TAMPCRrs;
impl crate::RegisterSpec for TAMPCRrs {
    type Ux = u32;
}
///`read()` method returns [`tampcr::R`](R) reader structure
impl crate::Readable for TAMPCRrs {}
///`write(|w| ..)` method takes [`tampcr::W`](W) writer structure
impl crate::Writable for TAMPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TAMPCR to value 0
impl crate::Resettable for TAMPCRrs {}
