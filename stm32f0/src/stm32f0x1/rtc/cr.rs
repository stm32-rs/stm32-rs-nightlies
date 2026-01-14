///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Time-stamp event active edge

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEDGE {
    ///0: RTC_TS input rising edge generates a time-stamp event
    RisingEdge = 0,
    ///1: RTC_TS input falling edge generates a time-stamp event
    FallingEdge = 1,
}
impl From<TSEDGE> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEDGE` reader - Time-stamp event active edge
pub type TSEDGE_R = crate::BitReader<TSEDGE>;
impl TSEDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSEDGE {
        match self.bits {
            false => TSEDGE::RisingEdge,
            true => TSEDGE::FallingEdge,
        }
    }
    ///RTC_TS input rising edge generates a time-stamp event
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TSEDGE::RisingEdge
    }
    ///RTC_TS input falling edge generates a time-stamp event
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TSEDGE::FallingEdge
    }
}
///Field `TSEDGE` writer - Time-stamp event active edge
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG, TSEDGE>;
impl<'a, REG> TSEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC_TS input rising edge generates a time-stamp event
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE::RisingEdge)
    }
    ///RTC_TS input falling edge generates a time-stamp event
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE::FallingEdge)
    }
}
/**RTC_REFIN reference clock detection enable (50 or 60 Hz)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFCKON {
    ///0: RTC_REFIN detection disabled
    Disabled = 0,
    ///1: RTC_REFIN detection enabled
    Enabled = 1,
}
impl From<REFCKON> for bool {
    #[inline(always)]
    fn from(variant: REFCKON) -> Self {
        variant as u8 != 0
    }
}
///Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)
pub type REFCKON_R = crate::BitReader<REFCKON>;
impl REFCKON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REFCKON {
        match self.bits {
            false => REFCKON::Disabled,
            true => REFCKON::Enabled,
        }
    }
    ///RTC_REFIN detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REFCKON::Disabled
    }
    ///RTC_REFIN detection enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REFCKON::Enabled
    }
}
///Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG, REFCKON>;
impl<'a, REG> REFCKON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC_REFIN detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON::Disabled)
    }
    ///RTC_REFIN detection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON::Enabled)
    }
}
///Field `BYPSHAD` reader - Bypass the shadow registers
pub type BYPSHAD_R = crate::BitReader;
///Field `BYPSHAD` writer - Bypass the shadow registers
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Hour format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMT {
    ///0: 24 hour/day format
    TwentyFourHour = 0,
    ///1: AM/PM hour format
    AmPm = 1,
}
impl From<FMT> for bool {
    #[inline(always)]
    fn from(variant: FMT) -> Self {
        variant as u8 != 0
    }
}
///Field `FMT` reader - Hour format
pub type FMT_R = crate::BitReader<FMT>;
impl FMT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMT {
        match self.bits {
            false => FMT::TwentyFourHour,
            true => FMT::AmPm,
        }
    }
    ///24 hour/day format
    #[inline(always)]
    pub fn is_twenty_four_hour(&self) -> bool {
        *self == FMT::TwentyFourHour
    }
    ///AM/PM hour format
    #[inline(always)]
    pub fn is_am_pm(&self) -> bool {
        *self == FMT::AmPm
    }
}
///Field `FMT` writer - Hour format
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG, FMT>;
impl<'a, REG> FMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///24 hour/day format
    #[inline(always)]
    pub fn twenty_four_hour(self) -> &'a mut crate::W<REG> {
        self.variant(FMT::TwentyFourHour)
    }
    ///AM/PM hour format
    #[inline(always)]
    pub fn am_pm(self) -> &'a mut crate::W<REG> {
        self.variant(FMT::AmPm)
    }
}
/**Alarm A enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAE {
    ///0: Alarm disabled
    Disabled = 0,
    ///1: Alarm enabled
    Enabled = 1,
}
impl From<ALRAE> for bool {
    #[inline(always)]
    fn from(variant: ALRAE) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAE` reader - Alarm A enable
pub type ALRAE_R = crate::BitReader<ALRAE>;
impl ALRAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRAE {
        match self.bits {
            false => ALRAE::Disabled,
            true => ALRAE::Enabled,
        }
    }
    ///Alarm disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAE::Disabled
    }
    ///Alarm enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAE::Enabled
    }
}
///Field `ALRAE` writer - Alarm A enable
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAE>;
impl<'a, REG> ALRAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE::Disabled)
    }
    ///Alarm enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE::Enabled)
    }
}
/**timestamp enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSE {
    ///0: Timestamp disabled
    Disabled = 0,
    ///1: Timestamp enabled
    Enabled = 1,
}
impl From<TSE> for bool {
    #[inline(always)]
    fn from(variant: TSE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSE` reader - timestamp enable
pub type TSE_R = crate::BitReader<TSE>;
impl TSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSE {
        match self.bits {
            false => TSE::Disabled,
            true => TSE::Enabled,
        }
    }
    ///Timestamp disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSE::Disabled
    }
    ///Timestamp enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSE::Enabled
    }
}
///Field `TSE` writer - timestamp enable
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG, TSE>;
impl<'a, REG> TSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timestamp disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSE::Disabled)
    }
    ///Timestamp enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSE::Enabled)
    }
}
/**Alarm A interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAIE {
    ///0: Alarm Interrupt disabled
    Disabled = 0,
    ///1: Alarm Interrupt enabled
    Enabled = 1,
}
impl From<ALRAIE> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAIE` reader - Alarm A interrupt enable
pub type ALRAIE_R = crate::BitReader<ALRAIE>;
impl ALRAIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRAIE {
        match self.bits {
            false => ALRAIE::Disabled,
            true => ALRAIE::Enabled,
        }
    }
    ///Alarm Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAIE::Disabled
    }
    ///Alarm Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAIE::Enabled
    }
}
///Field `ALRAIE` writer - Alarm A interrupt enable
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAIE>;
impl<'a, REG> ALRAIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE::Disabled)
    }
    ///Alarm Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE::Enabled)
    }
}
/**Time-stamp interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIE {
    ///0: Time-stamp Interrupt disabled
    Disabled = 0,
    ///1: Time-stamp Interrupt enabled
    Enabled = 1,
}
impl From<TSIE> for bool {
    #[inline(always)]
    fn from(variant: TSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSIE` reader - Time-stamp interrupt enable
pub type TSIE_R = crate::BitReader<TSIE>;
impl TSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSIE {
        match self.bits {
            false => TSIE::Disabled,
            true => TSIE::Enabled,
        }
    }
    ///Time-stamp Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSIE::Disabled
    }
    ///Time-stamp Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSIE::Enabled
    }
}
///Field `TSIE` writer - Time-stamp interrupt enable
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG, TSIE>;
impl<'a, REG> TSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Time-stamp Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE::Disabled)
    }
    ///Time-stamp Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE::Enabled)
    }
}
/**Add 1 hour (summer time change)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1HW {
    ///1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode
    Add1 = 1,
}
impl From<ADD1HW> for bool {
    #[inline(always)]
    fn from(variant: ADD1HW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADD1H` writer - Add 1 hour (summer time change)
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG, ADD1HW>;
impl<'a, REG> ADD1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Adds 1 hour to the current time. This can be used for summer time change outside initialization mode
    #[inline(always)]
    pub fn add1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1HW::Add1)
    }
}
/**Subtract 1 hour (winter time change)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUB1HW {
    ///1: Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode
    Sub1 = 1,
}
impl From<SUB1HW> for bool {
    #[inline(always)]
    fn from(variant: SUB1HW) -> Self {
        variant as u8 != 0
    }
}
///Field `SUB1H` writer - Subtract 1 hour (winter time change)
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG, SUB1HW>;
impl<'a, REG> SUB1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode
    #[inline(always)]
    pub fn sub1(self) -> &'a mut crate::W<REG> {
        self.variant(SUB1HW::Sub1)
    }
}
/**Backup

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP {
    ///0: Daylight Saving Time change has not been performed
    DstNotChanged = 0,
    ///1: Daylight Saving Time change has been performed
    DstChanged = 1,
}
impl From<BKP> for bool {
    #[inline(always)]
    fn from(variant: BKP) -> Self {
        variant as u8 != 0
    }
}
///Field `BKP` reader - Backup
pub type BKP_R = crate::BitReader<BKP>;
impl BKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKP {
        match self.bits {
            false => BKP::DstNotChanged,
            true => BKP::DstChanged,
        }
    }
    ///Daylight Saving Time change has not been performed
    #[inline(always)]
    pub fn is_dst_not_changed(&self) -> bool {
        *self == BKP::DstNotChanged
    }
    ///Daylight Saving Time change has been performed
    #[inline(always)]
    pub fn is_dst_changed(&self) -> bool {
        *self == BKP::DstChanged
    }
}
///Field `BKP` writer - Backup
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Daylight Saving Time change has not been performed
    #[inline(always)]
    pub fn dst_not_changed(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::DstNotChanged)
    }
    ///Daylight Saving Time change has been performed
    #[inline(always)]
    pub fn dst_changed(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::DstChanged)
    }
}
///Field `COSEL` reader - Calibration output selection
pub type COSEL_R = crate::BitReader;
///Field `COSEL` writer - Calibration output selection
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Output polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL {
    ///0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    High = 0,
    ///1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    Low = 1,
}
impl From<POL> for bool {
    #[inline(always)]
    fn from(variant: POL) -> Self {
        variant as u8 != 0
    }
}
///Field `POL` reader - Output polarity
pub type POL_R = crate::BitReader<POL>;
impl POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> POL {
        match self.bits {
            false => POL::High,
            true => POL::Low,
        }
    }
    ///The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL::High
    }
    ///The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL::Low
    }
}
///Field `POL` writer - Output polarity
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG, POL>;
impl<'a, REG> POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(POL::High)
    }
    ///The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(POL::Low)
    }
}
/**Output selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSEL {
    ///0: Output disabled
    Disabled = 0,
    ///1: Alarm A output enabled
    AlarmA = 1,
    ///2: Alarm B output enabled
    AlarmB = 2,
    ///3: Wakeup output enabled
    Wakeup = 3,
}
impl From<OSEL> for u8 {
    #[inline(always)]
    fn from(variant: OSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSEL {
    type Ux = u8;
}
impl crate::IsEnum for OSEL {}
///Field `OSEL` reader - Output selection
pub type OSEL_R = crate::FieldReader<OSEL>;
impl OSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSEL {
        match self.bits {
            0 => OSEL::Disabled,
            1 => OSEL::AlarmA,
            2 => OSEL::AlarmB,
            3 => OSEL::Wakeup,
            _ => unreachable!(),
        }
    }
    ///Output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSEL::Disabled
    }
    ///Alarm A output enabled
    #[inline(always)]
    pub fn is_alarm_a(&self) -> bool {
        *self == OSEL::AlarmA
    }
    ///Alarm B output enabled
    #[inline(always)]
    pub fn is_alarm_b(&self) -> bool {
        *self == OSEL::AlarmB
    }
    ///Wakeup output enabled
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == OSEL::Wakeup
    }
}
///Field `OSEL` writer - Output selection
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSEL, crate::Safe>;
impl<'a, REG> OSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::Disabled)
    }
    ///Alarm A output enabled
    #[inline(always)]
    pub fn alarm_a(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::AlarmA)
    }
    ///Alarm B output enabled
    #[inline(always)]
    pub fn alarm_b(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::AlarmB)
    }
    ///Wakeup output enabled
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::Wakeup)
    }
}
/**Calibration output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COE {
    ///0: Calibration output disabled
    Disabled = 0,
    ///1: Calibration output enabled
    Enabled = 1,
}
impl From<COE> for bool {
    #[inline(always)]
    fn from(variant: COE) -> Self {
        variant as u8 != 0
    }
}
///Field `COE` reader - Calibration output enable
pub type COE_R = crate::BitReader<COE>;
impl COE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COE {
        match self.bits {
            false => COE::Disabled,
            true => COE::Enabled,
        }
    }
    ///Calibration output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COE::Disabled
    }
    ///Calibration output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COE::Enabled
    }
}
///Field `COE` writer - Calibration output enable
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG, COE>;
impl<'a, REG> COE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COE::Disabled)
    }
    ///Calibration output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COE::Enabled)
    }
}
impl R {
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("tsedge", &self.tsedge())
            .field("refckon", &self.refckon())
            .field("bypshad", &self.bypshad())
            .field("fmt", &self.fmt())
            .field("alrae", &self.alrae())
            .field("tse", &self.tse())
            .field("alraie", &self.alraie())
            .field("tsie", &self.tsie())
            .field("bkp", &self.bkp())
            .field("cosel", &self.cosel())
            .field("pol", &self.pol())
            .field("osel", &self.osel())
            .field("coe", &self.coe())
            .finish()
    }
}
impl W {
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W<'_, CRrs> {
        TSEDGE_W::new(self, 3)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W<'_, CRrs> {
        REFCKON_W::new(self, 4)
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W<'_, CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W<'_, CRrs> {
        FMT_W::new(self, 6)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W<'_, CRrs> {
        ALRAE_W::new(self, 8)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, CRrs> {
        TSE_W::new(self, 11)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W<'_, CRrs> {
        ALRAIE_W::new(self, 12)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<'_, CRrs> {
        TSIE_W::new(self, 15)
    }
    ///Bit 16 - Add 1 hour (summer time change)
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W<'_, CRrs> {
        ADD1H_W::new(self, 16)
    }
    ///Bit 17 - Subtract 1 hour (winter time change)
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W<'_, CRrs> {
        SUB1H_W::new(self, 17)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, CRrs> {
        BKP_W::new(self, 18)
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W<'_, CRrs> {
        COSEL_W::new(self, 19)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<'_, CRrs> {
        POL_W::new(self, 20)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W<'_, CRrs> {
        OSEL_W::new(self, 21)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W<'_, CRrs> {
        COE_W::new(self, 23)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x1.html#RTC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
