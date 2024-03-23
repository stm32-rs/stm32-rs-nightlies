#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "System clock switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    #[doc = "0: MSI selected as system clock"]
    Msi = 0,
    #[doc = "1: HSI selected as system clock"]
    Hsi = 1,
    #[doc = "2: HSE selected as system clock"]
    Hse = 2,
    #[doc = "3: PLL selected as system clock"]
    Pll = 3,
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
    pub const fn variant(&self) -> SW {
        match self.bits {
            0 => SW::Msi,
            1 => SW::Hsi,
            2 => SW::Hse,
            3 => SW::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "MSI selected as system clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SW::Msi
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
pub type SW_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SW>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MSI selected as system clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Msi)
    }
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
    #[doc = "0: MSI oscillator used as system clock"]
    Msi = 0,
    #[doc = "1: HSI oscillator used as system clock"]
    Hsi = 1,
    #[doc = "2: HSE used as system clock"]
    Hse = 2,
    #[doc = "3: PLL used as system clock"]
    Pll = 3,
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
    pub const fn variant(&self) -> SWSR {
        match self.bits {
            0 => SWSR::Msi,
            1 => SWSR::Hsi,
            2 => SWSR::Hse,
            3 => SWSR::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "MSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SWSR::Msi
    }
    #[doc = "HSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR::Hsi
    }
    #[doc = "HSE used as system clock"]
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
#[doc = "PB low-speed prescaler (APB1)\n\nValue on reset: 0"]
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
#[doc = "Field `PPRE1` reader - PB low-speed prescaler (APB1)"]
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
#[doc = "Field `PPRE1` writer - PB low-speed prescaler (APB1)"]
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
#[doc = "Wakeup from Stop and CSS backup clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK {
    #[doc = "0: MSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    Msi = 0,
    #[doc = "1: HSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    Hsi = 1,
}
impl From<STOPWUCK> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPWUCK` reader - Wakeup from Stop and CSS backup clock selection"]
pub type STOPWUCK_R = crate::BitReader<STOPWUCK>;
impl STOPWUCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPWUCK {
        match self.bits {
            false => STOPWUCK::Msi,
            true => STOPWUCK::Hsi,
        }
    }
    #[doc = "MSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == STOPWUCK::Msi
    }
    #[doc = "HSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == STOPWUCK::Hsi
    }
}
#[doc = "Field `STOPWUCK` writer - Wakeup from Stop and CSS backup clock selection"]
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG, STOPWUCK>;
impl<'a, REG> STOPWUCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Msi)
    }
    #[doc = "HSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Hsi)
    }
}
#[doc = "Microcontroller clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL {
    #[doc = "0: MCO output disabled, no clock on MCO"]
    None = 0,
    #[doc = "1: SYSCLK system clock selected"]
    Sysclk = 1,
    #[doc = "2: MSI clock selected"]
    Msi = 2,
    #[doc = "3: HSI clock selected"]
    Hsi = 3,
    #[doc = "4: HSE clock selected"]
    Hse = 4,
    #[doc = "5: Main PLL clock selected"]
    Pll = 5,
    #[doc = "6: LSI clock selected"]
    Lsi = 6,
    #[doc = "7: LSE clock selected"]
    Lse = 7,
    #[doc = "8: Internal HSI48 clock selected"]
    Hsi48 = 8,
}
impl From<MCOSEL> for u8 {
    #[inline(always)]
    fn from(variant: MCOSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCOSEL {
    type Ux = u8;
}
#[doc = "Field `MCOSEL` reader - Microcontroller clock output"]
pub type MCOSEL_R = crate::FieldReader<MCOSEL>;
impl MCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCOSEL> {
        match self.bits {
            0 => Some(MCOSEL::None),
            1 => Some(MCOSEL::Sysclk),
            2 => Some(MCOSEL::Msi),
            3 => Some(MCOSEL::Hsi),
            4 => Some(MCOSEL::Hse),
            5 => Some(MCOSEL::Pll),
            6 => Some(MCOSEL::Lsi),
            7 => Some(MCOSEL::Lse),
            8 => Some(MCOSEL::Hsi48),
            _ => None,
        }
    }
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MCOSEL::None
    }
    #[doc = "SYSCLK system clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCOSEL::Sysclk
    }
    #[doc = "MSI clock selected"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == MCOSEL::Msi
    }
    #[doc = "HSI clock selected"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCOSEL::Hsi
    }
    #[doc = "HSE clock selected"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCOSEL::Hse
    }
    #[doc = "Main PLL clock selected"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCOSEL::Pll
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCOSEL::Lsi
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCOSEL::Lse
    }
    #[doc = "Internal HSI48 clock selected"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCOSEL::Hsi48
    }
}
#[doc = "Field `MCOSEL` writer - Microcontroller clock output"]
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCOSEL>;
impl<'a, REG> MCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::None)
    }
    #[doc = "SYSCLK system clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Sysclk)
    }
    #[doc = "MSI clock selected"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Msi)
    }
    #[doc = "HSI clock selected"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hsi)
    }
    #[doc = "HSE clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hse)
    }
    #[doc = "Main PLL clock selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Pll)
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lsi)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lse)
    }
    #[doc = "Internal HSI48 clock selected"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hsi48)
    }
}
#[doc = "Microcontroller clock output prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE {
    #[doc = "0: MCO divided by 1"]
    Div1 = 0,
    #[doc = "1: MCO divided by 2"]
    Div2 = 1,
    #[doc = "2: MCO divided by 4"]
    Div4 = 2,
    #[doc = "3: MCO divided by 8"]
    Div8 = 3,
    #[doc = "4: MCO divided by 16"]
    Div16 = 4,
}
impl From<MCOPRE> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCOPRE {
    type Ux = u8;
}
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler"]
pub type MCOPRE_R = crate::FieldReader<MCOPRE>;
impl MCOPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCOPRE> {
        match self.bits {
            0 => Some(MCOPRE::Div1),
            1 => Some(MCOPRE::Div2),
            2 => Some(MCOPRE::Div4),
            3 => Some(MCOPRE::Div8),
            4 => Some(MCOPRE::Div16),
            _ => None,
        }
    }
    #[doc = "MCO divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRE::Div1
    }
    #[doc = "MCO divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRE::Div2
    }
    #[doc = "MCO divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRE::Div4
    }
    #[doc = "MCO divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRE::Div8
    }
    #[doc = "MCO divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MCOPRE::Div16
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
    #[doc = "Bits 8:10 - PB low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
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
    #[doc = "Bits 8:10 - PB low-speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<CFGRrs> {
        PPRE1_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<CFGRrs> {
        PPRE2_W::new(self, 11)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<CFGRrs> {
        STOPWUCK_W::new(self, 15)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
