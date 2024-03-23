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
    #[doc = "1: HSI16 oscillator used as system clock"]
    Hsi16 = 1,
    #[doc = "2: HSE32 oscillator used as system clock"]
    Hse32 = 2,
    #[doc = "3: PLLRCLK used as system clock"]
    Pllr = 3,
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
            2 => SW::Hse32,
            3 => SW::Pllr,
            _ => unreachable!(),
        }
    }
    #[doc = "MSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SW::Msi
    }
    #[doc = "HSI16 oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SW::Hsi16
    }
    #[doc = "HSE32 oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == SW::Hse32
    }
    #[doc = "PLLRCLK used as system clock"]
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == SW::Pllr
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
    #[doc = "HSI16 oscillator used as system clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hsi16)
    }
    #[doc = "HSE32 oscillator used as system clock"]
    #[inline(always)]
    pub fn hse32(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hse32)
    }
    #[doc = "PLLRCLK used as system clock"]
    #[inline(always)]
    pub fn pllr(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Pllr)
    }
}
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWS {
    #[doc = "0: MSI oscillator used as system clock"]
    Msi = 0,
    #[doc = "1: HSI16 oscillator used as system clock"]
    Hsi16 = 1,
    #[doc = "2: HSE32 oscillator used as system clock"]
    Hse32 = 2,
    #[doc = "3: PLLRCLK used as system clock"]
    Pllr = 3,
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
            2 => SWS::Hse32,
            3 => SWS::Pllr,
            _ => unreachable!(),
        }
    }
    #[doc = "MSI oscillator used as system clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SWS::Msi
    }
    #[doc = "HSI16 oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SWS::Hsi16
    }
    #[doc = "HSE32 oscillator used as system clock"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == SWS::Hse32
    }
    #[doc = "PLLRCLK used as system clock"]
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == SWS::Pllr
    }
}
#[doc = "HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    #[doc = "0: SYSCLK not divided"]
    Div1 = 0,
    #[doc = "1: SYSCLK divided by 3"]
    Div3 = 1,
    #[doc = "2: SYSCLK divided by 5"]
    Div5 = 2,
    #[doc = "5: SYSCLK divided by 6"]
    Div6 = 5,
    #[doc = "6: SYSCLK divided by 10"]
    Div10 = 6,
    #[doc = "7: SYSCLK divided by 32"]
    Div32 = 7,
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
    #[doc = "14: SYSCLK divided by 128"]
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
#[doc = "Field `HPRE` reader - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HPRE> {
        match self.bits {
            0 => Some(HPRE::Div1),
            1 => Some(HPRE::Div3),
            2 => Some(HPRE::Div5),
            5 => Some(HPRE::Div6),
            6 => Some(HPRE::Div10),
            7 => Some(HPRE::Div32),
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
    #[doc = "SYSCLK divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == HPRE::Div3
    }
    #[doc = "SYSCLK divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == HPRE::Div5
    }
    #[doc = "SYSCLK divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == HPRE::Div6
    }
    #[doc = "SYSCLK divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == HPRE::Div10
    }
    #[doc = "SYSCLK divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == HPRE::Div32
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
    #[doc = "SYSCLK divided by 128"]
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
#[doc = "Field `HPRE` writer - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
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
    #[doc = "SYSCLK divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div3)
    }
    #[doc = "SYSCLK divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div5)
    }
    #[doc = "SYSCLK divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div6)
    }
    #[doc = "SYSCLK divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div10)
    }
    #[doc = "SYSCLK divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div32)
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
    #[doc = "SYSCLK divided by 128"]
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
#[doc = "PCLK1 low-speed prescaler (APB1)\n\nValue on reset: 0"]
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
#[doc = "Field `PPRE1` reader - PCLK1 low-speed prescaler (APB1)"]
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
#[doc = "Field `PPRE1` writer - PCLK1 low-speed prescaler (APB1)"]
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
#[doc = "PCLK2 high-speed prescaler (APB2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE2 {
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
impl From<PPRE2> for u8 {
    #[inline(always)]
    fn from(variant: PPRE2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE2 {
    type Ux = u8;
}
#[doc = "Field `PPRE2` reader - PCLK2 high-speed prescaler (APB2)"]
pub type PPRE2_R = crate::FieldReader<PPRE2>;
impl PPRE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PPRE2> {
        match self.bits {
            0 => Some(PPRE2::Div1),
            4 => Some(PPRE2::Div2),
            5 => Some(PPRE2::Div4),
            6 => Some(PPRE2::Div8),
            7 => Some(PPRE2::Div16),
            _ => None,
        }
    }
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE2::Div1
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE2::Div2
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE2::Div4
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE2::Div8
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE2::Div16
    }
}
#[doc = "Field `PPRE2` writer - PCLK2 high-speed prescaler (APB2)"]
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE2>;
impl<'a, REG> PPRE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE2::Div1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE2::Div2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE2::Div4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE2::Div8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE2::Div16)
    }
}
#[doc = "Wakeup from Stop and CSS backup clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK {
    #[doc = "0: MSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    Msi = 0,
    #[doc = "1: HSI16 oscillator selected as wakeup from stop clock and CSS backup clock"]
    Hsi16 = 1,
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
            true => STOPWUCK::Hsi16,
        }
    }
    #[doc = "MSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == STOPWUCK::Msi
    }
    #[doc = "HSI16 oscillator selected as wakeup from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == STOPWUCK::Hsi16
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
    #[doc = "HSI16 oscillator selected as wakeup from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Hsi16)
    }
}
#[doc = "HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPREF {
    #[doc = "0: HCLK1 prescaler value not yet applied"]
    NotApplied = 0,
    #[doc = "1: HCLK1 prescaler value applied"]
    Applied = 1,
}
impl From<HPREF> for bool {
    #[inline(always)]
    fn from(variant: HPREF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPREF` reader - HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)"]
pub type HPREF_R = crate::BitReader<HPREF>;
impl HPREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPREF {
        match self.bits {
            false => HPREF::NotApplied,
            true => HPREF::Applied,
        }
    }
    #[doc = "HCLK1 prescaler value not yet applied"]
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == HPREF::NotApplied
    }
    #[doc = "HCLK1 prescaler value applied"]
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == HPREF::Applied
    }
}
#[doc = "PCLK1 prescaler flag (APB1)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPRE1F {
    #[doc = "0: PCLK1 prescaler value not yet applied"]
    NotApplied = 0,
    #[doc = "1: PCLK1 prescaler value applied"]
    Applied = 1,
}
impl From<PPRE1F> for bool {
    #[inline(always)]
    fn from(variant: PPRE1F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPRE1F` reader - PCLK1 prescaler flag (APB1)"]
pub type PPRE1F_R = crate::BitReader<PPRE1F>;
impl PPRE1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPRE1F {
        match self.bits {
            false => PPRE1F::NotApplied,
            true => PPRE1F::Applied,
        }
    }
    #[doc = "PCLK1 prescaler value not yet applied"]
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == PPRE1F::NotApplied
    }
    #[doc = "PCLK1 prescaler value applied"]
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == PPRE1F::Applied
    }
}
#[doc = "PCLK2 prescaler flag (APB2)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPRE2F {
    #[doc = "0: PCLK2 prescaler value not yet applied"]
    NotApplied = 0,
    #[doc = "1: PCLK2 prescaler value applied"]
    Applied = 1,
}
impl From<PPRE2F> for bool {
    #[inline(always)]
    fn from(variant: PPRE2F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPRE2F` reader - PCLK2 prescaler flag (APB2)"]
pub type PPRE2F_R = crate::BitReader<PPRE2F>;
impl PPRE2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPRE2F {
        match self.bits {
            false => PPRE2F::NotApplied,
            true => PPRE2F::Applied,
        }
    }
    #[doc = "PCLK2 prescaler value not yet applied"]
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == PPRE2F::NotApplied
    }
    #[doc = "PCLK2 prescaler value applied"]
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == PPRE2F::Applied
    }
}
#[doc = "Microcontroller clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: MSI oscillator clock selected"]
    Msi = 2,
    #[doc = "3: HSI16 oscillator clock selected"]
    Hsi16 = 3,
    #[doc = "4: HSE32 oscillator clock selected"]
    Hse32 = 4,
    #[doc = "5: Main PLLRCLK clock selected"]
    Pllr = 5,
    #[doc = "6: LSI oscillator clock selected"]
    Lsi = 6,
    #[doc = "8: LSE oscillator clock selected"]
    Lse = 8,
    #[doc = "13: Main PLLPCLK clock selected"]
    Pllp = 13,
    #[doc = "14: Main PLLQCLK clock selected"]
    Pllq = 14,
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
            0 => Some(MCOSEL::NoClock),
            1 => Some(MCOSEL::Sysclk),
            2 => Some(MCOSEL::Msi),
            3 => Some(MCOSEL::Hsi16),
            4 => Some(MCOSEL::Hse32),
            5 => Some(MCOSEL::Pllr),
            6 => Some(MCOSEL::Lsi),
            8 => Some(MCOSEL::Lse),
            13 => Some(MCOSEL::Pllp),
            14 => Some(MCOSEL::Pllq),
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
    #[doc = "MSI oscillator clock selected"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == MCOSEL::Msi
    }
    #[doc = "HSI16 oscillator clock selected"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == MCOSEL::Hsi16
    }
    #[doc = "HSE32 oscillator clock selected"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == MCOSEL::Hse32
    }
    #[doc = "Main PLLRCLK clock selected"]
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == MCOSEL::Pllr
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
    #[doc = "Main PLLPCLK clock selected"]
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == MCOSEL::Pllp
    }
    #[doc = "Main PLLQCLK clock selected"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == MCOSEL::Pllq
    }
}
#[doc = "Field `MCOSEL` writer - Microcontroller clock output"]
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
    #[doc = "MSI oscillator clock selected"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Msi)
    }
    #[doc = "HSI16 oscillator clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hsi16)
    }
    #[doc = "HSE32 oscillator clock selected"]
    #[inline(always)]
    pub fn hse32(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hse32)
    }
    #[doc = "Main PLLRCLK clock selected"]
    #[inline(always)]
    pub fn pllr(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Pllr)
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
    #[doc = "Main PLLPCLK clock selected"]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Pllp)
    }
    #[doc = "Main PLLQCLK clock selected"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Pllq)
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
    #[doc = "Bits 4:7 - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - PCLK1 low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - PCLK2 high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)"]
    #[inline(always)]
    pub fn hpref(&self) -> HPREF_R {
        HPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PCLK1 prescaler flag (APB1)"]
    #[inline(always)]
    pub fn ppre1f(&self) -> PPRE1F_R {
        PPRE1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCLK2 prescaler flag (APB2)"]
    #[inline(always)]
    pub fn ppre2f(&self) -> PPRE2F_R {
        PPRE2F_R::new(((self.bits >> 18) & 1) != 0)
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
    #[doc = "Bits 4:7 - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<CFGRrs> {
        HPRE_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - PCLK1 low-speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<CFGRrs> {
        PPRE1_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - PCLK2 high-speed prescaler (APB2)"]
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
#[doc = "`reset()` method sets CFGR to value 0x0007_0000"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x0007_0000;
}
