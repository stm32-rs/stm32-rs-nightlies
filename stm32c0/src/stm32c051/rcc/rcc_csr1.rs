///Register `RCC_CSR1` reader
pub type R = crate::R<RCC_CSR1rs>;
///Register `RCC_CSR1` writer
pub type W = crate::W<RCC_CSR1rs>;
/**LSE oscillator enable Set and cleared by software to enable LSE oscillator:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<LSEON> for bool {
    #[inline(always)]
    fn from(variant: LSEON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEON` reader - LSE oscillator enable Set and cleared by software to enable LSE oscillator:
pub type LSEON_R = crate::BitReader<LSEON>;
impl LSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEON {
        match self.bits {
            false => LSEON::B0x0,
            true => LSEON::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSEON::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSEON::B0x1
    }
}
///Field `LSEON` writer - LSE oscillator enable Set and cleared by software to enable LSE oscillator:
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG, LSEON>;
impl<'a, REG> LSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::B0x1)
    }
}
/**LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDY {
    ///0: Not ready
    B0x0 = 0,
    ///1: Ready
    B0x1 = 1,
}
impl From<LSERDY> for bool {
    #[inline(always)]
    fn from(variant: LSERDY) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDY` reader - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.
pub type LSERDY_R = crate::BitReader<LSERDY>;
impl LSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSERDY {
        match self.bits {
            false => LSERDY::B0x0,
            true => LSERDY::B0x1,
        }
    }
    ///Not ready
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSERDY::B0x0
    }
    ///Ready
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSERDY::B0x1
    }
}
/**LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP {
    ///0: Not bypassed
    B0x0 = 0,
    ///1: Bypassed
    B0x1 = 1,
}
impl From<LSEBYP> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEBYP` reader - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0).
pub type LSEBYP_R = crate::BitReader<LSEBYP>;
impl LSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEBYP {
        match self.bits {
            false => LSEBYP::B0x0,
            true => LSEBYP::B0x1,
        }
    }
    ///Not bypassed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSEBYP::B0x0
    }
    ///Bypassed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSEBYP::B0x1
    }
}
///Field `LSEBYP` writer - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0).
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, LSEBYP>;
impl<'a, REG> LSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not bypassed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::B0x0)
    }
    ///Bypassed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::B0x1)
    }
}
/**LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEDRV {
    ///0: medium-high driving capability
    B0x0 = 0,
    ///1: high driving capability
    B0x1 = 1,
}
impl From<LSEDRV> for bool {
    #[inline(always)]
    fn from(variant: LSEDRV) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEDRV` reader - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.
pub type LSEDRV_R = crate::BitReader<LSEDRV>;
impl LSEDRV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEDRV {
        match self.bits {
            false => LSEDRV::B0x0,
            true => LSEDRV::B0x1,
        }
    }
    ///medium-high driving capability
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSEDRV::B0x0
    }
    ///high driving capability
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSEDRV::B0x1
    }
}
///Field `LSEDRV` writer - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.
pub type LSEDRV_W<'a, REG> = crate::BitWriter<'a, REG, LSEDRV>;
impl<'a, REG> LSEDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///medium-high driving capability
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::B0x0)
    }
    ///high driving capability
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::B0x1)
    }
}
/**CSS on LSE enable Set by software to enable the clock security system on LSE (32 kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<LSECSSON> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSON` reader - CSS on LSE enable Set by software to enable the clock security system on LSE (32 kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.
pub type LSECSSON_R = crate::BitReader<LSECSSON>;
impl LSECSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSON {
        match self.bits {
            false => LSECSSON::B0x0,
            true => LSECSSON::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSON::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSON::B0x1
    }
}
///Field `LSECSSON` writer - CSS on LSE enable Set by software to enable the clock security system on LSE (32 kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSON>;
impl<'a, REG> LSECSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::B0x1)
    }
}
/**CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 32 kHz oscillator (LSE):

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSD {
    ///0: No failure detected
    B0x0 = 0,
    ///1: Failure detected
    B0x1 = 1,
}
impl From<LSECSSD> for bool {
    #[inline(always)]
    fn from(variant: LSECSSD) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSD` reader - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 32 kHz oscillator (LSE):
pub type LSECSSD_R = crate::BitReader<LSECSSD>;
impl LSECSSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSD {
        match self.bits {
            false => LSECSSD::B0x0,
            true => LSECSSD::B0x1,
        }
    }
    ///No failure detected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSD::B0x0
    }
    ///Failure detected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSD::B0x1
    }
}
/**RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The RTCRST bit can be used to reset this bitfield to 00.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL {
    ///0: No clock
    B0x0 = 0,
    ///1: LSE
    B0x1 = 1,
    ///2: LSI
    B0x2 = 2,
    ///3: HSE divided by 32
    B0x3 = 3,
}
impl From<RTCSEL> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSEL {
    type Ux = u8;
}
impl crate::IsEnum for RTCSEL {}
///Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The RTCRST bit can be used to reset this bitfield to 00.
pub type RTCSEL_R = crate::FieldReader<RTCSEL>;
impl RTCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCSEL {
        match self.bits {
            0 => RTCSEL::B0x0,
            1 => RTCSEL::B0x1,
            2 => RTCSEL::B0x2,
            3 => RTCSEL::B0x3,
            _ => unreachable!(),
        }
    }
    ///No clock
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCSEL::B0x0
    }
    ///LSE
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCSEL::B0x1
    }
    ///LSI
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RTCSEL::B0x2
    }
    ///HSE divided by 32
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RTCSEL::B0x3
    }
}
///Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The RTCRST bit can be used to reset this bitfield to 00.
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RTCSEL, crate::Safe>;
impl<'a, REG> RTCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::B0x0)
    }
    ///LSE
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::B0x1)
    }
    ///LSI
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::B0x2)
    }
    ///HSE divided by 32
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::B0x3)
    }
}
/**RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RTCEN> for bool {
    #[inline(always)]
    fn from(variant: RTCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCEN` reader - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.
pub type RTCEN_R = crate::BitReader<RTCEN>;
impl RTCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCEN {
        match self.bits {
            false => RTCEN::B0x0,
            true => RTCEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCEN::B0x1
    }
}
///Field `RTCEN` writer - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCEN>;
impl<'a, REG> RTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::B0x1)
    }
}
/**RTC domain software reset Set and cleared by software to reset the RTC domain:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCRST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset
    B0x1 = 1,
}
impl From<RTCRST> for bool {
    #[inline(always)]
    fn from(variant: RTCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCRST` reader - RTC domain software reset Set and cleared by software to reset the RTC domain:
pub type RTCRST_R = crate::BitReader<RTCRST>;
impl RTCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCRST {
        match self.bits {
            false => RTCRST::B0x0,
            true => RTCRST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCRST::B0x0
    }
    ///Reset
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCRST::B0x1
    }
}
///Field `RTCRST` writer - RTC domain software reset Set and cleared by software to reset the RTC domain:
pub type RTCRST_W<'a, REG> = crate::BitWriter<'a, REG, RTCRST>;
impl<'a, REG> RTCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCRST::B0x0)
    }
    ///Reset
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCRST::B0x1)
    }
}
/**Low-speed clock output (LSCO) enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<LSCOEN> for bool {
    #[inline(always)]
    fn from(variant: LSCOEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LSCOEN` reader - Low-speed clock output (LSCO) enable Set and cleared by software.
pub type LSCOEN_R = crate::BitReader<LSCOEN>;
impl LSCOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSCOEN {
        match self.bits {
            false => LSCOEN::B0x0,
            true => LSCOEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSCOEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSCOEN::B0x1
    }
}
///Field `LSCOEN` writer - Low-speed clock output (LSCO) enable Set and cleared by software.
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG, LSCOEN>;
impl<'a, REG> LSCOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN::B0x1)
    }
}
/**Low-speed clock output selection Set and cleared by software to select the low-speed output clock:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOSEL {
    ///0: LSI
    B0x0 = 0,
    ///1: LSE
    B0x1 = 1,
}
impl From<LSCOSEL> for bool {
    #[inline(always)]
    fn from(variant: LSCOSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `LSCOSEL` reader - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:
pub type LSCOSEL_R = crate::BitReader<LSCOSEL>;
impl LSCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSCOSEL {
        match self.bits {
            false => LSCOSEL::B0x0,
            true => LSCOSEL::B0x1,
        }
    }
    ///LSI
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSCOSEL::B0x0
    }
    ///LSE
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSCOSEL::B0x1
    }
}
///Field `LSCOSEL` writer - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:
pub type LSCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, LSCOSEL>;
impl<'a, REG> LSCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL::B0x0)
    }
    ///LSE
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL::B0x1)
    }
}
impl R {
    ///Bit 0 - LSE oscillator enable Set and cleared by software to enable LSE oscillator:
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0).
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - CSS on LSE enable Set by software to enable the clock security system on LSE (32 kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 32 kHz oscillator (LSE):
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The RTCRST bit can be used to reset this bitfield to 00.
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 15 - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC domain software reset Set and cleared by software to reset the RTC domain:
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software.
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CSR1")
            .field("lseon", &self.lseon())
            .field("lserdy", &self.lserdy())
            .field("lsebyp", &self.lsebyp())
            .field("lsedrv", &self.lsedrv())
            .field("lsecsson", &self.lsecsson())
            .field("lsecssd", &self.lsecssd())
            .field("rtcsel", &self.rtcsel())
            .field("rtcen", &self.rtcen())
            .field("rtcrst", &self.rtcrst())
            .field("lscoen", &self.lscoen())
            .field("lscosel", &self.lscosel())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSE oscillator enable Set and cleared by software to enable LSE oscillator:
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, RCC_CSR1rs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 2 - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0).
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, RCC_CSR1rs> {
        LSEBYP_W::new(self, 2)
    }
    ///Bit 3 - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<'_, RCC_CSR1rs> {
        LSEDRV_W::new(self, 3)
    }
    ///Bit 5 - CSS on LSE enable Set by software to enable the clock security system on LSE (32 kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<'_, RCC_CSR1rs> {
        LSECSSON_W::new(self, 5)
    }
    ///Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The RTCRST bit can be used to reset this bitfield to 00.
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<'_, RCC_CSR1rs> {
        RTCSEL_W::new(self, 8)
    }
    ///Bit 15 - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<'_, RCC_CSR1rs> {
        RTCEN_W::new(self, 15)
    }
    ///Bit 16 - RTC domain software reset Set and cleared by software to reset the RTC domain:
    #[inline(always)]
    pub fn rtcrst(&mut self) -> RTCRST_W<'_, RCC_CSR1rs> {
        RTCRST_W::new(self, 16)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software.
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W<'_, RCC_CSR1rs> {
        LSCOEN_W::new(self, 24)
    }
    ///Bit 25 - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:
    #[inline(always)]
    pub fn lscosel(&mut self) -> LSCOSEL_W<'_, RCC_CSR1rs> {
        LSCOSEL_W::new(self, 25)
    }
}
/**RCC control/status register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_csr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CSR1)*/
pub struct RCC_CSR1rs;
impl crate::RegisterSpec for RCC_CSR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_csr1::R`](R) reader structure
impl crate::Readable for RCC_CSR1rs {}
///`write(|w| ..)` method takes [`rcc_csr1::W`](W) writer structure
impl crate::Writable for RCC_CSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_CSR1 to value 0
impl crate::Resettable for RCC_CSR1rs {}
