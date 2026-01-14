///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**System clock Switch

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    ///0: HSI selected as system clock
    Hsi = 0,
    ///1: HSE selected as system clock
    Hse = 1,
    ///2: PLL selected as system clock
    Pll = 2,
}
impl From<SW> for u8 {
    #[inline(always)]
    fn from(variant: SW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SW {
    type Ux = u8;
}
impl crate::IsEnum for SW {}
///Field `SW` reader - System clock Switch
pub type SW_R = crate::FieldReader<SW>;
impl SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SW> {
        match self.bits {
            0 => Some(SW::Hsi),
            1 => Some(SW::Hse),
            2 => Some(SW::Pll),
            _ => None,
        }
    }
    ///HSI selected as system clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW::Hsi
    }
    ///HSE selected as system clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW::Hse
    }
    ///PLL selected as system clock
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SW::Pll
    }
}
///Field `SW` writer - System clock Switch
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SW>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI selected as system clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hsi)
    }
    ///HSE selected as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hse)
    }
    ///PLL selected as system clock
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Pll)
    }
}
/**System Clock Switch Status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR {
    ///0: HSI oscillator used as system clock
    Hsi = 0,
    ///1: HSE oscillator used as system clock
    Hse = 1,
    ///2: PLL used as system clock
    Pll = 2,
}
impl From<SWSR> for u8 {
    #[inline(always)]
    fn from(variant: SWSR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWSR {
    type Ux = u8;
}
impl crate::IsEnum for SWSR {}
///Field `SWS` reader - System Clock Switch Status
pub type SWS_R = crate::FieldReader<SWSR>;
impl SWS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWSR> {
        match self.bits {
            0 => Some(SWSR::Hsi),
            1 => Some(SWSR::Hse),
            2 => Some(SWSR::Pll),
            _ => None,
        }
    }
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR::Hsi
    }
    ///HSE oscillator used as system clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR::Hse
    }
    ///PLL used as system clock
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWSR::Pll
    }
}
/**AHB prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    ///8: SYSCLK divided by 2
    Div2 = 8,
    ///9: SYSCLK divided by 4
    Div4 = 9,
    ///10: SYSCLK divided by 8
    Div8 = 10,
    ///11: SYSCLK divided by 16
    Div16 = 11,
    ///12: SYSCLK divided by 64
    Div64 = 12,
    ///13: SYSCLK divided by 128
    Div128 = 13,
    ///14: SYSCLK divided by 256
    Div256 = 14,
    ///15: SYSCLK divided by 512
    Div512 = 15,
    ///0: SYSCLK not divided
    Div1 = 0,
}
impl From<HPRE> for u8 {
    #[inline(always)]
    fn from(variant: HPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE {
    type Ux = u8;
}
impl crate::IsEnum for HPRE {}
///Field `HPRE` reader - AHB prescaler
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPRE {
        match self.bits {
            8 => HPRE::Div2,
            9 => HPRE::Div4,
            10 => HPRE::Div8,
            11 => HPRE::Div16,
            12 => HPRE::Div64,
            13 => HPRE::Div128,
            14 => HPRE::Div256,
            15 => HPRE::Div512,
            _ => HPRE::Div1,
        }
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    ///SYSCLK divided by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE::Div512
    }
    ///SYSCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), HPRE::Div1)
    }
}
///Field `HPRE` writer - AHB prescaler
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE, crate::Safe>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    ///SYSCLK divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
    ///SYSCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
}
/**APB Low speed prescaler (APB1)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1 {
    ///4: HCLK divided by 2
    Div2 = 4,
    ///5: HCLK divided by 4
    Div4 = 5,
    ///6: HCLK divided by 8
    Div8 = 6,
    ///7: HCLK divided by 16
    Div16 = 7,
    ///0: HCLK not divided
    Div1 = 0,
}
impl From<PPRE1> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE1 {
    type Ux = u8;
}
impl crate::IsEnum for PPRE1 {}
///Field `PPRE1` reader - APB Low speed prescaler (APB1)
pub type PPRE1_R = crate::FieldReader<PPRE1>;
impl PPRE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PPRE1 {
        match self.bits {
            4 => PPRE1::Div2,
            5 => PPRE1::Div4,
            6 => PPRE1::Div8,
            7 => PPRE1::Div16,
            _ => PPRE1::Div1,
        }
    }
    ///HCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1::Div2
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1::Div4
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1::Div8
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1::Div16
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), PPRE1::Div1)
    }
}
///Field `PPRE1` writer - APB Low speed prescaler (APB1)
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE1, crate::Safe>;
impl<'a, REG> PPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div2)
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div4)
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div8)
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div16)
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div1)
    }
}
///Field `PPRE2` reader - APB high speed prescaler (APB2)
pub use PPRE1_R as PPRE2_R;
///Field `PPRE2` writer - APB high speed prescaler (APB2)
pub use PPRE1_W as PPRE2_W;
/**ADC prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCPRE {
    ///0: PCLK divided by 2
    Div2 = 0,
    ///1: PCLK divided by 4
    Div4 = 1,
    ///2: PCLK divided by 6
    Div6 = 2,
    ///3: PCLK divided by 8
    Div8 = 3,
}
impl From<ADCPRE> for u8 {
    #[inline(always)]
    fn from(variant: ADCPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCPRE {
    type Ux = u8;
}
impl crate::IsEnum for ADCPRE {}
///Field `ADCPRE` reader - ADC prescaler
pub type ADCPRE_R = crate::FieldReader<ADCPRE>;
impl ADCPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCPRE {
        match self.bits {
            0 => ADCPRE::Div2,
            1 => ADCPRE::Div4,
            2 => ADCPRE::Div6,
            3 => ADCPRE::Div8,
            _ => unreachable!(),
        }
    }
    ///PCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRE::Div2
    }
    ///PCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRE::Div4
    }
    ///PCLK divided by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRE::Div6
    }
    ///PCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRE::Div8
    }
}
///Field `ADCPRE` writer - ADC prescaler
pub type ADCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADCPRE, crate::Safe>;
impl<'a, REG> ADCPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div2)
    }
    ///PCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div4)
    }
    ///PCLK divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div6)
    }
    ///PCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div8)
    }
}
/**PLL entry clock source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSRC {
    ///0: HSI divided by 2 selected as PLL input clock
    HsiDiv2 = 0,
    ///1: HSE divided by PREDIV selected as PLL input clock
    HseDivPrediv = 1,
}
impl From<PLLSRC> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSRC` reader - PLL entry clock source
pub type PLLSRC_R = crate::BitReader<PLLSRC>;
impl PLLSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSRC {
        match self.bits {
            false => PLLSRC::HsiDiv2,
            true => PLLSRC::HseDivPrediv,
        }
    }
    ///HSI divided by 2 selected as PLL input clock
    #[inline(always)]
    pub fn is_hsi_div2(&self) -> bool {
        *self == PLLSRC::HsiDiv2
    }
    ///HSE divided by PREDIV selected as PLL input clock
    #[inline(always)]
    pub fn is_hse_div_prediv(&self) -> bool {
        *self == PLLSRC::HseDivPrediv
    }
}
///Field `PLLSRC` writer - PLL entry clock source
pub type PLLSRC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSRC>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI divided by 2 selected as PLL input clock
    #[inline(always)]
    pub fn hsi_div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::HsiDiv2)
    }
    ///HSE divided by PREDIV selected as PLL input clock
    #[inline(always)]
    pub fn hse_div_prediv(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::HseDivPrediv)
    }
}
/**HSE divider for PLL entry

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLXTPRE {
    ///0: HSE clock not divided
    Div1 = 0,
    ///1: HSE clock divided by 2
    Div2 = 1,
}
impl From<PLLXTPRE> for bool {
    #[inline(always)]
    fn from(variant: PLLXTPRE) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLXTPRE` reader - HSE divider for PLL entry
pub type PLLXTPRE_R = crate::BitReader<PLLXTPRE>;
impl PLLXTPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLXTPRE {
        match self.bits {
            false => PLLXTPRE::Div1,
            true => PLLXTPRE::Div2,
        }
    }
    ///HSE clock not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLXTPRE::Div1
    }
    ///HSE clock divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLXTPRE::Div2
    }
}
///Field `PLLXTPRE` writer - HSE divider for PLL entry
pub type PLLXTPRE_W<'a, REG> = crate::BitWriter<'a, REG, PLLXTPRE>;
impl<'a, REG> PLLXTPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLXTPRE::Div1)
    }
    ///HSE clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLXTPRE::Div2)
    }
}
/**PLL Multiplication Factor

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL {
    ///0: PLL input clock x2
    Mul2 = 0,
    ///1: PLL input clock x3
    Mul3 = 1,
    ///2: PLL input clock x4
    Mul4 = 2,
    ///3: PLL input clock x5
    Mul5 = 3,
    ///4: PLL input clock x6
    Mul6 = 4,
    ///5: PLL input clock x7
    Mul7 = 5,
    ///6: PLL input clock x8
    Mul8 = 6,
    ///7: PLL input clock x9
    Mul9 = 7,
    ///8: PLL input clock x10
    Mul10 = 8,
    ///9: PLL input clock x11
    Mul11 = 9,
    ///10: PLL input clock x12
    Mul12 = 10,
    ///11: PLL input clock x13
    Mul13 = 11,
    ///12: PLL input clock x14
    Mul14 = 12,
    ///13: PLL input clock x15
    Mul15 = 13,
    ///14: PLL input clock x16
    Mul16 = 14,
    ///15: PLL input clock x16
    Mul16x = 15,
}
impl From<PLLMUL> for u8 {
    #[inline(always)]
    fn from(variant: PLLMUL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLMUL {
    type Ux = u8;
}
impl crate::IsEnum for PLLMUL {}
///Field `PLLMUL` reader - PLL Multiplication Factor
pub type PLLMUL_R = crate::FieldReader<PLLMUL>;
impl PLLMUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLMUL {
        match self.bits {
            0 => PLLMUL::Mul2,
            1 => PLLMUL::Mul3,
            2 => PLLMUL::Mul4,
            3 => PLLMUL::Mul5,
            4 => PLLMUL::Mul6,
            5 => PLLMUL::Mul7,
            6 => PLLMUL::Mul8,
            7 => PLLMUL::Mul9,
            8 => PLLMUL::Mul10,
            9 => PLLMUL::Mul11,
            10 => PLLMUL::Mul12,
            11 => PLLMUL::Mul13,
            12 => PLLMUL::Mul14,
            13 => PLLMUL::Mul15,
            14 => PLLMUL::Mul16,
            15 => PLLMUL::Mul16x,
            _ => unreachable!(),
        }
    }
    ///PLL input clock x2
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == PLLMUL::Mul2
    }
    ///PLL input clock x3
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMUL::Mul3
    }
    ///PLL input clock x4
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMUL::Mul4
    }
    ///PLL input clock x5
    #[inline(always)]
    pub fn is_mul5(&self) -> bool {
        *self == PLLMUL::Mul5
    }
    ///PLL input clock x6
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMUL::Mul6
    }
    ///PLL input clock x7
    #[inline(always)]
    pub fn is_mul7(&self) -> bool {
        *self == PLLMUL::Mul7
    }
    ///PLL input clock x8
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMUL::Mul8
    }
    ///PLL input clock x9
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        *self == PLLMUL::Mul9
    }
    ///PLL input clock x10
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        *self == PLLMUL::Mul10
    }
    ///PLL input clock x11
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        *self == PLLMUL::Mul11
    }
    ///PLL input clock x12
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMUL::Mul12
    }
    ///PLL input clock x13
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        *self == PLLMUL::Mul13
    }
    ///PLL input clock x14
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        *self == PLLMUL::Mul14
    }
    ///PLL input clock x15
    #[inline(always)]
    pub fn is_mul15(&self) -> bool {
        *self == PLLMUL::Mul15
    }
    ///PLL input clock x16
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMUL::Mul16
    }
    ///PLL input clock x16
    #[inline(always)]
    pub fn is_mul16x(&self) -> bool {
        *self == PLLMUL::Mul16x
    }
}
///Field `PLLMUL` writer - PLL Multiplication Factor
pub type PLLMUL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLLMUL, crate::Safe>;
impl<'a, REG> PLLMUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL input clock x2
    #[inline(always)]
    pub fn mul2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul2)
    }
    ///PLL input clock x3
    #[inline(always)]
    pub fn mul3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul3)
    }
    ///PLL input clock x4
    #[inline(always)]
    pub fn mul4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul4)
    }
    ///PLL input clock x5
    #[inline(always)]
    pub fn mul5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul5)
    }
    ///PLL input clock x6
    #[inline(always)]
    pub fn mul6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul6)
    }
    ///PLL input clock x7
    #[inline(always)]
    pub fn mul7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul7)
    }
    ///PLL input clock x8
    #[inline(always)]
    pub fn mul8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul8)
    }
    ///PLL input clock x9
    #[inline(always)]
    pub fn mul9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul9)
    }
    ///PLL input clock x10
    #[inline(always)]
    pub fn mul10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul10)
    }
    ///PLL input clock x11
    #[inline(always)]
    pub fn mul11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul11)
    }
    ///PLL input clock x12
    #[inline(always)]
    pub fn mul12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul12)
    }
    ///PLL input clock x13
    #[inline(always)]
    pub fn mul13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul13)
    }
    ///PLL input clock x14
    #[inline(always)]
    pub fn mul14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul14)
    }
    ///PLL input clock x15
    #[inline(always)]
    pub fn mul15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul15)
    }
    ///PLL input clock x16
    #[inline(always)]
    pub fn mul16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul16)
    }
    ///PLL input clock x16
    #[inline(always)]
    pub fn mul16x(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul16x)
    }
}
/**USB prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPRE {
    ///0: PLL clock is divided by 1.5
    Div1_5 = 0,
    ///1: PLL clock is not divided
    Div1 = 1,
}
impl From<USBPRE> for bool {
    #[inline(always)]
    fn from(variant: USBPRE) -> Self {
        variant as u8 != 0
    }
}
///Field `USBPRE` reader - USB prescaler
pub type USBPRE_R = crate::BitReader<USBPRE>;
impl USBPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBPRE {
        match self.bits {
            false => USBPRE::Div1_5,
            true => USBPRE::Div1,
        }
    }
    ///PLL clock is divided by 1.5
    #[inline(always)]
    pub fn is_div1_5(&self) -> bool {
        *self == USBPRE::Div1_5
    }
    ///PLL clock is not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == USBPRE::Div1
    }
}
///Field `USBPRE` writer - USB prescaler
pub type USBPRE_W<'a, REG> = crate::BitWriter<'a, REG, USBPRE>;
impl<'a, REG> USBPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL clock is divided by 1.5
    #[inline(always)]
    pub fn div1_5(self) -> &'a mut crate::W<REG> {
        self.variant(USBPRE::Div1_5)
    }
    ///PLL clock is not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(USBPRE::Div1)
    }
}
/**Microcontroller clock output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO {
    ///0: MCO output disabled, no clock on MCO
    NoMco = 0,
    ///2: Internal low speed (LSI) oscillator clock selected
    Lsi = 2,
    ///3: External low speed (LSE) oscillator clock selected
    Lse = 3,
    ///4: System clock selected
    Sysclk = 4,
    ///5: Internal RC 8 MHz (HSI) oscillator clock selected
    Hsi = 5,
    ///6: External 4-32 MHz (HSE) oscillator clock selected
    Hse = 6,
    ///7: PLL clock selected (divided by 1 or 2, depending en PLLNODIV)
    Pll = 7,
}
impl From<MCO> for u8 {
    #[inline(always)]
    fn from(variant: MCO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO {
    type Ux = u8;
}
impl crate::IsEnum for MCO {}
///Field `MCO` reader - Microcontroller clock output
pub type MCO_R = crate::FieldReader<MCO>;
impl MCO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO> {
        match self.bits {
            0 => Some(MCO::NoMco),
            2 => Some(MCO::Lsi),
            3 => Some(MCO::Lse),
            4 => Some(MCO::Sysclk),
            5 => Some(MCO::Hsi),
            6 => Some(MCO::Hse),
            7 => Some(MCO::Pll),
            _ => None,
        }
    }
    ///MCO output disabled, no clock on MCO
    #[inline(always)]
    pub fn is_no_mco(&self) -> bool {
        *self == MCO::NoMco
    }
    ///Internal low speed (LSI) oscillator clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO::Lsi
    }
    ///External low speed (LSE) oscillator clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO::Lse
    }
    ///System clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO::Sysclk
    }
    ///Internal RC 8 MHz (HSI) oscillator clock selected
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO::Hsi
    }
    ///External 4-32 MHz (HSE) oscillator clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO::Hse
    }
    ///PLL clock selected (divided by 1 or 2, depending en PLLNODIV)
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO::Pll
    }
}
///Field `MCO` writer - Microcontroller clock output
pub type MCO_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO>;
impl<'a, REG> MCO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MCO output disabled, no clock on MCO
    #[inline(always)]
    pub fn no_mco(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::NoMco)
    }
    ///Internal low speed (LSI) oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Lsi)
    }
    ///External low speed (LSE) oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Lse)
    }
    ///System clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Sysclk)
    }
    ///Internal RC 8 MHz (HSI) oscillator clock selected
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Hsi)
    }
    ///External 4-32 MHz (HSE) oscillator clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Hse)
    }
    ///PLL clock selected (divided by 1 or 2, depending en PLLNODIV)
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Pll)
    }
}
/**SDADC prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDPRE {
    ///17: SYSCLK divided by 4
    Div4 = 17,
    ///18: SYSCLK divided by 6
    Div6 = 18,
    ///19: SYSCLK divided by 8
    Div8 = 19,
    ///20: SYSCLK divided by 10
    Div10 = 20,
    ///21: SYSCLK divided by 12
    Div12 = 21,
    ///22: SYSCLK divided by 14
    Div14 = 22,
    ///23: SYSCLK divided by 16
    Div16 = 23,
    ///24: SYSCLK divided by 20
    Div20 = 24,
    ///25: SYSCLK divided by 24
    Div24 = 25,
    ///26: SYSCLK divided by 28
    Div28 = 26,
    ///27: SYSCLK divided by 32
    Div32 = 27,
    ///28: SYSCLK divided by 36
    Div36 = 28,
    ///29: SYSCLK divided by 40
    Div40 = 29,
    ///30: SYSCLK divided by 44
    Div44 = 30,
    ///31: SYSCLK divided by 48
    Div48 = 31,
    ///0: SYSCLK divided by 2
    Div2 = 0,
}
impl From<SDPRE> for u8 {
    #[inline(always)]
    fn from(variant: SDPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDPRE {
    type Ux = u8;
}
impl crate::IsEnum for SDPRE {}
///Field `SDPRE` reader - SDADC prescaler
pub type SDPRE_R = crate::FieldReader<SDPRE>;
impl SDPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDPRE {
        match self.bits {
            17 => SDPRE::Div4,
            18 => SDPRE::Div6,
            19 => SDPRE::Div8,
            20 => SDPRE::Div10,
            21 => SDPRE::Div12,
            22 => SDPRE::Div14,
            23 => SDPRE::Div16,
            24 => SDPRE::Div20,
            25 => SDPRE::Div24,
            26 => SDPRE::Div28,
            27 => SDPRE::Div32,
            28 => SDPRE::Div36,
            29 => SDPRE::Div40,
            30 => SDPRE::Div44,
            31 => SDPRE::Div48,
            _ => SDPRE::Div2,
        }
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SDPRE::Div4
    }
    ///SYSCLK divided by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == SDPRE::Div6
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SDPRE::Div8
    }
    ///SYSCLK divided by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == SDPRE::Div10
    }
    ///SYSCLK divided by 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == SDPRE::Div12
    }
    ///SYSCLK divided by 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == SDPRE::Div14
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == SDPRE::Div16
    }
    ///SYSCLK divided by 20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == SDPRE::Div20
    }
    ///SYSCLK divided by 24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == SDPRE::Div24
    }
    ///SYSCLK divided by 28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == SDPRE::Div28
    }
    ///SYSCLK divided by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == SDPRE::Div32
    }
    ///SYSCLK divided by 36
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        *self == SDPRE::Div36
    }
    ///SYSCLK divided by 40
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == SDPRE::Div40
    }
    ///SYSCLK divided by 44
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        *self == SDPRE::Div44
    }
    ///SYSCLK divided by 48
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == SDPRE::Div48
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        matches!(self.variant(), SDPRE::Div2)
    }
}
///Field `SDPRE` writer - SDADC prescaler
pub type SDPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SDPRE, crate::Safe>;
impl<'a, REG> SDPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div4)
    }
    ///SYSCLK divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div6)
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div8)
    }
    ///SYSCLK divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div10)
    }
    ///SYSCLK divided by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div12)
    }
    ///SYSCLK divided by 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div14)
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div16)
    }
    ///SYSCLK divided by 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div20)
    }
    ///SYSCLK divided by 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div24)
    }
    ///SYSCLK divided by 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div28)
    }
    ///SYSCLK divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div32)
    }
    ///SYSCLK divided by 36
    #[inline(always)]
    pub fn div36(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div36)
    }
    ///SYSCLK divided by 40
    #[inline(always)]
    pub fn div40(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div40)
    }
    ///SYSCLK divided by 44
    #[inline(always)]
    pub fn div44(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div44)
    }
    ///SYSCLK divided by 48
    #[inline(always)]
    pub fn div48(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div48)
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(SDPRE::Div2)
    }
}
impl R {
    ///Bits 0:1 - System clock Switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - System Clock Switch Status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - APB Low speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - APB high speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:15 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - PLL entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE divider for PLL entry
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:21 - PLL Multiplication Factor
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - USB prescaler
    #[inline(always)]
    pub fn usbpre(&self) -> USBPRE_R {
        USBPRE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:26 - Microcontroller clock output
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:31 - SDADC prescaler
    #[inline(always)]
    pub fn sdpre(&self) -> SDPRE_R {
        SDPRE_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("hpre", &self.hpre())
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("adcpre", &self.adcpre())
            .field("pllsrc", &self.pllsrc())
            .field("pllxtpre", &self.pllxtpre())
            .field("pllmul", &self.pllmul())
            .field("usbpre", &self.usbpre())
            .field("mco", &self.mco())
            .field("sdpre", &self.sdpre())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - System clock Switch
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, CFGRrs> {
        SW_W::new(self, 0)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, CFGRrs> {
        HPRE_W::new(self, 4)
    }
    ///Bits 8:10 - APB Low speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<'_, CFGRrs> {
        PPRE1_W::new(self, 8)
    }
    ///Bits 11:13 - APB high speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<'_, CFGRrs> {
        PPRE2_W::new(self, 11)
    }
    ///Bits 14:15 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W<'_, CFGRrs> {
        ADCPRE_W::new(self, 14)
    }
    ///Bit 16 - PLL entry clock source
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<'_, CFGRrs> {
        PLLSRC_W::new(self, 16)
    }
    ///Bit 17 - HSE divider for PLL entry
    #[inline(always)]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W<'_, CFGRrs> {
        PLLXTPRE_W::new(self, 17)
    }
    ///Bits 18:21 - PLL Multiplication Factor
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W<'_, CFGRrs> {
        PLLMUL_W::new(self, 18)
    }
    ///Bit 22 - USB prescaler
    #[inline(always)]
    pub fn usbpre(&mut self) -> USBPRE_W<'_, CFGRrs> {
        USBPRE_W::new(self, 22)
    }
    ///Bits 24:26 - Microcontroller clock output
    #[inline(always)]
    pub fn mco(&mut self) -> MCO_W<'_, CFGRrs> {
        MCO_W::new(self, 24)
    }
    ///Bits 27:31 - SDADC prescaler
    #[inline(always)]
    pub fn sdpre(&mut self) -> SDPRE_W<'_, CFGRrs> {
        SDPRE_W::new(self, 27)
    }
}
/**Clock configuration register (RCC_CFGR)

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#RCC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
