#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "ADC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENR {
    #[doc = "0: ADC disabled"]
    Disabled = 0,
    #[doc = "1: ADC enabled"]
    Enabled = 1,
}
impl From<ADENR> for bool {
    #[inline(always)]
    fn from(variant: ADENR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` reader - ADC enable"]
pub type ADEN_R = crate::BitReader<ADENR>;
impl ADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADENR {
        match self.bits {
            false => ADENR::Disabled,
            true => ADENR::Enabled,
        }
    }
    #[doc = "ADC disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADENR::Disabled
    }
    #[doc = "ADC enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADENR::Enabled
    }
}
#[doc = "ADC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENW {
    #[doc = "1: Enable the ADC"]
    Enabled = 1,
}
impl From<ADENW> for bool {
    #[inline(always)]
    fn from(variant: ADENW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` writer - ADC enable"]
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG, ADENW>;
impl<'a, REG> ADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the ADC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADENW::Enabled)
    }
}
#[doc = "ADC disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISR {
    #[doc = "0: No disable command active"]
    NotDisabling = 0,
    #[doc = "1: ADC disabling"]
    Disabling = 1,
}
impl From<ADDISR> for bool {
    #[inline(always)]
    fn from(variant: ADDISR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` reader - ADC disable"]
pub type ADDIS_R = crate::BitReader<ADDISR>;
impl ADDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDISR {
        match self.bits {
            false => ADDISR::NotDisabling,
            true => ADDISR::Disabling,
        }
    }
    #[doc = "No disable command active"]
    #[inline(always)]
    pub fn is_not_disabling(&self) -> bool {
        *self == ADDISR::NotDisabling
    }
    #[doc = "ADC disabling"]
    #[inline(always)]
    pub fn is_disabling(&self) -> bool {
        *self == ADDISR::Disabling
    }
}
#[doc = "ADC disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISW {
    #[doc = "1: Disable the ADC"]
    Disable = 1,
}
impl From<ADDISW> for bool {
    #[inline(always)]
    fn from(variant: ADDISW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` writer - ADC disable"]
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG, ADDISW>;
impl<'a, REG> ADDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the ADC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADDISW::Disable)
    }
}
#[doc = "ADC group regular conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTR {
    #[doc = "0: No conversion ongoing"]
    NotActive = 0,
    #[doc = "1: ADC operating and may be converting"]
    Active = 1,
}
impl From<ADSTARTR> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` reader - ADC group regular conversion start"]
pub type ADSTART_R = crate::BitReader<ADSTARTR>;
impl ADSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADSTARTR {
        match self.bits {
            false => ADSTARTR::NotActive,
            true => ADSTARTR::Active,
        }
    }
    #[doc = "No conversion ongoing"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ADSTARTR::NotActive
    }
    #[doc = "ADC operating and may be converting"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ADSTARTR::Active
    }
}
#[doc = "ADC group regular conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTW {
    #[doc = "1: Start the ADC conversion (may be delayed for hardware triggers)"]
    StartConversion = 1,
}
impl From<ADSTARTW> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` writer - ADC group regular conversion start"]
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG, ADSTARTW>;
impl<'a, REG> ADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start the ADC conversion (may be delayed for hardware triggers)"]
    #[inline(always)]
    pub fn start_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTARTW::StartConversion)
    }
}
#[doc = "Field `JADSTART` reader - ADC group injected conversion start"]
pub use ADSTART_R as JADSTART_R;
#[doc = "Field `JADSTART` writer - ADC group injected conversion start"]
pub use ADSTART_W as JADSTART_W;
#[doc = "ADC group regular conversion stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPR {
    #[doc = "0: No stop command active"]
    NotStopping = 0,
    #[doc = "1: ADC stopping conversion"]
    Stopping = 1,
}
impl From<ADSTPR> for bool {
    #[inline(always)]
    fn from(variant: ADSTPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` reader - ADC group regular conversion stop"]
pub type ADSTP_R = crate::BitReader<ADSTPR>;
impl ADSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADSTPR {
        match self.bits {
            false => ADSTPR::NotStopping,
            true => ADSTPR::Stopping,
        }
    }
    #[doc = "No stop command active"]
    #[inline(always)]
    pub fn is_not_stopping(&self) -> bool {
        *self == ADSTPR::NotStopping
    }
    #[doc = "ADC stopping conversion"]
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        *self == ADSTPR::Stopping
    }
}
#[doc = "ADC group regular conversion stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPW {
    #[doc = "1: Stop the active conversion"]
    StopConversion = 1,
}
impl From<ADSTPW> for bool {
    #[inline(always)]
    fn from(variant: ADSTPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` writer - ADC group regular conversion stop"]
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG, ADSTPW>;
impl<'a, REG> ADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop the active conversion"]
    #[inline(always)]
    pub fn stop_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTPW::StopConversion)
    }
}
#[doc = "Field `JADSTP` reader - ADC group injected conversion stop"]
pub use ADSTP_R as JADSTP_R;
#[doc = "Field `JADSTP` writer - ADC group injected conversion stop"]
pub use ADSTP_W as JADSTP_W;
#[doc = "Boost mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOST {
    #[doc = "0: Boost mode off. Used when ADC clock &lt; 20MHz"]
    Off = 0,
    #[doc = "1: Boost mode on. Used when ADC clock > 20MHz"]
    On = 1,
}
impl From<BOOST> for bool {
    #[inline(always)]
    fn from(variant: BOOST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOST` reader - Boost mode control"]
pub type BOOST_R = crate::BitReader<BOOST>;
impl BOOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOOST {
        match self.bits {
            false => BOOST::Off,
            true => BOOST::On,
        }
    }
    #[doc = "Boost mode off. Used when ADC clock &lt; 20MHz"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BOOST::Off
    }
    #[doc = "Boost mode on. Used when ADC clock > 20MHz"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == BOOST::On
    }
}
#[doc = "Field `BOOST` writer - Boost mode control"]
pub type BOOST_W<'a, REG> = crate::BitWriter<'a, REG, BOOST>;
impl<'a, REG> BOOST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Boost mode off. Used when ADC clock &lt; 20MHz"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(BOOST::Off)
    }
    #[doc = "Boost mode on. Used when ADC clock > 20MHz"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(BOOST::On)
    }
}
#[doc = "Linearity calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALLIN {
    #[doc = "0: ADC calibration without linearaity calibration"]
    NoLinearity = 0,
    #[doc = "1: ADC calibration with linearaity calibration"]
    Linearity = 1,
}
impl From<ADCALLIN> for bool {
    #[inline(always)]
    fn from(variant: ADCALLIN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCALLIN` reader - Linearity calibration"]
pub type ADCALLIN_R = crate::BitReader<ADCALLIN>;
impl ADCALLIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCALLIN {
        match self.bits {
            false => ADCALLIN::NoLinearity,
            true => ADCALLIN::Linearity,
        }
    }
    #[doc = "ADC calibration without linearaity calibration"]
    #[inline(always)]
    pub fn is_no_linearity(&self) -> bool {
        *self == ADCALLIN::NoLinearity
    }
    #[doc = "ADC calibration with linearaity calibration"]
    #[inline(always)]
    pub fn is_linearity(&self) -> bool {
        *self == ADCALLIN::Linearity
    }
}
#[doc = "Field `ADCALLIN` writer - Linearity calibration"]
pub type ADCALLIN_W<'a, REG> = crate::BitWriter<'a, REG, ADCALLIN>;
impl<'a, REG> ADCALLIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC calibration without linearaity calibration"]
    #[inline(always)]
    pub fn no_linearity(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALLIN::NoLinearity)
    }
    #[doc = "ADC calibration with linearaity calibration"]
    #[inline(always)]
    pub fn linearity(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALLIN::Linearity)
    }
}
#[doc = "Linearity calibration ready Word 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINCALRDYW1 {
    #[doc = "0: LINCALFACT Word Read"]
    Reset = 0,
    #[doc = "1: LINCALFACT Word Write"]
    Set = 1,
}
impl From<LINCALRDYW1> for bool {
    #[inline(always)]
    fn from(variant: LINCALRDYW1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINCALRDYW1` reader - Linearity calibration ready Word 1"]
pub type LINCALRDYW1_R = crate::BitReader<LINCALRDYW1>;
impl LINCALRDYW1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINCALRDYW1 {
        match self.bits {
            false => LINCALRDYW1::Reset,
            true => LINCALRDYW1::Set,
        }
    }
    #[doc = "LINCALFACT Word Read"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LINCALRDYW1::Reset
    }
    #[doc = "LINCALFACT Word Write"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == LINCALRDYW1::Set
    }
}
#[doc = "Field `LINCALRDYW1` writer - Linearity calibration ready Word 1"]
pub type LINCALRDYW1_W<'a, REG> = crate::BitWriter<'a, REG, LINCALRDYW1>;
impl<'a, REG> LINCALRDYW1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LINCALFACT Word Read"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LINCALRDYW1::Reset)
    }
    #[doc = "LINCALFACT Word Write"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(LINCALRDYW1::Set)
    }
}
#[doc = "Field `LINCALRDYW2` reader - Linearity calibration ready Word 2"]
pub use LINCALRDYW1_R as LINCALRDYW2_R;
#[doc = "Field `LINCALRDYW3` reader - Linearity calibration ready Word 3"]
pub use LINCALRDYW1_R as LINCALRDYW3_R;
#[doc = "Field `LINCALRDYW4` reader - Linearity calibration ready Word 4"]
pub use LINCALRDYW1_R as LINCALRDYW4_R;
#[doc = "Field `LINCALRDYW5` reader - Linearity calibration ready Word 5"]
pub use LINCALRDYW1_R as LINCALRDYW5_R;
#[doc = "Field `LINCALRDYW6` reader - Linearity calibration ready Word 6"]
pub use LINCALRDYW1_R as LINCALRDYW6_R;
#[doc = "Field `LINCALRDYW2` writer - Linearity calibration ready Word 2"]
pub use LINCALRDYW1_W as LINCALRDYW2_W;
#[doc = "Field `LINCALRDYW3` writer - Linearity calibration ready Word 3"]
pub use LINCALRDYW1_W as LINCALRDYW3_W;
#[doc = "Field `LINCALRDYW4` writer - Linearity calibration ready Word 4"]
pub use LINCALRDYW1_W as LINCALRDYW4_W;
#[doc = "Field `LINCALRDYW5` writer - Linearity calibration ready Word 5"]
pub use LINCALRDYW1_W as LINCALRDYW5_W;
#[doc = "Field `LINCALRDYW6` writer - Linearity calibration ready Word 6"]
pub use LINCALRDYW1_W as LINCALRDYW6_W;
#[doc = "ADC voltage regulator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVREGEN {
    #[doc = "0: ADC voltage regulator disabled"]
    Disabled = 0,
    #[doc = "1: ADC voltage regulator enabled"]
    Enabled = 1,
}
impl From<ADVREGEN> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADVREGEN` reader - ADC voltage regulator enable"]
pub type ADVREGEN_R = crate::BitReader<ADVREGEN>;
impl ADVREGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADVREGEN {
        match self.bits {
            false => ADVREGEN::Disabled,
            true => ADVREGEN::Enabled,
        }
    }
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADVREGEN::Disabled
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADVREGEN::Enabled
    }
}
#[doc = "Field `ADVREGEN` writer - ADC voltage regulator enable"]
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG, ADVREGEN>;
impl<'a, REG> ADVREGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Disabled)
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Enabled)
    }
}
#[doc = "ADC deep power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEPPWD {
    #[doc = "0: ADC not in deep power down"]
    PowerUp = 0,
    #[doc = "1: ADC in deep power down"]
    PowerDown = 1,
}
impl From<DEEPPWD> for bool {
    #[inline(always)]
    fn from(variant: DEEPPWD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEEPPWD` reader - ADC deep power down enable"]
pub type DEEPPWD_R = crate::BitReader<DEEPPWD>;
impl DEEPPWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEEPPWD {
        match self.bits {
            false => DEEPPWD::PowerUp,
            true => DEEPPWD::PowerDown,
        }
    }
    #[doc = "ADC not in deep power down"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == DEEPPWD::PowerUp
    }
    #[doc = "ADC in deep power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == DEEPPWD::PowerDown
    }
}
#[doc = "Field `DEEPPWD` writer - ADC deep power down enable"]
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG, DEEPPWD>;
impl<'a, REG> DEEPPWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC not in deep power down"]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD::PowerUp)
    }
    #[doc = "ADC in deep power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD::PowerDown)
    }
}
#[doc = "ADC differential mode for calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALDIF {
    #[doc = "0: Calibration for single-ended mode"]
    SingleEnded = 0,
    #[doc = "1: Calibration for differential mode"]
    Differential = 1,
}
impl From<ADCALDIF> for bool {
    #[inline(always)]
    fn from(variant: ADCALDIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCALDIF` reader - ADC differential mode for calibration"]
pub type ADCALDIF_R = crate::BitReader<ADCALDIF>;
impl ADCALDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCALDIF {
        match self.bits {
            false => ADCALDIF::SingleEnded,
            true => ADCALDIF::Differential,
        }
    }
    #[doc = "Calibration for single-ended mode"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == ADCALDIF::SingleEnded
    }
    #[doc = "Calibration for differential mode"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == ADCALDIF::Differential
    }
}
#[doc = "Field `ADCALDIF` writer - ADC differential mode for calibration"]
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG, ADCALDIF>;
impl<'a, REG> ADCALDIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration for single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF::SingleEnded)
    }
    #[doc = "Calibration for differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF::Differential)
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCAL {
    #[doc = "0: Calibration complete"]
    Complete = 0,
    #[doc = "1: Start the calibration of the ADC"]
    Calibration = 1,
}
impl From<ADCAL> for bool {
    #[inline(always)]
    fn from(variant: ADCAL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` reader - ADC calibration"]
pub type ADCAL_R = crate::BitReader<ADCAL>;
impl ADCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCAL {
        match self.bits {
            false => ADCAL::Complete,
            true => ADCAL::Calibration,
        }
    }
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == ADCAL::Complete
    }
    #[doc = "Start the calibration of the ADC"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == ADCAL::Calibration
    }
}
#[doc = "Field `ADCAL` writer - ADC calibration"]
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG, ADCAL>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(ADCAL::Complete)
    }
    #[doc = "Start the calibration of the ADC"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut crate::W<REG> {
        self.variant(ADCAL::Calibration)
    }
}
impl R {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Boost mode control"]
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Linearity calibration"]
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 22 - Linearity calibration ready Word 1"]
    #[inline(always)]
    pub fn lincalrdyw1(&self) -> LINCALRDYW1_R {
        LINCALRDYW1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Linearity calibration ready Word 2"]
    #[inline(always)]
    pub fn lincalrdyw2(&self) -> LINCALRDYW2_R {
        LINCALRDYW2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Linearity calibration ready Word 3"]
    #[inline(always)]
    pub fn lincalrdyw3(&self) -> LINCALRDYW3_R {
        LINCALRDYW3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Linearity calibration ready Word 4"]
    #[inline(always)]
    pub fn lincalrdyw4(&self) -> LINCALRDYW4_R {
        LINCALRDYW4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Linearity calibration ready Word 5"]
    #[inline(always)]
    pub fn lincalrdyw5(&self) -> LINCALRDYW5_R {
        LINCALRDYW5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Linearity calibration ready Word 6"]
    #[inline(always)]
    pub fn lincalrdyw6(&self) -> LINCALRDYW6_R {
        LINCALRDYW6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<CRrs> {
        ADEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<CRrs> {
        ADDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<CRrs> {
        ADSTART_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JADSTART_W<CRrs> {
        JADSTART_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<CRrs> {
        ADSTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JADSTP_W<CRrs> {
        JADSTP_W::new(self, 5)
    }
    #[doc = "Bit 8 - Boost mode control"]
    #[inline(always)]
    #[must_use]
    pub fn boost(&mut self) -> BOOST_W<CRrs> {
        BOOST_W::new(self, 8)
    }
    #[doc = "Bit 16 - Linearity calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcallin(&mut self) -> ADCALLIN_W<CRrs> {
        ADCALLIN_W::new(self, 16)
    }
    #[doc = "Bit 22 - Linearity calibration ready Word 1"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw1(&mut self) -> LINCALRDYW1_W<CRrs> {
        LINCALRDYW1_W::new(self, 22)
    }
    #[doc = "Bit 23 - Linearity calibration ready Word 2"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw2(&mut self) -> LINCALRDYW2_W<CRrs> {
        LINCALRDYW2_W::new(self, 23)
    }
    #[doc = "Bit 24 - Linearity calibration ready Word 3"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw3(&mut self) -> LINCALRDYW3_W<CRrs> {
        LINCALRDYW3_W::new(self, 24)
    }
    #[doc = "Bit 25 - Linearity calibration ready Word 4"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw4(&mut self) -> LINCALRDYW4_W<CRrs> {
        LINCALRDYW4_W::new(self, 25)
    }
    #[doc = "Bit 26 - Linearity calibration ready Word 5"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw5(&mut self) -> LINCALRDYW5_W<CRrs> {
        LINCALRDYW5_W::new(self, 26)
    }
    #[doc = "Bit 27 - Linearity calibration ready Word 6"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw6(&mut self) -> LINCALRDYW6_W<CRrs> {
        LINCALRDYW6_W::new(self, 27)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<CRrs> {
        DEEPPWD_W::new(self, 29)
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<CRrs> {
        ADCALDIF_W::new(self, 30)
    }
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<CRrs> {
        ADCAL_W::new(self, 31)
    }
}
#[doc = "ADC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
