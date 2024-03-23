#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "System clock switch\n\nValue on reset: 0"]
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
#[doc = "Field `SW` reader - System clock switch"]
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
#[doc = "Field `SW` writer - System clock switch"]
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
#[doc = "System clock switch status\n\nValue on reset: 0"]
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
#[doc = "Field `SWS` reader - System clock switch status"]
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
#[doc = "Field `SWS` writer - System clock switch status"]
pub type SWS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SWSR>;
impl<'a, REG> SWS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI oscillator used as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(SWSR::Hsi)
    }
    #[doc = "HSE oscillator used as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SWSR::Hse)
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SWSR::Pll)
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
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2)"]
pub use PPRE1_R as PPRE2_R;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2)"]
pub use PPRE1_W as PPRE2_W;
#[doc = "Field `RTCPRE` reader - HSE division factor for RTC clock"]
pub type RTCPRE_R = crate::FieldReader;
#[doc = "Field `RTCPRE` writer - HSE division factor for RTC clock"]
pub type RTCPRE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Microcontroller clock output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1 {
    #[doc = "0: HSI clock selected"]
    Hsi = 0,
    #[doc = "1: LSE oscillator selected"]
    Lse = 1,
    #[doc = "2: HSE oscillator clock selected"]
    Hse = 2,
    #[doc = "3: PLL clock selected"]
    Pll = 3,
}
impl From<MCO1> for u8 {
    #[inline(always)]
    fn from(variant: MCO1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO1 {
    type Ux = u8;
}
#[doc = "Field `MCO1` reader - Microcontroller clock output 1"]
pub type MCO1_R = crate::FieldReader<MCO1>;
impl MCO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCO1 {
        match self.bits {
            0 => MCO1::Hsi,
            1 => MCO1::Lse,
            2 => MCO1::Hse,
            3 => MCO1::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "HSI clock selected"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO1::Hsi
    }
    #[doc = "LSE oscillator selected"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO1::Lse
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO1::Hse
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO1::Pll
    }
}
#[doc = "Field `MCO1` writer - Microcontroller clock output 1"]
pub type MCO1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MCO1>;
impl<'a, REG> MCO1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI clock selected"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1::Hsi)
    }
    #[doc = "LSE oscillator selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1::Lse)
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1::Hse)
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1::Pll)
    }
}
#[doc = "I2S clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SSRC {
    #[doc = "0: PLLI2S clock used as I2S clock source"]
    Plli2s = 0,
    #[doc = "1: External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    Ckin = 1,
}
impl From<I2SSRC> for bool {
    #[inline(always)]
    fn from(variant: I2SSRC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SSRC` reader - I2S clock selection"]
pub type I2SSRC_R = crate::BitReader<I2SSRC>;
impl I2SSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SSRC {
        match self.bits {
            false => I2SSRC::Plli2s,
            true => I2SSRC::Ckin,
        }
    }
    #[doc = "PLLI2S clock used as I2S clock source"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == I2SSRC::Plli2s
    }
    #[doc = "External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    #[inline(always)]
    pub fn is_ckin(&self) -> bool {
        *self == I2SSRC::Ckin
    }
}
#[doc = "Field `I2SSRC` writer - I2S clock selection"]
pub type I2SSRC_W<'a, REG> = crate::BitWriter<'a, REG, I2SSRC>;
impl<'a, REG> I2SSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLI2S clock used as I2S clock source"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSRC::Plli2s)
    }
    #[doc = "External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    #[inline(always)]
    pub fn ckin(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSRC::Ckin)
    }
}
#[doc = "MCO1 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1PRE {
    #[doc = "0: No division"]
    Div1 = 0,
    #[doc = "4: Division by 2"]
    Div2 = 4,
    #[doc = "5: Division by 3"]
    Div3 = 5,
    #[doc = "6: Division by 4"]
    Div4 = 6,
    #[doc = "7: Division by 5"]
    Div5 = 7,
}
impl From<MCO1PRE> for u8 {
    #[inline(always)]
    fn from(variant: MCO1PRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO1PRE {
    type Ux = u8;
}
#[doc = "Field `MCO1PRE` reader - MCO1 prescaler"]
pub type MCO1PRE_R = crate::FieldReader<MCO1PRE>;
impl MCO1PRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO1PRE> {
        match self.bits {
            0 => Some(MCO1PRE::Div1),
            4 => Some(MCO1PRE::Div2),
            5 => Some(MCO1PRE::Div3),
            6 => Some(MCO1PRE::Div4),
            7 => Some(MCO1PRE::Div5),
            _ => None,
        }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCO1PRE::Div1
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCO1PRE::Div2
    }
    #[doc = "Division by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == MCO1PRE::Div3
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCO1PRE::Div4
    }
    #[doc = "Division by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == MCO1PRE::Div5
    }
}
#[doc = "Field `MCO1PRE` writer - MCO1 prescaler"]
pub type MCO1PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO1PRE>;
impl<'a, REG> MCO1PRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE::Div1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE::Div2)
    }
    #[doc = "Division by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE::Div3)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE::Div4)
    }
    #[doc = "Division by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE::Div5)
    }
}
#[doc = "Field `MCO2PRE` reader - MCO2 prescaler"]
pub use MCO1PRE_R as MCO2PRE_R;
#[doc = "Field `MCO2PRE` writer - MCO2 prescaler"]
pub use MCO1PRE_W as MCO2PRE_W;
#[doc = "Microcontroller clock output 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2 {
    #[doc = "0: System clock (SYSCLK) selected"]
    Sysclk = 0,
    #[doc = "1: PLLI2S clock selected"]
    Plli2s = 1,
    #[doc = "2: HSE oscillator clock selected"]
    Hse = 2,
    #[doc = "3: PLL clock selected"]
    Pll = 3,
}
impl From<MCO2> for u8 {
    #[inline(always)]
    fn from(variant: MCO2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO2 {
    type Ux = u8;
}
#[doc = "Field `MCO2` reader - Microcontroller clock output 2"]
pub type MCO2_R = crate::FieldReader<MCO2>;
impl MCO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCO2 {
        match self.bits {
            0 => MCO2::Sysclk,
            1 => MCO2::Plli2s,
            2 => MCO2::Hse,
            3 => MCO2::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "System clock (SYSCLK) selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2::Sysclk
    }
    #[doc = "PLLI2S clock selected"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == MCO2::Plli2s
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2::Hse
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO2::Pll
    }
}
#[doc = "Field `MCO2` writer - Microcontroller clock output 2"]
pub type MCO2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MCO2>;
impl<'a, REG> MCO2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System clock (SYSCLK) selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Sysclk)
    }
    #[doc = "PLLI2S clock selected"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Plli2s)
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Hse)
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2::Pll)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<CFGRrs> {
        SW_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SWS_W<CFGRrs> {
        SWS_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<CFGRrs> {
        HPRE_W::new(self, 4)
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<CFGRrs> {
        PPRE1_W::new(self, 10)
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<CFGRrs> {
        PPRE2_W::new(self, 13)
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<CFGRrs> {
        RTCPRE_W::new(self, 16)
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    #[must_use]
    pub fn mco1(&mut self) -> MCO1_W<CFGRrs> {
        MCO1_W::new(self, 21)
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2ssrc(&mut self) -> I2SSRC_W<CFGRrs> {
        I2SSRC_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<CFGRrs> {
        MCO1PRE_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<CFGRrs> {
        MCO2PRE_W::new(self, 27)
    }
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    #[must_use]
    pub fn mco2(&mut self) -> MCO2_W<CFGRrs> {
        MCO2_W::new(self, 30)
    }
}
#[doc = "clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
