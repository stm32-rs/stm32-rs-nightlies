///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**I/O analog switch voltage booster enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTEN {
    ///0: I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation
    Disabled = 0,
    ///1: I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation
    Enabled = 1,
}
impl From<BOOSTEN> for bool {
    #[inline(always)]
    fn from(variant: BOOSTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BOOSTEN` reader - I/O analog switch voltage booster enable
pub type BOOSTEN_R = crate::BitReader<BOOSTEN>;
impl BOOSTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTEN {
        match self.bits {
            false => BOOSTEN::Disabled,
            true => BOOSTEN::Enabled,
        }
    }
    ///I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOOSTEN::Disabled
    }
    ///I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOOSTEN::Enabled
    }
}
///Field `BOOSTEN` writer - I/O analog switch voltage booster enable
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTEN>;
impl<'a, REG> BOOSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Disabled)
    }
    ///I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Enabled)
    }
}
/**Fast-mode Plus (Fm+) driving capability activation on PB6

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP {
    ///0: PB6 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB6_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB6_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB6
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP>;
impl I2C_PB6_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB6_FMP {
        match self.bits {
            false => I2C_PB6_FMP::Standard,
            true => I2C_PB6_FMP::Fmp,
        }
    }
    ///PB6 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB6_FMP::Standard
    }
    ///I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB6_FMP::Fmp
    }
}
///Field `I2C_PB6_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB6
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB6_FMP>;
impl<'a, REG> I2C_PB6_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PB6 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Fmp)
    }
}
/**Fast-mode Plus (Fm+) driving capability activation on PB7

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB7_FMP {
    ///0: PB7 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB7_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB7_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB7
pub type I2C_PB7_FMP_R = crate::BitReader<I2C_PB7_FMP>;
impl I2C_PB7_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB7_FMP {
        match self.bits {
            false => I2C_PB7_FMP::Standard,
            true => I2C_PB7_FMP::Fmp,
        }
    }
    ///PB7 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB7_FMP::Standard
    }
    ///I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB7_FMP::Fmp
    }
}
///Field `I2C_PB7_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB7
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB7_FMP>;
impl<'a, REG> I2C_PB7_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PB7 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::Fmp)
    }
}
/**Fast-mode Plus (Fm+) driving capability activation on PB8

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB8_FMP {
    ///0: PB8 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB8_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB8_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB8
pub type I2C_PB8_FMP_R = crate::BitReader<I2C_PB8_FMP>;
impl I2C_PB8_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB8_FMP {
        match self.bits {
            false => I2C_PB8_FMP::Standard,
            true => I2C_PB8_FMP::Fmp,
        }
    }
    ///PB8 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB8_FMP::Standard
    }
    ///I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB8_FMP::Fmp
    }
}
///Field `I2C_PB8_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB8
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB8_FMP>;
impl<'a, REG> I2C_PB8_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PB8 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::Fmp)
    }
}
/**Fast-mode Plus (Fm+) driving capability activation on PB9

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB9_FMP {
    ///0: PB9 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB9_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB9_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB9
pub type I2C_PB9_FMP_R = crate::BitReader<I2C_PB9_FMP>;
impl I2C_PB9_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB9_FMP {
        match self.bits {
            false => I2C_PB9_FMP::Standard,
            true => I2C_PB9_FMP::Fmp,
        }
    }
    ///PB9 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB9_FMP::Standard
    }
    ///I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB9_FMP::Fmp
    }
}
///Field `I2C_PB9_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB9
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB9_FMP>;
impl<'a, REG> I2C_PB9_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PB9 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::Fmp)
    }
}
/**I2C1 Fast-mode Plus driving capability activation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
    Fmp = 1,
}
impl From<I2C1_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1_FMP` reader - I2C1 Fast-mode Plus driving capability activation
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP>;
impl I2C1_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1_FMP {
        match self.bits {
            false => I2C1_FMP::Standard,
            true => I2C1_FMP::Fmp,
        }
    }
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C1_FMP::Standard
    }
    ///FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C1_FMP::Fmp
    }
}
///Field `I2C1_FMP` writer - I2C1 Fast-mode Plus driving capability activation
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C1_FMP>;
impl<'a, REG> I2C1_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Standard)
    }
    ///FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Fmp)
    }
}
/**I2C3 Fast-mode Plus driving capability activation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C3_FMP {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers
    Fmp = 1,
}
impl From<I2C3_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C3_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C3_FMP` reader - I2C3 Fast-mode Plus driving capability activation
pub type I2C3_FMP_R = crate::BitReader<I2C3_FMP>;
impl I2C3_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C3_FMP {
        match self.bits {
            false => I2C3_FMP::Standard,
            true => I2C3_FMP::Fmp,
        }
    }
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C3_FMP::Standard
    }
    ///FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C3_FMP::Fmp
    }
}
///Field `I2C3_FMP` writer - I2C3 Fast-mode Plus driving capability activation
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C3_FMP>;
impl<'a, REG> I2C3_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C3_FMP::Standard)
    }
    ///FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C3_FMP::Fmp)
    }
}
/**Floating Point Unit interrupts enable bits

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE0 {
    ///0: Invalid operation interrupt disable
    Disabled = 0,
    ///1: Invalid operation interrupt enable
    Enabled = 1,
}
impl From<FPU_IE0> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE0) -> Self {
        variant as u8 != 0
    }
}
///Field `FPU_IE0` reader - Floating Point Unit interrupts enable bits
pub type FPU_IE0_R = crate::BitReader<FPU_IE0>;
impl FPU_IE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE0 {
        match self.bits {
            false => FPU_IE0::Disabled,
            true => FPU_IE0::Enabled,
        }
    }
    ///Invalid operation interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE0::Disabled
    }
    ///Invalid operation interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE0::Enabled
    }
}
///Field `FPU_IE0` writer - Floating Point Unit interrupts enable bits
pub type FPU_IE0_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE0>;
impl<'a, REG> FPU_IE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid operation interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE0::Disabled)
    }
    ///Invalid operation interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE0::Enabled)
    }
}
/**Floating Point Unit interrupts enable bits

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE1 {
    ///0: Devide-by-zero interrupt disable
    Disabled = 0,
    ///1: Devide-by-zero interrupt enable
    Enabled = 1,
}
impl From<FPU_IE1> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE1) -> Self {
        variant as u8 != 0
    }
}
///Field `FPU_IE1` reader - Floating Point Unit interrupts enable bits
pub type FPU_IE1_R = crate::BitReader<FPU_IE1>;
impl FPU_IE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE1 {
        match self.bits {
            false => FPU_IE1::Disabled,
            true => FPU_IE1::Enabled,
        }
    }
    ///Devide-by-zero interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE1::Disabled
    }
    ///Devide-by-zero interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE1::Enabled
    }
}
///Field `FPU_IE1` writer - Floating Point Unit interrupts enable bits
pub type FPU_IE1_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE1>;
impl<'a, REG> FPU_IE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Devide-by-zero interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE1::Disabled)
    }
    ///Devide-by-zero interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE1::Enabled)
    }
}
/**Floating Point Unit interrupts enable bits

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE2 {
    ///0: Underflow interrupt disable
    Disabled = 0,
    ///1: Underflow interrupt enable
    Enabled = 1,
}
impl From<FPU_IE2> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE2) -> Self {
        variant as u8 != 0
    }
}
///Field `FPU_IE2` reader - Floating Point Unit interrupts enable bits
pub type FPU_IE2_R = crate::BitReader<FPU_IE2>;
impl FPU_IE2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE2 {
        match self.bits {
            false => FPU_IE2::Disabled,
            true => FPU_IE2::Enabled,
        }
    }
    ///Underflow interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE2::Disabled
    }
    ///Underflow interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE2::Enabled
    }
}
///Field `FPU_IE2` writer - Floating Point Unit interrupts enable bits
pub type FPU_IE2_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE2>;
impl<'a, REG> FPU_IE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Underflow interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE2::Disabled)
    }
    ///Underflow interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE2::Enabled)
    }
}
/**Floating Point Unit interrupts enable bits

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE3 {
    ///0: Overflow interrupt disable
    Disabled = 0,
    ///1: Overflow interrupt enable
    Enabled = 1,
}
impl From<FPU_IE3> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE3) -> Self {
        variant as u8 != 0
    }
}
///Field `FPU_IE3` reader - Floating Point Unit interrupts enable bits
pub type FPU_IE3_R = crate::BitReader<FPU_IE3>;
impl FPU_IE3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE3 {
        match self.bits {
            false => FPU_IE3::Disabled,
            true => FPU_IE3::Enabled,
        }
    }
    ///Overflow interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE3::Disabled
    }
    ///Overflow interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE3::Enabled
    }
}
///Field `FPU_IE3` writer - Floating Point Unit interrupts enable bits
pub type FPU_IE3_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE3>;
impl<'a, REG> FPU_IE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overflow interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE3::Disabled)
    }
    ///Overflow interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE3::Enabled)
    }
}
/**Floating Point Unit interrupts enable bits

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE4 {
    ///0: Input denormal interrupt disable
    Disabled = 0,
    ///1: Input denormal interrupt enable
    Enabled = 1,
}
impl From<FPU_IE4> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE4) -> Self {
        variant as u8 != 0
    }
}
///Field `FPU_IE4` reader - Floating Point Unit interrupts enable bits
pub type FPU_IE4_R = crate::BitReader<FPU_IE4>;
impl FPU_IE4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE4 {
        match self.bits {
            false => FPU_IE4::Disabled,
            true => FPU_IE4::Enabled,
        }
    }
    ///Input denormal interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE4::Disabled
    }
    ///Input denormal interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE4::Enabled
    }
}
///Field `FPU_IE4` writer - Floating Point Unit interrupts enable bits
pub type FPU_IE4_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE4>;
impl<'a, REG> FPU_IE4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input denormal interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE4::Disabled)
    }
    ///Input denormal interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE4::Enabled)
    }
}
/**Floating Point Unit interrupts enable bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE5 {
    ///0: Inexact interrupt disable
    Disabled = 0,
    ///1: Inexact interrupt enable
    Enabled = 1,
}
impl From<FPU_IE5> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE5) -> Self {
        variant as u8 != 0
    }
}
///Field `FPU_IE5` reader - Floating Point Unit interrupts enable bits
pub type FPU_IE5_R = crate::BitReader<FPU_IE5>;
impl FPU_IE5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE5 {
        match self.bits {
            false => FPU_IE5::Disabled,
            true => FPU_IE5::Enabled,
        }
    }
    ///Inexact interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE5::Disabled
    }
    ///Inexact interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE5::Enabled
    }
}
///Field `FPU_IE5` writer - Floating Point Unit interrupts enable bits
pub type FPU_IE5_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE5>;
impl<'a, REG> FPU_IE5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Inexact interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE5::Disabled)
    }
    ///Inexact interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE5::Enabled)
    }
}
impl R {
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I2C1 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - I2C3 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 26 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie0(&self) -> FPU_IE0_R {
        FPU_IE0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie1(&self) -> FPU_IE1_R {
        FPU_IE1_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie2(&self) -> FPU_IE2_R {
        FPU_IE2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie3(&self) -> FPU_IE3_R {
        FPU_IE3_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie4(&self) -> FPU_IE4_R {
        FPU_IE4_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie5(&self) -> FPU_IE5_R {
        FPU_IE5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("i2c3_fmp", &self.i2c3_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("boosten", &self.boosten())
            .field("fpu_ie0", &self.fpu_ie0())
            .field("fpu_ie1", &self.fpu_ie1())
            .field("fpu_ie2", &self.fpu_ie2())
            .field("fpu_ie3", &self.fpu_ie3())
            .field("fpu_ie4", &self.fpu_ie4())
            .field("fpu_ie5", &self.fpu_ie5())
            .finish()
    }
}
impl W {
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<'_, CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<'_, CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<'_, CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<'_, CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<'_, CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    ///Bit 20 - I2C1 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<'_, CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    ///Bit 22 - I2C3 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<'_, CFGR1rs> {
        I2C3_FMP_W::new(self, 22)
    }
    ///Bit 26 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie0(&mut self) -> FPU_IE0_W<'_, CFGR1rs> {
        FPU_IE0_W::new(self, 26)
    }
    ///Bit 27 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie1(&mut self) -> FPU_IE1_W<'_, CFGR1rs> {
        FPU_IE1_W::new(self, 27)
    }
    ///Bit 28 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie2(&mut self) -> FPU_IE2_W<'_, CFGR1rs> {
        FPU_IE2_W::new(self, 28)
    }
    ///Bit 29 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie3(&mut self) -> FPU_IE3_W<'_, CFGR1rs> {
        FPU_IE3_W::new(self, 29)
    }
    ///Bit 30 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie4(&mut self) -> FPU_IE4_W<'_, CFGR1rs> {
        FPU_IE4_W::new(self, 30)
    }
    ///Bit 31 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie5(&mut self) -> FPU_IE5_W<'_, CFGR1rs> {
        FPU_IE5_W::new(self, 31)
    }
}
/**configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0x7c00_0001
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0x7c00_0001;
}
