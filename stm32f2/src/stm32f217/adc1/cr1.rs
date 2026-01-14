///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `AWDCH` reader - Analog watchdog channel select bits
pub type AWDCH_R = crate::FieldReader;
///Field `AWDCH` writer - Analog watchdog channel select bits
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Interrupt enable for EOC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE {
    ///0: EOC interrupt disabled
    Disabled = 0,
    ///1: EOC interrupt enabled
    Enabled = 1,
}
impl From<EOCIE> for bool {
    #[inline(always)]
    fn from(variant: EOCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCIE` reader - Interrupt enable for EOC
pub type EOCIE_R = crate::BitReader<EOCIE>;
impl EOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCIE {
        match self.bits {
            false => EOCIE::Disabled,
            true => EOCIE::Enabled,
        }
    }
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE::Disabled
    }
    ///EOC interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE::Enabled
    }
}
///Field `EOCIE` writer - Interrupt enable for EOC
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCIE>;
impl<'a, REG> EOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Disabled)
    }
    ///EOC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Enabled)
    }
}
/**Analog watchdog interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDIE {
    ///0: Analogue watchdog interrupt disabled
    Disabled = 0,
    ///1: Analogue watchdog interrupt enabled
    Enabled = 1,
}
impl From<AWDIE> for bool {
    #[inline(always)]
    fn from(variant: AWDIE) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDIE` reader - Analog watchdog interrupt enable
pub type AWDIE_R = crate::BitReader<AWDIE>;
impl AWDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDIE {
        match self.bits {
            false => AWDIE::Disabled,
            true => AWDIE::Enabled,
        }
    }
    ///Analogue watchdog interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIE::Disabled
    }
    ///Analogue watchdog interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIE::Enabled
    }
}
///Field `AWDIE` writer - Analog watchdog interrupt enable
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG, AWDIE>;
impl<'a, REG> AWDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analogue watchdog interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Disabled)
    }
    ///Analogue watchdog interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Enabled)
    }
}
/**Interrupt enable for injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCIE {
    ///0: JEOC interrupt disabled
    Disabled = 0,
    ///1: JEOC interrupt enabled
    Enabled = 1,
}
impl From<JEOCIE> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOCIE` reader - Interrupt enable for injected channels
pub type JEOCIE_R = crate::BitReader<JEOCIE>;
impl JEOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOCIE {
        match self.bits {
            false => JEOCIE::Disabled,
            true => JEOCIE::Enabled,
        }
    }
    ///JEOC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOCIE::Disabled
    }
    ///JEOC interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOCIE::Enabled
    }
}
///Field `JEOCIE` writer - Interrupt enable for injected channels
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG, JEOCIE>;
impl<'a, REG> JEOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///JEOC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Disabled)
    }
    ///JEOC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Enabled)
    }
}
/**Scan mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCAN {
    ///0: Scan mode disabled
    Disabled = 0,
    ///1: Scan mode enabled
    Enabled = 1,
}
impl From<SCAN> for bool {
    #[inline(always)]
    fn from(variant: SCAN) -> Self {
        variant as u8 != 0
    }
}
///Field `SCAN` reader - Scan mode
pub type SCAN_R = crate::BitReader<SCAN>;
impl SCAN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCAN {
        match self.bits {
            false => SCAN::Disabled,
            true => SCAN::Enabled,
        }
    }
    ///Scan mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCAN::Disabled
    }
    ///Scan mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCAN::Enabled
    }
}
///Field `SCAN` writer - Scan mode
pub type SCAN_W<'a, REG> = crate::BitWriter<'a, REG, SCAN>;
impl<'a, REG> SCAN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Scan mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCAN::Disabled)
    }
    ///Scan mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCAN::Enabled)
    }
}
/**Enable the watchdog on a single channel in scan mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDSGL {
    ///0: Analog watchdog enabled on all channels
    AllChannels = 0,
    ///1: Analog watchdog enabled on a single channel
    SingleChannel = 1,
}
impl From<AWDSGL> for bool {
    #[inline(always)]
    fn from(variant: AWDSGL) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDSGL` reader - Enable the watchdog on a single channel in scan mode
pub type AWDSGL_R = crate::BitReader<AWDSGL>;
impl AWDSGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDSGL {
        match self.bits {
            false => AWDSGL::AllChannels,
            true => AWDSGL::SingleChannel,
        }
    }
    ///Analog watchdog enabled on all channels
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        *self == AWDSGL::AllChannels
    }
    ///Analog watchdog enabled on a single channel
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == AWDSGL::SingleChannel
    }
}
///Field `AWDSGL` writer - Enable the watchdog on a single channel in scan mode
pub type AWDSGL_W<'a, REG> = crate::BitWriter<'a, REG, AWDSGL>;
impl<'a, REG> AWDSGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog enabled on all channels
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut crate::W<REG> {
        self.variant(AWDSGL::AllChannels)
    }
    ///Analog watchdog enabled on a single channel
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut crate::W<REG> {
        self.variant(AWDSGL::SingleChannel)
    }
}
/**Automatic injected group conversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAUTO {
    ///0: Automatic injected group conversion disabled
    Disabled = 0,
    ///1: Automatic injected group conversion enabled
    Enabled = 1,
}
impl From<JAUTO> for bool {
    #[inline(always)]
    fn from(variant: JAUTO) -> Self {
        variant as u8 != 0
    }
}
///Field `JAUTO` reader - Automatic injected group conversion
pub type JAUTO_R = crate::BitReader<JAUTO>;
impl JAUTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JAUTO {
        match self.bits {
            false => JAUTO::Disabled,
            true => JAUTO::Enabled,
        }
    }
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO::Disabled
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO::Enabled
    }
}
///Field `JAUTO` writer - Automatic injected group conversion
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG, JAUTO>;
impl<'a, REG> JAUTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Disabled)
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Enabled)
    }
}
/**Discontinuous mode on regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN {
    ///0: Discontinuous mode on regular channels disabled
    Disabled = 0,
    ///1: Discontinuous mode on regular channels enabled
    Enabled = 1,
}
impl From<DISCEN> for bool {
    #[inline(always)]
    fn from(variant: DISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DISCEN` reader - Discontinuous mode on regular channels
pub type DISCEN_R = crate::BitReader<DISCEN>;
impl DISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DISCEN {
        match self.bits {
            false => DISCEN::Disabled,
            true => DISCEN::Enabled,
        }
    }
    ///Discontinuous mode on regular channels disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN::Disabled
    }
    ///Discontinuous mode on regular channels enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN::Enabled
    }
}
///Field `DISCEN` writer - Discontinuous mode on regular channels
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode on regular channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Disabled)
    }
    ///Discontinuous mode on regular channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Enabled)
    }
}
/**Discontinuous mode on injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDISCEN {
    ///0: Discontinuous mode on injected channels disabled
    Disabled = 0,
    ///1: Discontinuous mode on injected channels enabled
    Enabled = 1,
}
impl From<JDISCEN> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `JDISCEN` reader - Discontinuous mode on injected channels
pub type JDISCEN_R = crate::BitReader<JDISCEN>;
impl JDISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JDISCEN {
        match self.bits {
            false => JDISCEN::Disabled,
            true => JDISCEN::Enabled,
        }
    }
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN::Disabled
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN::Enabled
    }
}
///Field `JDISCEN` writer - Discontinuous mode on injected channels
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG, JDISCEN>;
impl<'a, REG> JDISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Disabled)
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Enabled)
    }
}
///Field `DISCNUM` reader - Discontinuous mode channel count
pub type DISCNUM_R = crate::FieldReader;
///Field `DISCNUM` writer - Discontinuous mode channel count
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**Analog watchdog enable on injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAWDEN {
    ///0: Analog watchdog disabled on injected channels
    Disabled = 0,
    ///1: Analog watchdog enabled on injected channels
    Enabled = 1,
}
impl From<JAWDEN> for bool {
    #[inline(always)]
    fn from(variant: JAWDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `JAWDEN` reader - Analog watchdog enable on injected channels
pub type JAWDEN_R = crate::BitReader<JAWDEN>;
impl JAWDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JAWDEN {
        match self.bits {
            false => JAWDEN::Disabled,
            true => JAWDEN::Enabled,
        }
    }
    ///Analog watchdog disabled on injected channels
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWDEN::Disabled
    }
    ///Analog watchdog enabled on injected channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWDEN::Enabled
    }
}
///Field `JAWDEN` writer - Analog watchdog enable on injected channels
pub type JAWDEN_W<'a, REG> = crate::BitWriter<'a, REG, JAWDEN>;
impl<'a, REG> JAWDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog disabled on injected channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWDEN::Disabled)
    }
    ///Analog watchdog enabled on injected channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWDEN::Enabled)
    }
}
/**Analog watchdog enable on regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDEN {
    ///0: Analog watchdog disabled on regular channels
    Disabled = 0,
    ///1: Analog watchdog enabled on regular channels
    Enabled = 1,
}
impl From<AWDEN> for bool {
    #[inline(always)]
    fn from(variant: AWDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDEN` reader - Analog watchdog enable on regular channels
pub type AWDEN_R = crate::BitReader<AWDEN>;
impl AWDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDEN {
        match self.bits {
            false => AWDEN::Disabled,
            true => AWDEN::Enabled,
        }
    }
    ///Analog watchdog disabled on regular channels
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDEN::Disabled
    }
    ///Analog watchdog enabled on regular channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDEN::Enabled
    }
}
///Field `AWDEN` writer - Analog watchdog enable on regular channels
pub type AWDEN_W<'a, REG> = crate::BitWriter<'a, REG, AWDEN>;
impl<'a, REG> AWDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog disabled on regular channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDEN::Disabled)
    }
    ///Analog watchdog enabled on regular channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDEN::Enabled)
    }
}
/**Resolution

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    ///0: 12-bit (15 ADCCLK cycles)
    TwelveBit = 0,
    ///1: 10-bit (13 ADCCLK cycles)
    TenBit = 1,
    ///2: 8-bit (11 ADCCLK cycles)
    EightBit = 2,
    ///3: 6-bit (9 ADCCLK cycles)
    SixBit = 3,
}
impl From<RES> for u8 {
    #[inline(always)]
    fn from(variant: RES) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES {
    type Ux = u8;
}
impl crate::IsEnum for RES {}
///Field `RES` reader - Resolution
pub type RES_R = crate::FieldReader<RES>;
impl RES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RES {
        match self.bits {
            0 => RES::TwelveBit,
            1 => RES::TenBit,
            2 => RES::EightBit,
            3 => RES::SixBit,
            _ => unreachable!(),
        }
    }
    ///12-bit (15 ADCCLK cycles)
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RES::TwelveBit
    }
    ///10-bit (13 ADCCLK cycles)
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == RES::TenBit
    }
    ///8-bit (11 ADCCLK cycles)
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == RES::EightBit
    }
    ///6-bit (9 ADCCLK cycles)
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == RES::SixBit
    }
}
///Field `RES` writer - Resolution
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RES, crate::Safe>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///12-bit (15 ADCCLK cycles)
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TwelveBit)
    }
    ///10-bit (13 ADCCLK cycles)
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TenBit)
    }
    ///8-bit (11 ADCCLK cycles)
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::EightBit)
    }
    ///6-bit (9 ADCCLK cycles)
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::SixBit)
    }
}
/**Overrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE {
    ///0: Overrun interrupt disabled
    Disabled = 0,
    ///1: Overrun interrupt enabled
    Enabled = 1,
}
impl From<OVRIE> for bool {
    #[inline(always)]
    fn from(variant: OVRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRIE` reader - Overrun interrupt enable
pub type OVRIE_R = crate::BitReader<OVRIE>;
impl OVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRIE {
        match self.bits {
            false => OVRIE::Disabled,
            true => OVRIE::Enabled,
        }
    }
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRIE::Disabled
    }
    ///Overrun interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRIE::Enabled
    }
}
///Field `OVRIE` writer - Overrun interrupt enable
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRIE>;
impl<'a, REG> OVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Disabled)
    }
    ///Overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Enabled)
    }
}
impl R {
    ///Bits 0:4 - Analog watchdog channel select bits
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - Interrupt enable for EOC
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt enable for injected channels
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Scan mode
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable the watchdog on a single channel in scan mode
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Discontinuous mode on regular channels
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 22 - Analog watchdog enable on injected channels
    #[inline(always)]
    pub fn jawden(&self) -> JAWDEN_R {
        JAWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog enable on regular channels
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - Resolution
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("ovrie", &self.ovrie())
            .field("res", &self.res())
            .field("awden", &self.awden())
            .field("jawden", &self.jawden())
            .field("discnum", &self.discnum())
            .field("jdiscen", &self.jdiscen())
            .field("discen", &self.discen())
            .field("jauto", &self.jauto())
            .field("awdsgl", &self.awdsgl())
            .field("scan", &self.scan())
            .field("jeocie", &self.jeocie())
            .field("awdie", &self.awdie())
            .field("eocie", &self.eocie())
            .field("awdch", &self.awdch())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Analog watchdog channel select bits
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W<'_, CR1rs> {
        AWDCH_W::new(self, 0)
    }
    ///Bit 5 - Interrupt enable for EOC
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<'_, CR1rs> {
        EOCIE_W::new(self, 5)
    }
    ///Bit 6 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W<'_, CR1rs> {
        AWDIE_W::new(self, 6)
    }
    ///Bit 7 - Interrupt enable for injected channels
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<'_, CR1rs> {
        JEOCIE_W::new(self, 7)
    }
    ///Bit 8 - Scan mode
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W<'_, CR1rs> {
        SCAN_W::new(self, 8)
    }
    ///Bit 9 - Enable the watchdog on a single channel in scan mode
    #[inline(always)]
    pub fn awdsgl(&mut self) -> AWDSGL_W<'_, CR1rs> {
        AWDSGL_W::new(self, 9)
    }
    ///Bit 10 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<'_, CR1rs> {
        JAUTO_W::new(self, 10)
    }
    ///Bit 11 - Discontinuous mode on regular channels
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CR1rs> {
        DISCEN_W::new(self, 11)
    }
    ///Bit 12 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<'_, CR1rs> {
        JDISCEN_W::new(self, 12)
    }
    ///Bits 13:15 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<'_, CR1rs> {
        DISCNUM_W::new(self, 13)
    }
    ///Bit 22 - Analog watchdog enable on injected channels
    #[inline(always)]
    pub fn jawden(&mut self) -> JAWDEN_W<'_, CR1rs> {
        JAWDEN_W::new(self, 22)
    }
    ///Bit 23 - Analog watchdog enable on regular channels
    #[inline(always)]
    pub fn awden(&mut self) -> AWDEN_W<'_, CR1rs> {
        AWDEN_W::new(self, 23)
    }
    ///Bits 24:25 - Resolution
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, CR1rs> {
        RES_W::new(self, 24)
    }
    ///Bit 26 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<'_, CR1rs> {
        OVRIE_W::new(self, 26)
    }
}
/**control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#ADC1:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
