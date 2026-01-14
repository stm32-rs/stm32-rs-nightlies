///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**ADC enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENR {
    ///0: ADC disabled
    Disabled = 0,
    ///1: ADC enabled
    Enabled = 1,
}
impl From<ADENR> for bool {
    #[inline(always)]
    fn from(variant: ADENR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEN` reader - ADC enable
pub type ADEN_R = crate::BitReader<ADENR>;
impl ADEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADENR {
        match self.bits {
            false => ADENR::Disabled,
            true => ADENR::Enabled,
        }
    }
    ///ADC disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADENR::Disabled
    }
    ///ADC enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADENR::Enabled
    }
}
/**ADC enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENW {
    ///1: Enable the ADC
    Enabled = 1,
}
impl From<ADENW> for bool {
    #[inline(always)]
    fn from(variant: ADENW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEN` writer - ADC enable
pub type ADEN_W<'a, REG> = crate::BitWriter1S<'a, REG, ADENW>;
impl<'a, REG> ADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable the ADC
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADENW::Enabled)
    }
}
/**ADC disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISR {
    ///0: No disable command active
    NotDisabling = 0,
    ///1: ADC disabling
    Disabling = 1,
}
impl From<ADDISR> for bool {
    #[inline(always)]
    fn from(variant: ADDISR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDIS` reader - ADC disable
pub type ADDIS_R = crate::BitReader<ADDISR>;
impl ADDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDISR {
        match self.bits {
            false => ADDISR::NotDisabling,
            true => ADDISR::Disabling,
        }
    }
    ///No disable command active
    #[inline(always)]
    pub fn is_not_disabling(&self) -> bool {
        *self == ADDISR::NotDisabling
    }
    ///ADC disabling
    #[inline(always)]
    pub fn is_disabling(&self) -> bool {
        *self == ADDISR::Disabling
    }
}
/**ADC disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISW {
    ///1: Disable the ADC
    Disable = 1,
}
impl From<ADDISW> for bool {
    #[inline(always)]
    fn from(variant: ADDISW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDIS` writer - ADC disable
pub type ADDIS_W<'a, REG> = crate::BitWriter1S<'a, REG, ADDISW>;
impl<'a, REG> ADDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the ADC
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADDISW::Disable)
    }
}
/**ADC group regular conversion start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTR {
    ///0: No conversion ongoing
    NotActive = 0,
    ///1: ADC operating and may be converting
    Active = 1,
}
impl From<ADSTARTR> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTART` reader - ADC group regular conversion start
pub type ADSTART_R = crate::BitReader<ADSTARTR>;
impl ADSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTARTR {
        match self.bits {
            false => ADSTARTR::NotActive,
            true => ADSTARTR::Active,
        }
    }
    ///No conversion ongoing
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ADSTARTR::NotActive
    }
    ///ADC operating and may be converting
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ADSTARTR::Active
    }
}
/**ADC group regular conversion start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTW {
    ///1: Start the ADC conversion (may be delayed for hardware triggers)
    StartConversion = 1,
}
impl From<ADSTARTW> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTART` writer - ADC group regular conversion start
pub type ADSTART_W<'a, REG> = crate::BitWriter1S<'a, REG, ADSTARTW>;
impl<'a, REG> ADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start the ADC conversion (may be delayed for hardware triggers)
    #[inline(always)]
    pub fn start_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTARTW::StartConversion)
    }
}
///Field `JADSTART` reader - ADC group injected conversion start
pub use ADSTART_R as JADSTART_R;
///Field `JADSTART` writer - ADC group injected conversion start
pub use ADSTART_W as JADSTART_W;
/**ADC group regular conversion stop

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPR {
    ///0: No stop command active
    NotStopping = 0,
    ///1: ADC stopping conversion
    Stopping = 1,
}
impl From<ADSTPR> for bool {
    #[inline(always)]
    fn from(variant: ADSTPR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTP` reader - ADC group regular conversion stop
pub type ADSTP_R = crate::BitReader<ADSTPR>;
impl ADSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTPR {
        match self.bits {
            false => ADSTPR::NotStopping,
            true => ADSTPR::Stopping,
        }
    }
    ///No stop command active
    #[inline(always)]
    pub fn is_not_stopping(&self) -> bool {
        *self == ADSTPR::NotStopping
    }
    ///ADC stopping conversion
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        *self == ADSTPR::Stopping
    }
}
/**ADC group regular conversion stop

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPW {
    ///1: Stop the active conversion
    StopConversion = 1,
}
impl From<ADSTPW> for bool {
    #[inline(always)]
    fn from(variant: ADSTPW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTP` writer - ADC group regular conversion stop
pub type ADSTP_W<'a, REG> = crate::BitWriter1S<'a, REG, ADSTPW>;
impl<'a, REG> ADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop the active conversion
    #[inline(always)]
    pub fn stop_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTPW::StopConversion)
    }
}
///Field `JADSTP` reader - ADC group injected conversion stop
pub use ADSTP_R as JADSTP_R;
///Field `JADSTP` writer - ADC group injected conversion stop
pub use ADSTP_W as JADSTP_W;
/**Boost mode control

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOST {
    ///0: Boost mode used when ADC clock ≤ 6.25 MHz
    Lt6_25 = 0,
    ///1: Boost mode used when 6.25 MHz < ADC clock ≤ 12.5 MHz
    Lt12_5 = 1,
    ///2: Boost mode used when 12.5 MHz < ADC clock ≤ 25.0 MHz
    Lt25 = 2,
    ///3: Boost mode used when 25.0 MHz < ADC clock ≤ 50.0 MHz
    Lt50 = 3,
}
impl From<BOOST> for u8 {
    #[inline(always)]
    fn from(variant: BOOST) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOOST {
    type Ux = u8;
}
impl crate::IsEnum for BOOST {}
///Field `BOOST` reader - Boost mode control
pub type BOOST_R = crate::FieldReader<BOOST>;
impl BOOST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOOST {
        match self.bits {
            0 => BOOST::Lt6_25,
            1 => BOOST::Lt12_5,
            2 => BOOST::Lt25,
            3 => BOOST::Lt50,
            _ => unreachable!(),
        }
    }
    ///Boost mode used when ADC clock ≤ 6.25 MHz
    #[inline(always)]
    pub fn is_lt6_25(&self) -> bool {
        *self == BOOST::Lt6_25
    }
    ///Boost mode used when 6.25 MHz < ADC clock ≤ 12.5 MHz
    #[inline(always)]
    pub fn is_lt12_5(&self) -> bool {
        *self == BOOST::Lt12_5
    }
    ///Boost mode used when 12.5 MHz < ADC clock ≤ 25.0 MHz
    #[inline(always)]
    pub fn is_lt25(&self) -> bool {
        *self == BOOST::Lt25
    }
    ///Boost mode used when 25.0 MHz < ADC clock ≤ 50.0 MHz
    #[inline(always)]
    pub fn is_lt50(&self) -> bool {
        *self == BOOST::Lt50
    }
}
///Field `BOOST` writer - Boost mode control
pub type BOOST_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BOOST, crate::Safe>;
impl<'a, REG> BOOST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Boost mode used when ADC clock ≤ 6.25 MHz
    #[inline(always)]
    pub fn lt6_25(self) -> &'a mut crate::W<REG> {
        self.variant(BOOST::Lt6_25)
    }
    ///Boost mode used when 6.25 MHz < ADC clock ≤ 12.5 MHz
    #[inline(always)]
    pub fn lt12_5(self) -> &'a mut crate::W<REG> {
        self.variant(BOOST::Lt12_5)
    }
    ///Boost mode used when 12.5 MHz < ADC clock ≤ 25.0 MHz
    #[inline(always)]
    pub fn lt25(self) -> &'a mut crate::W<REG> {
        self.variant(BOOST::Lt25)
    }
    ///Boost mode used when 25.0 MHz < ADC clock ≤ 50.0 MHz
    #[inline(always)]
    pub fn lt50(self) -> &'a mut crate::W<REG> {
        self.variant(BOOST::Lt50)
    }
}
/**Linearity calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALLIN {
    ///0: ADC calibration without linearaity calibration
    NoLinearity = 0,
    ///1: ADC calibration with linearaity calibration
    Linearity = 1,
}
impl From<ADCALLIN> for bool {
    #[inline(always)]
    fn from(variant: ADCALLIN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCALLIN` reader - Linearity calibration
pub type ADCALLIN_R = crate::BitReader<ADCALLIN>;
impl ADCALLIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCALLIN {
        match self.bits {
            false => ADCALLIN::NoLinearity,
            true => ADCALLIN::Linearity,
        }
    }
    ///ADC calibration without linearaity calibration
    #[inline(always)]
    pub fn is_no_linearity(&self) -> bool {
        *self == ADCALLIN::NoLinearity
    }
    ///ADC calibration with linearaity calibration
    #[inline(always)]
    pub fn is_linearity(&self) -> bool {
        *self == ADCALLIN::Linearity
    }
}
///Field `ADCALLIN` writer - Linearity calibration
pub type ADCALLIN_W<'a, REG> = crate::BitWriter<'a, REG, ADCALLIN>;
impl<'a, REG> ADCALLIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC calibration without linearaity calibration
    #[inline(always)]
    pub fn no_linearity(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALLIN::NoLinearity)
    }
    ///ADC calibration with linearaity calibration
    #[inline(always)]
    pub fn linearity(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALLIN::Linearity)
    }
}
/**Linearity calibration ready Word %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINCALRDYW1 {
    ///0: LINCALFACT Word Read
    Reset = 0,
    ///1: LINCALFACT Word Write
    Set = 1,
}
impl From<LINCALRDYW1> for bool {
    #[inline(always)]
    fn from(variant: LINCALRDYW1) -> Self {
        variant as u8 != 0
    }
}
///Field `LINCALRDYW(1-6)` reader - Linearity calibration ready Word %s
pub type LINCALRDYW_R = crate::BitReader<LINCALRDYW1>;
impl LINCALRDYW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LINCALRDYW1 {
        match self.bits {
            false => LINCALRDYW1::Reset,
            true => LINCALRDYW1::Set,
        }
    }
    ///LINCALFACT Word Read
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LINCALRDYW1::Reset
    }
    ///LINCALFACT Word Write
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == LINCALRDYW1::Set
    }
}
///Field `LINCALRDYW(1-6)` writer - Linearity calibration ready Word %s
pub type LINCALRDYW_W<'a, REG> = crate::BitWriter<'a, REG, LINCALRDYW1>;
impl<'a, REG> LINCALRDYW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LINCALFACT Word Read
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LINCALRDYW1::Reset)
    }
    ///LINCALFACT Word Write
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(LINCALRDYW1::Set)
    }
}
/**ADC voltage regulator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVREGEN {
    ///0: ADC voltage regulator disabled
    Disabled = 0,
    ///1: ADC voltage regulator enabled
    Enabled = 1,
}
impl From<ADVREGEN> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADVREGEN` reader - ADC voltage regulator enable
pub type ADVREGEN_R = crate::BitReader<ADVREGEN>;
impl ADVREGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADVREGEN {
        match self.bits {
            false => ADVREGEN::Disabled,
            true => ADVREGEN::Enabled,
        }
    }
    ///ADC voltage regulator disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADVREGEN::Disabled
    }
    ///ADC voltage regulator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADVREGEN::Enabled
    }
}
///Field `ADVREGEN` writer - ADC voltage regulator enable
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG, ADVREGEN>;
impl<'a, REG> ADVREGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC voltage regulator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Disabled)
    }
    ///ADC voltage regulator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Enabled)
    }
}
/**ADC deep power down enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEPPWD {
    ///0: ADC not in deep power down
    PowerUp = 0,
    ///1: ADC in deep power down
    PowerDown = 1,
}
impl From<DEEPPWD> for bool {
    #[inline(always)]
    fn from(variant: DEEPPWD) -> Self {
        variant as u8 != 0
    }
}
///Field `DEEPPWD` reader - ADC deep power down enable
pub type DEEPPWD_R = crate::BitReader<DEEPPWD>;
impl DEEPPWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEEPPWD {
        match self.bits {
            false => DEEPPWD::PowerUp,
            true => DEEPPWD::PowerDown,
        }
    }
    ///ADC not in deep power down
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == DEEPPWD::PowerUp
    }
    ///ADC in deep power down
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == DEEPPWD::PowerDown
    }
}
///Field `DEEPPWD` writer - ADC deep power down enable
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG, DEEPPWD>;
impl<'a, REG> DEEPPWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC not in deep power down
    #[inline(always)]
    pub fn power_up(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD::PowerUp)
    }
    ///ADC in deep power down
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD::PowerDown)
    }
}
/**ADC differential mode for calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALDIF {
    ///0: Calibration for single-ended mode
    SingleEnded = 0,
    ///1: Calibration for differential mode
    Differential = 1,
}
impl From<ADCALDIF> for bool {
    #[inline(always)]
    fn from(variant: ADCALDIF) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCALDIF` reader - ADC differential mode for calibration
pub type ADCALDIF_R = crate::BitReader<ADCALDIF>;
impl ADCALDIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCALDIF {
        match self.bits {
            false => ADCALDIF::SingleEnded,
            true => ADCALDIF::Differential,
        }
    }
    ///Calibration for single-ended mode
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == ADCALDIF::SingleEnded
    }
    ///Calibration for differential mode
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == ADCALDIF::Differential
    }
}
///Field `ADCALDIF` writer - ADC differential mode for calibration
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG, ADCALDIF>;
impl<'a, REG> ADCALDIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration for single-ended mode
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF::SingleEnded)
    }
    ///Calibration for differential mode
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF::Differential)
    }
}
/**ADC calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALR {
    ///0: ADC calibration either not yet performed or completed
    NotCalibrating = 0,
    ///1: ADC calibration in progress
    Calibrating = 1,
}
impl From<ADCALR> for bool {
    #[inline(always)]
    fn from(variant: ADCALR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCAL` reader - ADC calibration
pub type ADCAL_R = crate::BitReader<ADCALR>;
impl ADCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCALR {
        match self.bits {
            false => ADCALR::NotCalibrating,
            true => ADCALR::Calibrating,
        }
    }
    ///ADC calibration either not yet performed or completed
    #[inline(always)]
    pub fn is_not_calibrating(&self) -> bool {
        *self == ADCALR::NotCalibrating
    }
    ///ADC calibration in progress
    #[inline(always)]
    pub fn is_calibrating(&self) -> bool {
        *self == ADCALR::Calibrating
    }
}
/**ADC calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALW {
    ///1: Start the ADC calibration sequence
    StartCalibration = 1,
}
impl From<ADCALW> for bool {
    #[inline(always)]
    fn from(variant: ADCALW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCAL` writer - ADC calibration
pub type ADCAL_W<'a, REG> = crate::BitWriter1S<'a, REG, ADCALW>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start the ADC calibration sequence
    #[inline(always)]
    pub fn start_calibration(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALW::StartCalibration)
    }
}
impl R {
    ///Bit 0 - ADC enable
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC disable
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC group regular conversion start
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC group injected conversion start
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC group regular conversion stop
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC group injected conversion stop
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:9 - Boost mode control
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 16 - Linearity calibration
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Linearity calibration ready Word (1-6)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LINCALRDYW1` field.</div>
    #[inline(always)]
    pub fn lincalrdyw(&self, n: u8) -> LINCALRDYW_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        LINCALRDYW_R::new(((self.bits >> (n + 22)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Linearity calibration ready Word (1-6)
    #[inline(always)]
    pub fn lincalrdyw_iter(&self) -> impl Iterator<Item = LINCALRDYW_R> + '_ {
        (0..6).map(move |n| LINCALRDYW_R::new(((self.bits >> (n + 22)) & 1) != 0))
    }
    ///Bit 22 - Linearity calibration ready Word 1
    #[inline(always)]
    pub fn lincalrdyw1(&self) -> LINCALRDYW_R {
        LINCALRDYW_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Linearity calibration ready Word 2
    #[inline(always)]
    pub fn lincalrdyw2(&self) -> LINCALRDYW_R {
        LINCALRDYW_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Linearity calibration ready Word 3
    #[inline(always)]
    pub fn lincalrdyw3(&self) -> LINCALRDYW_R {
        LINCALRDYW_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Linearity calibration ready Word 4
    #[inline(always)]
    pub fn lincalrdyw4(&self) -> LINCALRDYW_R {
        LINCALRDYW_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Linearity calibration ready Word 5
    #[inline(always)]
    pub fn lincalrdyw5(&self) -> LINCALRDYW_R {
        LINCALRDYW_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Linearity calibration ready Word 6
    #[inline(always)]
    pub fn lincalrdyw6(&self) -> LINCALRDYW_R {
        LINCALRDYW_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - ADC voltage regulator enable
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ADC deep power down enable
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ADC differential mode for calibration
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ADC calibration
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("adcal", &self.adcal())
            .field("adcaldif", &self.adcaldif())
            .field("deeppwd", &self.deeppwd())
            .field("advregen", &self.advregen())
            .field("lincalrdyw1", &self.lincalrdyw1())
            .field("lincalrdyw2", &self.lincalrdyw2())
            .field("lincalrdyw3", &self.lincalrdyw3())
            .field("lincalrdyw4", &self.lincalrdyw4())
            .field("lincalrdyw5", &self.lincalrdyw5())
            .field("lincalrdyw6", &self.lincalrdyw6())
            .field("adcallin", &self.adcallin())
            .field("boost", &self.boost())
            .field("adstp", &self.adstp())
            .field("jadstp", &self.jadstp())
            .field("adstart", &self.adstart())
            .field("jadstart", &self.jadstart())
            .field("addis", &self.addis())
            .field("aden", &self.aden())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC enable
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W<'_, CRrs> {
        ADEN_W::new(self, 0)
    }
    ///Bit 1 - ADC disable
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W<'_, CRrs> {
        ADDIS_W::new(self, 1)
    }
    ///Bit 2 - ADC group regular conversion start
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W<'_, CRrs> {
        ADSTART_W::new(self, 2)
    }
    ///Bit 3 - ADC group injected conversion start
    #[inline(always)]
    pub fn jadstart(&mut self) -> JADSTART_W<'_, CRrs> {
        JADSTART_W::new(self, 3)
    }
    ///Bit 4 - ADC group regular conversion stop
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<'_, CRrs> {
        ADSTP_W::new(self, 4)
    }
    ///Bit 5 - ADC group injected conversion stop
    #[inline(always)]
    pub fn jadstp(&mut self) -> JADSTP_W<'_, CRrs> {
        JADSTP_W::new(self, 5)
    }
    ///Bits 8:9 - Boost mode control
    #[inline(always)]
    pub fn boost(&mut self) -> BOOST_W<'_, CRrs> {
        BOOST_W::new(self, 8)
    }
    ///Bit 16 - Linearity calibration
    #[inline(always)]
    pub fn adcallin(&mut self) -> ADCALLIN_W<'_, CRrs> {
        ADCALLIN_W::new(self, 16)
    }
    ///Linearity calibration ready Word (1-6)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LINCALRDYW1` field.</div>
    #[inline(always)]
    pub fn lincalrdyw(&mut self, n: u8) -> LINCALRDYW_W<'_, CRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        LINCALRDYW_W::new(self, n + 22)
    }
    ///Bit 22 - Linearity calibration ready Word 1
    #[inline(always)]
    pub fn lincalrdyw1(&mut self) -> LINCALRDYW_W<'_, CRrs> {
        LINCALRDYW_W::new(self, 22)
    }
    ///Bit 23 - Linearity calibration ready Word 2
    #[inline(always)]
    pub fn lincalrdyw2(&mut self) -> LINCALRDYW_W<'_, CRrs> {
        LINCALRDYW_W::new(self, 23)
    }
    ///Bit 24 - Linearity calibration ready Word 3
    #[inline(always)]
    pub fn lincalrdyw3(&mut self) -> LINCALRDYW_W<'_, CRrs> {
        LINCALRDYW_W::new(self, 24)
    }
    ///Bit 25 - Linearity calibration ready Word 4
    #[inline(always)]
    pub fn lincalrdyw4(&mut self) -> LINCALRDYW_W<'_, CRrs> {
        LINCALRDYW_W::new(self, 25)
    }
    ///Bit 26 - Linearity calibration ready Word 5
    #[inline(always)]
    pub fn lincalrdyw5(&mut self) -> LINCALRDYW_W<'_, CRrs> {
        LINCALRDYW_W::new(self, 26)
    }
    ///Bit 27 - Linearity calibration ready Word 6
    #[inline(always)]
    pub fn lincalrdyw6(&mut self) -> LINCALRDYW_W<'_, CRrs> {
        LINCALRDYW_W::new(self, 27)
    }
    ///Bit 28 - ADC voltage regulator enable
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W<'_, CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    ///Bit 29 - ADC deep power down enable
    #[inline(always)]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<'_, CRrs> {
        DEEPPWD_W::new(self, 29)
    }
    ///Bit 30 - ADC differential mode for calibration
    #[inline(always)]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<'_, CRrs> {
        ADCALDIF_W::new(self, 30)
    }
    ///Bit 31 - ADC calibration
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W<'_, CRrs> {
        ADCAL_W::new(self, 31)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#ADC3:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_003f;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
