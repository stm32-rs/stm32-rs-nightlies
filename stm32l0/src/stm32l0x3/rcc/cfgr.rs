#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "System clock switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    #[doc = "0: MSI oscillator used as system clock"]
    Msi = 0,
    #[doc = "1: HSI oscillator used as system clock"]
    Hsi16 = 1,
    #[doc = "2: HSE oscillator used as system clock"]
    Hse = 2,
    #[doc = "3: PLL used as system clock"]
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
            1 => SW::Hsi16,
            2 => SW::Hse,
            3 => SW::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "MSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SW::Msi
    }
    #[doc = "HSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SW::Hsi16
    }
    #[doc = "HSE oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW::Hse
    }
    #[doc = "PLL used as system clock"]
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
    #[doc = "MSI oscillator used as system clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Msi)
    }
    #[doc = "HSI oscillator used as system clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hsi16)
    }
    #[doc = "HSE oscillator used as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hse)
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Pll)
    }
}
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWS {
    #[doc = "0: MSI oscillator used as system clock"]
    Msi = 0,
    #[doc = "1: HSI oscillator used as system clock"]
    Hsi16 = 1,
    #[doc = "2: HSE oscillator used as system clock"]
    Hse = 2,
    #[doc = "3: PLL used as system clock"]
    Pll = 3,
}
impl From<SWS> for u8 {
    #[inline(always)]
    fn from(variant: SWS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWS {
    type Ux = u8;
}
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SWS_R = crate::FieldReader<SWS>;
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWS {
        match self.bits {
            0 => SWS::Msi,
            1 => SWS::Hsi16,
            2 => SWS::Hse,
            3 => SWS::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "MSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SWS::Msi
    }
    #[doc = "HSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SWS::Hsi16
    }
    #[doc = "HSE oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWS::Hse
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWS::Pll
    }
}
#[doc = "AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    #[doc = "0: system clock not divided"]
    Div1 = 0,
    #[doc = "8: system clock divided by 2"]
    Div2 = 8,
    #[doc = "9: system clock divided by 4"]
    Div4 = 9,
    #[doc = "10: system clock divided by 8"]
    Div8 = 10,
    #[doc = "11: system clock divided by 16"]
    Div16 = 11,
    #[doc = "12: system clock divided by 64"]
    Div64 = 12,
    #[doc = "13: system clock divided by 128"]
    Div128 = 13,
    #[doc = "14: system clock divided by 256"]
    Div256 = 14,
    #[doc = "15: system clock divided by 512"]
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
    #[doc = "system clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE::Div1
    }
    #[doc = "system clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    #[doc = "system clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    #[doc = "system clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    #[doc = "system clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    #[doc = "system clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    #[doc = "system clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    #[doc = "system clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    #[doc = "system clock divided by 512"]
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
    #[doc = "system clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
    #[doc = "system clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    #[doc = "system clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    #[doc = "system clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    #[doc = "system clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    #[doc = "system clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    #[doc = "system clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    #[doc = "system clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    #[doc = "system clock divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
}
#[doc = "APB low-speed prescaler (APB1)\n\nValue on reset: 0"]
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
#[doc = "Field `PPRE1` reader - APB low-speed prescaler (APB1)"]
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
#[doc = "Field `PPRE1` writer - APB low-speed prescaler (APB1)"]
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
#[doc = "Wake-up from stop clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK {
    #[doc = "0: Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock"]
    Msi = 0,
    #[doc = "1: Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)"]
    Hsi16 = 1,
}
impl From<STOPWUCK> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPWUCK` reader - Wake-up from stop clock selection"]
pub type STOPWUCK_R = crate::BitReader<STOPWUCK>;
impl STOPWUCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPWUCK {
        match self.bits {
            false => STOPWUCK::Msi,
            true => STOPWUCK::Hsi16,
        }
    }
    #[doc = "Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == STOPWUCK::Msi
    }
    #[doc = "Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == STOPWUCK::Hsi16
    }
}
#[doc = "Field `STOPWUCK` writer - Wake-up from stop clock selection"]
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG, STOPWUCK>;
impl<'a, REG> STOPWUCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Msi)
    }
    #[doc = "Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Hsi16)
    }
}
#[doc = "PLL entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSRC {
    #[doc = "0: HSI selected as PLL input clock"]
    Hsi16 = 0,
    #[doc = "1: HSE selected as PLL input clock"]
    Hse = 1,
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
            false => PLLSRC::Hsi16,
            true => PLLSRC::Hse,
        }
    }
    #[doc = "HSI selected as PLL input clock"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC::Hsi16
    }
    #[doc = "HSE selected as PLL input clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC::Hse
    }
}
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PLLSRC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSRC>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi16)
    }
    #[doc = "HSE selected as PLL input clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse)
    }
}
#[doc = "PLL multiplication factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL {
    #[doc = "0: PLL clock entry x 3"]
    Mul3 = 0,
    #[doc = "1: PLL clock entry x 4"]
    Mul4 = 1,
    #[doc = "2: PLL clock entry x 6"]
    Mul6 = 2,
    #[doc = "3: PLL clock entry x 8"]
    Mul8 = 3,
    #[doc = "4: PLL clock entry x 12"]
    Mul12 = 4,
    #[doc = "5: PLL clock entry x 16"]
    Mul16 = 5,
    #[doc = "6: PLL clock entry x 24"]
    Mul24 = 6,
    #[doc = "7: PLL clock entry x 32"]
    Mul32 = 7,
    #[doc = "8: PLL clock entry x 48"]
    Mul48 = 8,
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
#[doc = "Field `PLLMUL` reader - PLL multiplication factor"]
pub type PLLMUL_R = crate::FieldReader<PLLMUL>;
impl PLLMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLMUL> {
        match self.bits {
            0 => Some(PLLMUL::Mul3),
            1 => Some(PLLMUL::Mul4),
            2 => Some(PLLMUL::Mul6),
            3 => Some(PLLMUL::Mul8),
            4 => Some(PLLMUL::Mul12),
            5 => Some(PLLMUL::Mul16),
            6 => Some(PLLMUL::Mul24),
            7 => Some(PLLMUL::Mul32),
            8 => Some(PLLMUL::Mul48),
            _ => None,
        }
    }
    #[doc = "PLL clock entry x 3"]
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMUL::Mul3
    }
    #[doc = "PLL clock entry x 4"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMUL::Mul4
    }
    #[doc = "PLL clock entry x 6"]
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMUL::Mul6
    }
    #[doc = "PLL clock entry x 8"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMUL::Mul8
    }
    #[doc = "PLL clock entry x 12"]
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMUL::Mul12
    }
    #[doc = "PLL clock entry x 16"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMUL::Mul16
    }
    #[doc = "PLL clock entry x 24"]
    #[inline(always)]
    pub fn is_mul24(&self) -> bool {
        *self == PLLMUL::Mul24
    }
    #[doc = "PLL clock entry x 32"]
    #[inline(always)]
    pub fn is_mul32(&self) -> bool {
        *self == PLLMUL::Mul32
    }
    #[doc = "PLL clock entry x 48"]
    #[inline(always)]
    pub fn is_mul48(&self) -> bool {
        *self == PLLMUL::Mul48
    }
}
#[doc = "Field `PLLMUL` writer - PLL multiplication factor"]
pub type PLLMUL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLLMUL>;
impl<'a, REG> PLLMUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL clock entry x 3"]
    #[inline(always)]
    pub fn mul3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul3)
    }
    #[doc = "PLL clock entry x 4"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul4)
    }
    #[doc = "PLL clock entry x 6"]
    #[inline(always)]
    pub fn mul6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul6)
    }
    #[doc = "PLL clock entry x 8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul8)
    }
    #[doc = "PLL clock entry x 12"]
    #[inline(always)]
    pub fn mul12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul12)
    }
    #[doc = "PLL clock entry x 16"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul16)
    }
    #[doc = "PLL clock entry x 24"]
    #[inline(always)]
    pub fn mul24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul24)
    }
    #[doc = "PLL clock entry x 32"]
    #[inline(always)]
    pub fn mul32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul32)
    }
    #[doc = "PLL clock entry x 48"]
    #[inline(always)]
    pub fn mul48(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul48)
    }
}
#[doc = "PLL output division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLDIV {
    #[doc = "1: PLLVCO / 2"]
    Div2 = 1,
    #[doc = "2: PLLVCO / 3"]
    Div3 = 2,
    #[doc = "3: PLLVCO / 4"]
    Div4 = 3,
}
impl From<PLLDIV> for u8 {
    #[inline(always)]
    fn from(variant: PLLDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLDIV {
    type Ux = u8;
}
#[doc = "Field `PLLDIV` reader - PLL output division"]
pub type PLLDIV_R = crate::FieldReader<PLLDIV>;
impl PLLDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLDIV> {
        match self.bits {
            1 => Some(PLLDIV::Div2),
            2 => Some(PLLDIV::Div3),
            3 => Some(PLLDIV::Div4),
            _ => None,
        }
    }
    #[doc = "PLLVCO / 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLDIV::Div2
    }
    #[doc = "PLLVCO / 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLDIV::Div3
    }
    #[doc = "PLLVCO / 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLDIV::Div4
    }
}
#[doc = "Field `PLLDIV` writer - PLL output division"]
pub type PLLDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLDIV>;
impl<'a, REG> PLLDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLVCO / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIV::Div2)
    }
    #[doc = "PLLVCO / 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIV::Div3)
    }
    #[doc = "PLLVCO / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIV::Div4)
    }
}
#[doc = "Microcontroller clock output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI oscillator clock selected"]
    Hsi16 = 2,
    #[doc = "3: MSI oscillator clock selected"]
    Msi = 3,
    #[doc = "4: HSE oscillator clock selected"]
    Hse = 4,
    #[doc = "5: PLL clock selected"]
    Pll = 5,
    #[doc = "6: LSI oscillator clock selected"]
    Lsi = 6,
    #[doc = "7: LSE oscillator clock selected"]
    Lse = 7,
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
#[doc = "Field `MCOSEL` reader - Microcontroller clock output selection"]
pub type MCOSEL_R = crate::FieldReader<MCOSEL>;
impl MCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCOSEL> {
        match self.bits {
            0 => Some(MCOSEL::NoClock),
            1 => Some(MCOSEL::Sysclk),
            2 => Some(MCOSEL::Hsi16),
            3 => Some(MCOSEL::Msi),
            4 => Some(MCOSEL::Hse),
            5 => Some(MCOSEL::Pll),
            6 => Some(MCOSEL::Lsi),
            7 => Some(MCOSEL::Lse),
            _ => None,
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == MCOSEL::NoClock
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCOSEL::Sysclk
    }
    #[doc = "HSI oscillator clock selected"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == MCOSEL::Hsi16
    }
    #[doc = "MSI oscillator clock selected"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == MCOSEL::Msi
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCOSEL::Hse
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCOSEL::Pll
    }
    #[doc = "LSI oscillator clock selected"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCOSEL::Lsi
    }
    #[doc = "LSE oscillator clock selected"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCOSEL::Lse
    }
}
#[doc = "Field `MCOSEL` writer - Microcontroller clock output selection"]
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCOSEL>;
impl<'a, REG> MCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::NoClock)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Sysclk)
    }
    #[doc = "HSI oscillator clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hsi16)
    }
    #[doc = "MSI oscillator clock selected"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Msi)
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hse)
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Pll)
    }
    #[doc = "LSI oscillator clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lsi)
    }
    #[doc = "LSE oscillator clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lse)
    }
}
#[doc = "Microcontroller clock output prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE {
    #[doc = "0: No division"]
    Div1 = 0,
    #[doc = "1: Division by 2"]
    Div2 = 1,
    #[doc = "2: Division by 4"]
    Div4 = 2,
    #[doc = "3: Division by 8"]
    Div8 = 3,
    #[doc = "4: Division by 16"]
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
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRE::Div1
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRE::Div2
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRE::Div4
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRE::Div8
    }
    #[doc = "Division by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MCOPRE::Div16
    }
}
#[doc = "Field `MCOPRE` writer - Microcontroller clock output prescaler"]
pub type MCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCOPRE>;
impl<'a, REG> MCOPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div2)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div4)
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div8)
    }
    #[doc = "Division by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div16)
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
    #[doc = "Bits 8:10 - APB low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 15 - Wake-up from stop clock selection"]
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL multiplication factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - PLL output division"]
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output selection"]
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
    #[doc = "Bits 8:10 - APB low-speed prescaler (APB1)"]
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
    #[doc = "Bit 15 - Wake-up from stop clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<CFGRrs> {
        STOPWUCK_W::new(self, 15)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<CFGRrs> {
        PLLSRC_W::new(self, 16)
    }
    #[doc = "Bits 18:21 - PLL multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<CFGRrs> {
        PLLMUL_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - PLL output division"]
    #[inline(always)]
    #[must_use]
    pub fn plldiv(&mut self) -> PLLDIV_W<CFGRrs> {
        PLLDIV_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output selection"]
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> MCOPRE_W<CFGRrs> {
        MCOPRE_W::new(self, 28)
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
