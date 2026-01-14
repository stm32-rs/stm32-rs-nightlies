///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**System clock switch

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    ///0: MSI selected as system clock
    Msi = 0,
    ///1: HSI selected as system clock
    Hsi = 1,
    ///2: HSE selected as system clock
    Hse = 2,
    ///3: PLL selected as system clock
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
            1 => SW::Hsi,
            2 => SW::Hse,
            3 => SW::Pll,
            _ => unreachable!(),
        }
    }
    ///MSI selected as system clock
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SW::Msi
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
///Field `SW` writer - System clock switch
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SW, crate::Safe>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MSI selected as system clock
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Msi)
    }
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
/**System clock switch status

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR {
    ///0: MSI oscillator used as system clock
    Msi = 0,
    ///1: HSI oscillator used as system clock
    Hsi = 1,
    ///2: HSE used as system clock
    Hse = 2,
    ///3: PLL used as system clock
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
impl crate::IsEnum for SWSR {}
///Field `SWS` reader - System clock switch status
pub type SWS_R = crate::FieldReader<SWSR>;
impl SWS_R {
    ///Get enumerated values variant
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
    ///MSI oscillator used as system clock
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SWSR::Msi
    }
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR::Hsi
    }
    ///HSE used as system clock
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
/**APB1 prescaler

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
///Field `PPRE1` reader - APB1 prescaler
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
///Field `PPRE1` writer - APB1 prescaler
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
///Field `PPRE2` reader - APB2 prescaler
pub use PPRE1_R as PPRE2_R;
///Field `PPRE2` writer - APB2 prescaler
pub use PPRE1_W as PPRE2_W;
/**Microcontroller clock output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL {
    ///0: MCO output disabled, no clock on MCO
    None = 0,
    ///1: SYSCLK system clock selected
    Sysclk = 1,
    ///2: MSI clock selected
    Msi = 2,
    ///3: HSI clock selected
    Hsi = 3,
    ///4: HSE clock selected
    Hse = 4,
    ///5: Main PLL clock selected
    Pll = 5,
    ///6: LSI clock selected
    Lsi = 6,
    ///7: LSE clock selected
    Lse = 7,
    ///8: Internal HSI48 clock selected
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
impl crate::IsEnum for MCOSEL {}
///Field `MCOSEL` reader - Microcontroller clock output
pub type MCOSEL_R = crate::FieldReader<MCOSEL>;
impl MCOSEL_R {
    ///Get enumerated values variant
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
    ///MCO output disabled, no clock on MCO
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MCOSEL::None
    }
    ///SYSCLK system clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCOSEL::Sysclk
    }
    ///MSI clock selected
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == MCOSEL::Msi
    }
    ///HSI clock selected
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCOSEL::Hsi
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCOSEL::Hse
    }
    ///Main PLL clock selected
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCOSEL::Pll
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCOSEL::Lsi
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCOSEL::Lse
    }
    ///Internal HSI48 clock selected
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCOSEL::Hsi48
    }
}
///Field `MCOSEL` writer - Microcontroller clock output
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCOSEL>;
impl<'a, REG> MCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MCO output disabled, no clock on MCO
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::None)
    }
    ///SYSCLK system clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Sysclk)
    }
    ///MSI clock selected
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Msi)
    }
    ///HSI clock selected
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hsi)
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hse)
    }
    ///Main PLL clock selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Pll)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lsi)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lse)
    }
    ///Internal HSI48 clock selected
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hsi48)
    }
}
/**Microcontroller clock output prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE {
    ///0: MCO divided by 1
    Div1 = 0,
    ///1: MCO divided by 2
    Div2 = 1,
    ///2: MCO divided by 4
    Div4 = 2,
    ///3: MCO divided by 8
    Div8 = 3,
    ///4: MCO divided by 16
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
    ///MCO divided by 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRE::Div1
    }
    ///MCO divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRE::Div2
    }
    ///MCO divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRE::Div4
    }
    ///MCO divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRE::Div8
    }
    ///MCO divided by 16
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
    ///MCO divided by 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div1)
    }
    ///MCO divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div2)
    }
    ///MCO divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div4)
    }
    ///MCO divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div8)
    }
    ///MCO divided by 16
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
    ///Bits 8:10 - APB1 prescaler
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - APB2 prescaler
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output
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
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("hpre", &self.hpre())
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("mcosel", &self.mcosel())
            .field("mcopre", &self.mcopre())
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
    ///Bits 8:10 - APB1 prescaler
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<'_, CFGRrs> {
        PPRE1_W::new(self, 8)
    }
    ///Bits 11:13 - APB2 prescaler
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<'_, CFGRrs> {
        PPRE2_W::new(self, 11)
    }
    ///Bits 24:27 - Microcontroller clock output
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#RCC:CFGR)*/
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
///`reset()` method sets CFGR to value 0x05
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x05;
}
