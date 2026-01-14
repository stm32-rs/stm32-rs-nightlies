///Register `ADC_ISR` reader
pub type R = crate::R<ADC_ISRrs>;
///Register `ADC_ISR` writer
pub type W = crate::W<ADC_ISRrs>;
/**ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDY {
    ///0: ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)
    B0x0 = 0,
    ///1: ADC is ready to start conversion
    B0x1 = 1,
}
impl From<ADRDY> for bool {
    #[inline(always)]
    fn from(variant: ADRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` reader - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
pub type ADRDY_R = crate::BitReader<ADRDY>;
impl ADRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADRDY {
        match self.bits {
            false => ADRDY::B0x0,
            true => ADRDY::B0x1,
        }
    }
    ///ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADRDY::B0x0
    }
    ///ADC is ready to start conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADRDY::B0x1
    }
}
///Field `ADRDY` writer - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG, ADRDY>;
impl<'a, REG> ADRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDY::B0x0)
    }
    ///ADC is ready to start conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDY::B0x1)
    }
}
/**End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to 1 .

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMP {
    ///0: Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)
    B0x0 = 0,
    ///1: End of sampling phase reached
    B0x1 = 1,
}
impl From<EOSMP> for bool {
    #[inline(always)]
    fn from(variant: EOSMP) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` reader - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to 1 .
pub type EOSMP_R = crate::BitReader<EOSMP>;
impl EOSMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSMP {
        match self.bits {
            false => EOSMP::B0x0,
            true => EOSMP::B0x1,
        }
    }
    ///Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOSMP::B0x0
    }
    ///End of sampling phase reached
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOSMP::B0x1
    }
}
///Field `EOSMP` writer - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to 1 .
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG, EOSMP>;
impl<'a, REG> EOSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMP::B0x0)
    }
    ///End of sampling phase reached
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMP::B0x1)
    }
}
/**End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOC {
    ///0: Channel conversion not complete (or the flag event was already acknowledged and cleared by software)
    B0x0 = 0,
    ///1: Channel conversion complete
    B0x1 = 1,
}
impl From<EOC> for bool {
    #[inline(always)]
    fn from(variant: EOC) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` reader - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.
pub type EOC_R = crate::BitReader<EOC>;
impl EOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOC {
        match self.bits {
            false => EOC::B0x0,
            true => EOC::B0x1,
        }
    }
    ///Channel conversion not complete (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOC::B0x0
    }
    ///Channel conversion complete
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOC::B0x1
    }
}
///Field `EOC` writer - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG, EOC>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel conversion not complete (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOC::B0x0)
    }
    ///Channel conversion complete
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOC::B0x1)
    }
}
/**End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOS {
    ///0: Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)
    B0x0 = 0,
    ///1: Conversion sequence complete
    B0x1 = 1,
}
impl From<EOS> for bool {
    #[inline(always)]
    fn from(variant: EOS) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` reader - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.
pub type EOS_R = crate::BitReader<EOS>;
impl EOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOS {
        match self.bits {
            false => EOS::B0x0,
            true => EOS::B0x1,
        }
    }
    ///Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOS::B0x0
    }
    ///Conversion sequence complete
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOS::B0x1
    }
}
///Field `EOS` writer - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG, EOS>;
impl<'a, REG> EOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOS::B0x0)
    }
    ///Conversion sequence complete
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOS::B0x1)
    }
}
/**ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR {
    ///0: No overrun occurred (or the flag event was already acknowledged and cleared by software)
    B0x0 = 0,
    ///1: Overrun has occurred
    B0x1 = 1,
}
impl From<OVR> for bool {
    #[inline(always)]
    fn from(variant: OVR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.
pub type OVR_R = crate::BitReader<OVR>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR {
        match self.bits {
            false => OVR::B0x0,
            true => OVR::B0x1,
        }
    }
    ///No overrun occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVR::B0x0
    }
    ///Overrun has occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVR::B0x1
    }
}
///Field `OVR` writer - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG, OVR>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overrun occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVR::B0x0)
    }
    ///Overrun has occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVR::B0x1)
    }
}
/**Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1 {
    ///0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)
    B0x0 = 0,
    ///1: Analog watchdog event occurred
    B0x1 = 1,
}
impl From<AWD1> for bool {
    #[inline(always)]
    fn from(variant: AWD1) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1` reader - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.
pub type AWD1_R = crate::BitReader<AWD1>;
impl AWD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1 {
        match self.bits {
            false => AWD1::B0x0,
            true => AWD1::B0x1,
        }
    }
    ///No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1::B0x0
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1::B0x1
    }
}
///Field `AWD1` writer - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG, AWD1>;
impl<'a, REG> AWD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1::B0x0)
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1::B0x1)
    }
}
/**Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2 {
    ///0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)
    B0x0 = 0,
    ///1: Analog watchdog event occurred
    B0x1 = 1,
}
impl From<AWD2> for bool {
    #[inline(always)]
    fn from(variant: AWD2) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2` reader - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.
pub type AWD2_R = crate::BitReader<AWD2>;
impl AWD2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2 {
        match self.bits {
            false => AWD2::B0x0,
            true => AWD2::B0x1,
        }
    }
    ///No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2::B0x0
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2::B0x1
    }
}
///Field `AWD2` writer - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.
pub type AWD2_W<'a, REG> = crate::BitWriter<'a, REG, AWD2>;
impl<'a, REG> AWD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2::B0x0)
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2::B0x1)
    }
}
/**Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3 {
    ///0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)
    B0x0 = 0,
    ///1: Analog watchdog event occurred
    B0x1 = 1,
}
impl From<AWD3> for bool {
    #[inline(always)]
    fn from(variant: AWD3) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3` reader - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.
pub type AWD3_R = crate::BitReader<AWD3>;
impl AWD3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3 {
        match self.bits {
            false => AWD3::B0x0,
            true => AWD3::B0x1,
        }
    }
    ///No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3::B0x0
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3::B0x1
    }
}
///Field `AWD3` writer - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.
pub type AWD3_W<'a, REG> = crate::BitWriter<'a, REG, AWD3>;
impl<'a, REG> AWD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3::B0x0)
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3::B0x1)
    }
}
/**End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCAL {
    ///0: Calibration is not complete
    B0x0 = 0,
    ///1: Calibration is complete
    B0x1 = 1,
}
impl From<EOCAL> for bool {
    #[inline(always)]
    fn from(variant: EOCAL) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCAL` reader - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.
pub type EOCAL_R = crate::BitReader<EOCAL>;
impl EOCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCAL {
        match self.bits {
            false => EOCAL::B0x0,
            true => EOCAL::B0x1,
        }
    }
    ///Calibration is not complete
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOCAL::B0x0
    }
    ///Calibration is complete
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOCAL::B0x1
    }
}
///Field `EOCAL` writer - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.
pub type EOCAL_W<'a, REG> = crate::BitWriter<'a, REG, EOCAL>;
impl<'a, REG> EOCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration is not complete
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOCAL::B0x0)
    }
    ///Calibration is complete
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOCAL::B0x1)
    }
}
/**Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRDY {
    ///0: Channel configuration update not applied.
    B0x0 = 0,
    ///1: Channel configuration update is applied.
    B0x1 = 1,
}
impl From<CCRDY> for bool {
    #[inline(always)]
    fn from(variant: CCRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `CCRDY` reader - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.
pub type CCRDY_R = crate::BitReader<CCRDY>;
impl CCRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCRDY {
        match self.bits {
            false => CCRDY::B0x0,
            true => CCRDY::B0x1,
        }
    }
    ///Channel configuration update not applied.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCRDY::B0x0
    }
    ///Channel configuration update is applied.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCRDY::B0x1
    }
}
///Field `CCRDY` writer - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.
pub type CCRDY_W<'a, REG> = crate::BitWriter<'a, REG, CCRDY>;
impl<'a, REG> CCRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel configuration update not applied.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCRDY::B0x0)
    }
    ///Channel configuration update is applied.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCRDY::B0x1)
    }
}
impl R {
    ///Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to 1 .
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.
    #[inline(always)]
    pub fn ccrdy(&self) -> CCRDY_R {
        CCRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_ISR")
            .field("adrdy", &self.adrdy())
            .field("eosmp", &self.eosmp())
            .field("eoc", &self.eoc())
            .field("eos", &self.eos())
            .field("ovr", &self.ovr())
            .field("awd1", &self.awd1())
            .field("awd2", &self.awd2())
            .field("awd3", &self.awd3())
            .field("eocal", &self.eocal())
            .field("ccrdy", &self.ccrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W<'_, ADC_ISRrs> {
        ADRDY_W::new(self, 0)
    }
    ///Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to 1 .
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W<'_, ADC_ISRrs> {
        EOSMP_W::new(self, 1)
    }
    ///Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<'_, ADC_ISRrs> {
        EOC_W::new(self, 2)
    }
    ///Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W<'_, ADC_ISRrs> {
        EOS_W::new(self, 3)
    }
    ///Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<'_, ADC_ISRrs> {
        OVR_W::new(self, 4)
    }
    ///Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD1_W<'_, ADC_ISRrs> {
        AWD1_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD2_W<'_, ADC_ISRrs> {
        AWD2_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD3_W<'_, ADC_ISRrs> {
        AWD3_W::new(self, 9)
    }
    ///Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eocal(&mut self) -> EOCAL_W<'_, ADC_ISRrs> {
        EOCAL_W::new(self, 11)
    }
    ///Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.
    #[inline(always)]
    pub fn ccrdy(&mut self) -> CCRDY_W<'_, ADC_ISRrs> {
        CCRDY_W::new(self, 13)
    }
}
/**ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`adc_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#ADC:ADC_ISR)*/
pub struct ADC_ISRrs;
impl crate::RegisterSpec for ADC_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_isr::R`](R) reader structure
impl crate::Readable for ADC_ISRrs {}
///`write(|w| ..)` method takes [`adc_isr::W`](W) writer structure
impl crate::Writable for ADC_ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_ISR to value 0
impl crate::Resettable for ADC_ISRrs {}
