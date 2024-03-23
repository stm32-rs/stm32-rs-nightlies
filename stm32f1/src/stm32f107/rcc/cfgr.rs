#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "System clock Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    #[doc = "0: HSI selected as system clock"]
    Hsi = 0,
    #[doc = "1: HSE selected as system clock"]
    Hse = 1,
    #[doc = "2: PLL selected as system clock"]
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
#[doc = "Field `SW` reader - System clock Switch"]
pub type SW_R = crate::FieldReader<SW>;
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SW> {
        match self.bits {
            0 => Some(SW::Hsi),
            1 => Some(SW::Hse),
            2 => Some(SW::Pll),
            _ => None,
        }
    }
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW::Hsi
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW::Hse
    }
    #[doc = "PLL selected as system clock"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SW::Pll
    }
}
#[doc = "Field `SW` writer - System clock Switch"]
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SW>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hsi)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hse)
    }
    #[doc = "PLL selected as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Pll)
    }
}
#[doc = "System Clock Switch Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR {
    #[doc = "0: HSI oscillator used as system clock"]
    Hsi = 0,
    #[doc = "1: HSE oscillator used as system clock"]
    Hse = 1,
    #[doc = "2: PLL used as system clock"]
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
#[doc = "Field `SWS` reader - System Clock Switch Status"]
pub type SWS_R = crate::FieldReader<SWSR>;
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWSR> {
        match self.bits {
            0 => Some(SWSR::Hsi),
            1 => Some(SWSR::Hse),
            2 => Some(SWSR::Pll),
            _ => None,
        }
    }
    #[doc = "HSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR::Hsi
    }
    #[doc = "HSE oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR::Hse
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWSR::Pll
    }
}
#[doc = "AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    #[doc = "0: SYSCLK not divided"]
    Div1 = 0,
    #[doc = "8: SYSCLK divided by 2"]
    Div2 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    Div4 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    Div8 = 10,
    #[doc = "11: SYSCLK divided by 16"]
    Div16 = 11,
    #[doc = "12: SYSCLK divided by 64"]
    Div64 = 12,
    #[doc = "13: SYSCLK divided by 128"]
    Div128 = 13,
    #[doc = "14: SYSCLK divided by 256"]
    Div256 = 14,
    #[doc = "15: SYSCLK divided by 512"]
    Div512 = 15,
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
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HPRE> {
        match self.bits {
            0 => Some(HPRE::Div1),
            8 => Some(HPRE::Div2),
            9 => Some(HPRE::Div4),
            10 => Some(HPRE::Div8),
            11 => Some(HPRE::Div16),
            12 => Some(HPRE::Div64),
            13 => Some(HPRE::Div128),
            14 => Some(HPRE::Div256),
            15 => Some(HPRE::Div512),
            _ => None,
        }
    }
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE::Div1
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE::Div512
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
}
#[doc = "APB Low speed prescaler (APB1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1 {
    #[doc = "0: HCLK not divided"]
    Div1 = 0,
    #[doc = "4: HCLK divided by 2"]
    Div2 = 4,
    #[doc = "5: HCLK divided by 4"]
    Div4 = 5,
    #[doc = "6: HCLK divided by 8"]
    Div8 = 6,
    #[doc = "7: HCLK divided by 16"]
    Div16 = 7,
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
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader<PPRE1>;
impl PPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PPRE1> {
        match self.bits {
            0 => Some(PPRE1::Div1),
            4 => Some(PPRE1::Div2),
            5 => Some(PPRE1::Div4),
            6 => Some(PPRE1::Div8),
            7 => Some(PPRE1::Div16),
            _ => None,
        }
    }
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE1::Div1
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1::Div2
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1::Div4
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1::Div8
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1::Div16
    }
}
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE1>;
impl<'a, REG> PPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div16)
    }
}
#[doc = "Field `PPRE2` reader - APB High speed prescaler (APB2)"]
pub use PPRE1_R as PPRE2_R;
#[doc = "Field `PPRE2` writer - APB High speed prescaler (APB2)"]
pub use PPRE1_W as PPRE2_W;
#[doc = "ADC prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCPRE {
    #[doc = "0: PCLK2 divided by 2"]
    Div2 = 0,
    #[doc = "1: PCLK2 divided by 4"]
    Div4 = 1,
    #[doc = "2: PCLK2 divided by 8"]
    Div6 = 2,
    #[doc = "3: PCLK2 divided by 16"]
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
#[doc = "Field `ADCPRE` reader - ADC prescaler"]
pub type ADCPRE_R = crate::FieldReader<ADCPRE>;
impl ADCPRE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PCLK2 divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRE::Div2
    }
    #[doc = "PCLK2 divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRE::Div4
    }
    #[doc = "PCLK2 divided by 8"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRE::Div6
    }
    #[doc = "PCLK2 divided by 16"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRE::Div8
    }
}
#[doc = "Field `ADCPRE` writer - ADC prescaler"]
pub type ADCPRE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ADCPRE>;
impl<'a, REG> ADCPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK2 divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div2)
    }
    #[doc = "PCLK2 divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div4)
    }
    #[doc = "PCLK2 divided by 8"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div6)
    }
    #[doc = "PCLK2 divided by 16"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div8)
    }
}
#[doc = "PLL entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSRC {
    #[doc = "0: HSI divided by 2 selected as PLL input clock"]
    HsiDiv2 = 0,
    #[doc = "1: HSE divided by PREDIV selected as PLL input clock"]
    HseDivPrediv = 1,
}
impl From<PLLSRC> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PLLSRC_R = crate::BitReader<PLLSRC>;
impl PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSRC {
        match self.bits {
            false => PLLSRC::HsiDiv2,
            true => PLLSRC::HseDivPrediv,
        }
    }
    #[doc = "HSI divided by 2 selected as PLL input clock"]
    #[inline(always)]
    pub fn is_hsi_div2(&self) -> bool {
        *self == PLLSRC::HsiDiv2
    }
    #[doc = "HSE divided by PREDIV selected as PLL input clock"]
    #[inline(always)]
    pub fn is_hse_div_prediv(&self) -> bool {
        *self == PLLSRC::HseDivPrediv
    }
}
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PLLSRC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSRC>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI divided by 2 selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi_div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::HsiDiv2)
    }
    #[doc = "HSE divided by PREDIV selected as PLL input clock"]
    #[inline(always)]
    pub fn hse_div_prediv(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::HseDivPrediv)
    }
}
#[doc = "HSE divider for PLL entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLXTPRE {
    #[doc = "0: HSE clock not divided"]
    Div1 = 0,
    #[doc = "1: HSE clock divided by 2"]
    Div2 = 1,
}
impl From<PLLXTPRE> for bool {
    #[inline(always)]
    fn from(variant: PLLXTPRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry"]
pub type PLLXTPRE_R = crate::BitReader<PLLXTPRE>;
impl PLLXTPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLXTPRE {
        match self.bits {
            false => PLLXTPRE::Div1,
            true => PLLXTPRE::Div2,
        }
    }
    #[doc = "HSE clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLXTPRE::Div1
    }
    #[doc = "HSE clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLXTPRE::Div2
    }
}
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry"]
pub type PLLXTPRE_W<'a, REG> = crate::BitWriter<'a, REG, PLLXTPRE>;
impl<'a, REG> PLLXTPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLXTPRE::Div1)
    }
    #[doc = "HSE clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLXTPRE::Div2)
    }
}
#[doc = "PLL Multiplication Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL {
    #[doc = "2: PLL input clock x4"]
    Mul4 = 2,
    #[doc = "3: PLL input clock x5"]
    Mul5 = 3,
    #[doc = "4: PLL input clock x6"]
    Mul6 = 4,
    #[doc = "5: PLL input clock x7"]
    Mul7 = 5,
    #[doc = "6: PLL input clock x8"]
    Mul8 = 6,
    #[doc = "7: PLL input clock x9"]
    Mul9 = 7,
    #[doc = "13: PLL input clock x6.5"]
    Mul6_5 = 13,
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
#[doc = "Field `PLLMUL` reader - PLL Multiplication Factor"]
pub type PLLMUL_R = crate::FieldReader<PLLMUL>;
impl PLLMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLMUL> {
        match self.bits {
            2 => Some(PLLMUL::Mul4),
            3 => Some(PLLMUL::Mul5),
            4 => Some(PLLMUL::Mul6),
            5 => Some(PLLMUL::Mul7),
            6 => Some(PLLMUL::Mul8),
            7 => Some(PLLMUL::Mul9),
            13 => Some(PLLMUL::Mul6_5),
            _ => None,
        }
    }
    #[doc = "PLL input clock x4"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMUL::Mul4
    }
    #[doc = "PLL input clock x5"]
    #[inline(always)]
    pub fn is_mul5(&self) -> bool {
        *self == PLLMUL::Mul5
    }
    #[doc = "PLL input clock x6"]
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMUL::Mul6
    }
    #[doc = "PLL input clock x7"]
    #[inline(always)]
    pub fn is_mul7(&self) -> bool {
        *self == PLLMUL::Mul7
    }
    #[doc = "PLL input clock x8"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMUL::Mul8
    }
    #[doc = "PLL input clock x9"]
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        *self == PLLMUL::Mul9
    }
    #[doc = "PLL input clock x6.5"]
    #[inline(always)]
    pub fn is_mul6_5(&self) -> bool {
        *self == PLLMUL::Mul6_5
    }
}
#[doc = "Field `PLLMUL` writer - PLL Multiplication Factor"]
pub type PLLMUL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLLMUL>;
impl<'a, REG> PLLMUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL input clock x4"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul4)
    }
    #[doc = "PLL input clock x5"]
    #[inline(always)]
    pub fn mul5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul5)
    }
    #[doc = "PLL input clock x6"]
    #[inline(always)]
    pub fn mul6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul6)
    }
    #[doc = "PLL input clock x7"]
    #[inline(always)]
    pub fn mul7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul7)
    }
    #[doc = "PLL input clock x8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul8)
    }
    #[doc = "PLL input clock x9"]
    #[inline(always)]
    pub fn mul9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul9)
    }
    #[doc = "PLL input clock x6.5"]
    #[inline(always)]
    pub fn mul6_5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul6_5)
    }
}
#[doc = "USB OTG FS prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSPRE {
    #[doc = "0: PLL clock is divided by 1.5"]
    Div1_5 = 0,
    #[doc = "1: PLL clock is not divided"]
    Div1 = 1,
}
impl From<OTGFSPRE> for bool {
    #[inline(always)]
    fn from(variant: OTGFSPRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGFSPRE` reader - USB OTG FS prescaler"]
pub type OTGFSPRE_R = crate::BitReader<OTGFSPRE>;
impl OTGFSPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OTGFSPRE {
        match self.bits {
            false => OTGFSPRE::Div1_5,
            true => OTGFSPRE::Div1,
        }
    }
    #[doc = "PLL clock is divided by 1.5"]
    #[inline(always)]
    pub fn is_div1_5(&self) -> bool {
        *self == OTGFSPRE::Div1_5
    }
    #[doc = "PLL clock is not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == OTGFSPRE::Div1
    }
}
#[doc = "Field `OTGFSPRE` writer - USB OTG FS prescaler"]
pub type OTGFSPRE_W<'a, REG> = crate::BitWriter<'a, REG, OTGFSPRE>;
impl<'a, REG> OTGFSPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL clock is divided by 1.5"]
    #[inline(always)]
    pub fn div1_5(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSPRE::Div1_5)
    }
    #[doc = "PLL clock is not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSPRE::Div1)
    }
}
#[doc = "Microcontroller clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO {
    #[doc = "0: MCO output disabled, no clock on MCO"]
    NoMco = 0,
    #[doc = "4: System clock selected"]
    Sysclk = 4,
    #[doc = "5: HSI oscillator clock selected"]
    Hsi = 5,
    #[doc = "6: HSE oscillator clock selected"]
    Hse = 6,
    #[doc = "7: PLL clock divided by 2 selected"]
    Pll = 7,
    #[doc = "8: PLL2 clock selected"]
    Pll2 = 8,
    #[doc = "9: PLL3 clock divided by 2 selected"]
    Pll3 = 9,
    #[doc = "10: XT1 external 3-25 MHz oscillator clock selected (for Ethernet)"]
    Xt1 = 10,
    #[doc = "11: PLL3 clock selected (for Ethernet)"]
    Pll3ethernet = 11,
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
#[doc = "Field `MCO` reader - Microcontroller clock output"]
pub type MCO_R = crate::FieldReader<MCO>;
impl MCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO> {
        match self.bits {
            0 => Some(MCO::NoMco),
            4 => Some(MCO::Sysclk),
            5 => Some(MCO::Hsi),
            6 => Some(MCO::Hse),
            7 => Some(MCO::Pll),
            8 => Some(MCO::Pll2),
            9 => Some(MCO::Pll3),
            10 => Some(MCO::Xt1),
            11 => Some(MCO::Pll3ethernet),
            _ => None,
        }
    }
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn is_no_mco(&self) -> bool {
        *self == MCO::NoMco
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO::Sysclk
    }
    #[doc = "HSI oscillator clock selected"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO::Hsi
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO::Hse
    }
    #[doc = "PLL clock divided by 2 selected"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO::Pll
    }
    #[doc = "PLL2 clock selected"]
    #[inline(always)]
    pub fn is_pll2(&self) -> bool {
        *self == MCO::Pll2
    }
    #[doc = "PLL3 clock divided by 2 selected"]
    #[inline(always)]
    pub fn is_pll3(&self) -> bool {
        *self == MCO::Pll3
    }
    #[doc = "XT1 external 3-25 MHz oscillator clock selected (for Ethernet)"]
    #[inline(always)]
    pub fn is_xt1(&self) -> bool {
        *self == MCO::Xt1
    }
    #[doc = "PLL3 clock selected (for Ethernet)"]
    #[inline(always)]
    pub fn is_pll3ethernet(&self) -> bool {
        *self == MCO::Pll3ethernet
    }
}
#[doc = "Field `MCO` writer - Microcontroller clock output"]
pub type MCO_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCO>;
impl<'a, REG> MCO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn no_mco(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::NoMco)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Sysclk)
    }
    #[doc = "HSI oscillator clock selected"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Hsi)
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Hse)
    }
    #[doc = "PLL clock divided by 2 selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Pll)
    }
    #[doc = "PLL2 clock selected"]
    #[inline(always)]
    pub fn pll2(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Pll2)
    }
    #[doc = "PLL3 clock divided by 2 selected"]
    #[inline(always)]
    pub fn pll3(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Pll3)
    }
    #[doc = "XT1 external 3-25 MHz oscillator clock selected (for Ethernet)"]
    #[inline(always)]
    pub fn xt1(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Xt1)
    }
    #[doc = "PLL3 clock selected (for Ethernet)"]
    #[inline(always)]
    pub fn pll3ethernet(self) -> &'a mut crate::W<REG> {
        self.variant(MCO::Pll3ethernet)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock Switch Status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB High speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - USB OTG FS prescaler"]
    #[inline(always)]
    pub fn otgfspre(&self) -> OTGFSPRE_R {
        OTGFSPRE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<CFGRrs> {
        SW_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<CFGRrs> {
        HPRE_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<CFGRrs> {
        PPRE1_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB High speed prescaler (APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<CFGRrs> {
        PPRE2_W::new(self, 11)
    }
    #[doc = "Bits 14:15 - ADC prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adcpre(&mut self) -> ADCPRE_W<CFGRrs> {
        ADCPRE_W::new(self, 14)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<CFGRrs> {
        PLLSRC_W::new(self, 16)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    #[must_use]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W<CFGRrs> {
        PLLXTPRE_W::new(self, 17)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<CFGRrs> {
        PLLMUL_W::new(self, 18)
    }
    #[doc = "Bit 22 - USB OTG FS prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn otgfspre(&mut self) -> OTGFSPRE_W<CFGRrs> {
        OTGFSPRE_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    #[must_use]
    pub fn mco(&mut self) -> MCO_W<CFGRrs> {
        MCO_W::new(self, 24)
    }
}
#[doc = "Clock configuration register (RCC_CFGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
