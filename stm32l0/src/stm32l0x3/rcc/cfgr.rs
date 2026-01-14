///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**System clock switch

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    ///0: MSI oscillator used as system clock
    Msi = 0,
    ///1: HSI oscillator used as system clock
    Hsi16 = 1,
    ///2: HSE oscillator used as system clock
    Hse = 2,
    ///3: PLL used as system clock
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
impl crate::IsEnum for SW {}
///Field `SW` reader - System clock switch
pub type SW_R = crate::FieldReader<SW>;
impl SW_R {
    ///Get enumerated values variant
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
    ///MSI oscillator used as system clock
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SW::Msi
    }
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SW::Hsi16
    }
    ///HSE oscillator used as system clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW::Hse
    }
    ///PLL used as system clock
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SW::Pll
    }
}
///Field `SW` writer - System clock switch
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SW, crate::Safe>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MSI oscillator used as system clock
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Msi)
    }
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hsi16)
    }
    ///HSE oscillator used as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hse)
    }
    ///PLL used as system clock
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Pll)
    }
}
/**System clock switch status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWS {
    ///0: MSI oscillator used as system clock
    Msi = 0,
    ///1: HSI oscillator used as system clock
    Hsi16 = 1,
    ///2: HSE oscillator used as system clock
    Hse = 2,
    ///3: PLL used as system clock
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
impl crate::IsEnum for SWS {}
///Field `SWS` reader - System clock switch status
pub type SWS_R = crate::FieldReader<SWS>;
impl SWS_R {
    ///Get enumerated values variant
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
    ///MSI oscillator used as system clock
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SWS::Msi
    }
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SWS::Hsi16
    }
    ///HSE oscillator used as system clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWS::Hse
    }
    ///PLL used as system clock
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWS::Pll
    }
}
/**AHB prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    ///8: system clock divided by 2
    Div2 = 8,
    ///9: system clock divided by 4
    Div4 = 9,
    ///10: system clock divided by 8
    Div8 = 10,
    ///11: system clock divided by 16
    Div16 = 11,
    ///12: system clock divided by 64
    Div64 = 12,
    ///13: system clock divided by 128
    Div128 = 13,
    ///14: system clock divided by 256
    Div256 = 14,
    ///15: system clock divided by 512
    Div512 = 15,
    ///0: system clock not divided
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
    ///system clock divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    ///system clock divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    ///system clock divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    ///system clock divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    ///system clock divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    ///system clock divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    ///system clock divided by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    ///system clock divided by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE::Div512
    }
    ///system clock not divided
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
    ///system clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    ///system clock divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    ///system clock divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    ///system clock divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    ///system clock divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    ///system clock divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    ///system clock divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    ///system clock divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
    ///system clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
}
/**APB low-speed prescaler (APB1)

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
///Field `PPRE1` reader - APB low-speed prescaler (APB1)
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
///Field `PPRE1` writer - APB low-speed prescaler (APB1)
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
///Field `PPRE2` reader - APB high-speed prescaler (APB2)
pub use PPRE1_R as PPRE2_R;
///Field `PPRE2` writer - APB high-speed prescaler (APB2)
pub use PPRE1_W as PPRE2_W;
/**Wake-up from stop clock selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK {
    ///0: Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock
    Msi = 0,
    ///1: Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)
    Hsi16 = 1,
}
impl From<STOPWUCK> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPWUCK` reader - Wake-up from stop clock selection
pub type STOPWUCK_R = crate::BitReader<STOPWUCK>;
impl STOPWUCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOPWUCK {
        match self.bits {
            false => STOPWUCK::Msi,
            true => STOPWUCK::Hsi16,
        }
    }
    ///Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == STOPWUCK::Msi
    }
    ///Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == STOPWUCK::Hsi16
    }
}
///Field `STOPWUCK` writer - Wake-up from stop clock selection
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG, STOPWUCK>;
impl<'a, REG> STOPWUCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Msi)
    }
    ///Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Hsi16)
    }
}
/**PLL entry clock source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSRC {
    ///0: HSI selected as PLL input clock
    Hsi16 = 0,
    ///1: HSE selected as PLL input clock
    Hse = 1,
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
            false => PLLSRC::Hsi16,
            true => PLLSRC::Hse,
        }
    }
    ///HSI selected as PLL input clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC::Hsi16
    }
    ///HSE selected as PLL input clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC::Hse
    }
}
///Field `PLLSRC` writer - PLL entry clock source
pub type PLLSRC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSRC>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI selected as PLL input clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi16)
    }
    ///HSE selected as PLL input clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse)
    }
}
/**PLL multiplication factor

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL {
    ///0: PLL clock entry x 3
    Mul3 = 0,
    ///1: PLL clock entry x 4
    Mul4 = 1,
    ///2: PLL clock entry x 6
    Mul6 = 2,
    ///3: PLL clock entry x 8
    Mul8 = 3,
    ///4: PLL clock entry x 12
    Mul12 = 4,
    ///5: PLL clock entry x 16
    Mul16 = 5,
    ///6: PLL clock entry x 24
    Mul24 = 6,
    ///7: PLL clock entry x 32
    Mul32 = 7,
    ///8: PLL clock entry x 48
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
impl crate::IsEnum for PLLMUL {}
///Field `PLLMUL` reader - PLL multiplication factor
pub type PLLMUL_R = crate::FieldReader<PLLMUL>;
impl PLLMUL_R {
    ///Get enumerated values variant
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
    ///PLL clock entry x 3
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMUL::Mul3
    }
    ///PLL clock entry x 4
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMUL::Mul4
    }
    ///PLL clock entry x 6
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMUL::Mul6
    }
    ///PLL clock entry x 8
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMUL::Mul8
    }
    ///PLL clock entry x 12
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMUL::Mul12
    }
    ///PLL clock entry x 16
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMUL::Mul16
    }
    ///PLL clock entry x 24
    #[inline(always)]
    pub fn is_mul24(&self) -> bool {
        *self == PLLMUL::Mul24
    }
    ///PLL clock entry x 32
    #[inline(always)]
    pub fn is_mul32(&self) -> bool {
        *self == PLLMUL::Mul32
    }
    ///PLL clock entry x 48
    #[inline(always)]
    pub fn is_mul48(&self) -> bool {
        *self == PLLMUL::Mul48
    }
}
///Field `PLLMUL` writer - PLL multiplication factor
pub type PLLMUL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLLMUL>;
impl<'a, REG> PLLMUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL clock entry x 3
    #[inline(always)]
    pub fn mul3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul3)
    }
    ///PLL clock entry x 4
    #[inline(always)]
    pub fn mul4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul4)
    }
    ///PLL clock entry x 6
    #[inline(always)]
    pub fn mul6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul6)
    }
    ///PLL clock entry x 8
    #[inline(always)]
    pub fn mul8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul8)
    }
    ///PLL clock entry x 12
    #[inline(always)]
    pub fn mul12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul12)
    }
    ///PLL clock entry x 16
    #[inline(always)]
    pub fn mul16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul16)
    }
    ///PLL clock entry x 24
    #[inline(always)]
    pub fn mul24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul24)
    }
    ///PLL clock entry x 32
    #[inline(always)]
    pub fn mul32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul32)
    }
    ///PLL clock entry x 48
    #[inline(always)]
    pub fn mul48(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL::Mul48)
    }
}
/**PLL output division

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLDIV {
    ///1: PLLVCO / 2
    Div2 = 1,
    ///2: PLLVCO / 3
    Div3 = 2,
    ///3: PLLVCO / 4
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
impl crate::IsEnum for PLLDIV {}
///Field `PLLDIV` reader - PLL output division
pub type PLLDIV_R = crate::FieldReader<PLLDIV>;
impl PLLDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLDIV> {
        match self.bits {
            1 => Some(PLLDIV::Div2),
            2 => Some(PLLDIV::Div3),
            3 => Some(PLLDIV::Div4),
            _ => None,
        }
    }
    ///PLLVCO / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLDIV::Div2
    }
    ///PLLVCO / 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLDIV::Div3
    }
    ///PLLVCO / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLDIV::Div4
    }
}
///Field `PLLDIV` writer - PLL output division
pub type PLLDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLDIV>;
impl<'a, REG> PLLDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLVCO / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIV::Div2)
    }
    ///PLLVCO / 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIV::Div3)
    }
    ///PLLVCO / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIV::Div4)
    }
}
/**Microcontroller clock output selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL {
    ///0: No clock
    NoClock = 0,
    ///1: SYSCLK clock selected
    Sysclk = 1,
    ///2: HSI oscillator clock selected
    Hsi16 = 2,
    ///3: MSI oscillator clock selected
    Msi = 3,
    ///4: HSE oscillator clock selected
    Hse = 4,
    ///5: PLL clock selected
    Pll = 5,
    ///6: LSI oscillator clock selected
    Lsi = 6,
    ///7: LSE oscillator clock selected
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
impl crate::IsEnum for MCOSEL {}
///Field `MCOSEL` reader - Microcontroller clock output selection
pub type MCOSEL_R = crate::FieldReader<MCOSEL>;
impl MCOSEL_R {
    ///Get enumerated values variant
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
    ///No clock
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == MCOSEL::NoClock
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCOSEL::Sysclk
    }
    ///HSI oscillator clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == MCOSEL::Hsi16
    }
    ///MSI oscillator clock selected
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == MCOSEL::Msi
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCOSEL::Hse
    }
    ///PLL clock selected
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCOSEL::Pll
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCOSEL::Lsi
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCOSEL::Lse
    }
}
///Field `MCOSEL` writer - Microcontroller clock output selection
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCOSEL>;
impl<'a, REG> MCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::NoClock)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Sysclk)
    }
    ///HSI oscillator clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hsi16)
    }
    ///MSI oscillator clock selected
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Msi)
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hse)
    }
    ///PLL clock selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Pll)
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lsi)
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lse)
    }
}
/**Microcontroller clock output prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE {
    ///0: No division
    Div1 = 0,
    ///1: Division by 2
    Div2 = 1,
    ///2: Division by 4
    Div4 = 2,
    ///3: Division by 8
    Div8 = 3,
    ///4: Division by 16
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
impl crate::IsEnum for MCOPRE {}
///Field `MCOPRE` reader - Microcontroller clock output prescaler
pub type MCOPRE_R = crate::FieldReader<MCOPRE>;
impl MCOPRE_R {
    ///Get enumerated values variant
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
    ///No division
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRE::Div1
    }
    ///Division by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRE::Div2
    }
    ///Division by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRE::Div4
    }
    ///Division by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRE::Div8
    }
    ///Division by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MCOPRE::Div16
    }
}
///Field `MCOPRE` writer - Microcontroller clock output prescaler
pub type MCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCOPRE>;
impl<'a, REG> MCOPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No division
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div1)
    }
    ///Division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div2)
    }
    ///Division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div4)
    }
    ///Division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div8)
    }
    ///Division by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div16)
    }
}
impl R {
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - System clock switch status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - APB low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 15 - Wake-up from stop clock selection
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - PLL entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 18:21 - PLL multiplication factor
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:23 - PLL output division
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output selection
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("mcopre", &self.mcopre())
            .field("mcosel", &self.mcosel())
            .field("plldiv", &self.plldiv())
            .field("pllmul", &self.pllmul())
            .field("pllsrc", &self.pllsrc())
            .field("stopwuck", &self.stopwuck())
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("hpre", &self.hpre())
            .field("sws", &self.sws())
            .field("sw", &self.sw())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, CFGRrs> {
        SW_W::new(self, 0)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, CFGRrs> {
        HPRE_W::new(self, 4)
    }
    ///Bits 8:10 - APB low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<'_, CFGRrs> {
        PPRE1_W::new(self, 8)
    }
    ///Bits 11:13 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<'_, CFGRrs> {
        PPRE2_W::new(self, 11)
    }
    ///Bit 15 - Wake-up from stop clock selection
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<'_, CFGRrs> {
        STOPWUCK_W::new(self, 15)
    }
    ///Bit 16 - PLL entry clock source
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<'_, CFGRrs> {
        PLLSRC_W::new(self, 16)
    }
    ///Bits 18:21 - PLL multiplication factor
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W<'_, CFGRrs> {
        PLLMUL_W::new(self, 18)
    }
    ///Bits 22:23 - PLL output division
    #[inline(always)]
    pub fn plldiv(&mut self) -> PLLDIV_W<'_, CFGRrs> {
        PLLDIV_W::new(self, 22)
    }
    ///Bits 24:27 - Microcontroller clock output selection
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<'_, CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
    ///Bits 28:30 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W<'_, CFGRrs> {
        MCOPRE_W::new(self, 28)
    }
}
/**Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#RCC:CFGR)*/
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
