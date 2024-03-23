#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PLLCFGRrs>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PLLCFGRrs>;
#[doc = "Main PLL entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC {
    #[doc = "0: No clock sent to PLL"]
    NoClock = 0,
    #[doc = "1: MSI clock selected as PLL clock entry"]
    Msi = 1,
    #[doc = "2: HSI16 clock selected as PLL clock entry"]
    Hsi16 = 2,
    #[doc = "3: HSE32 clock selected as PLL clock entry"]
    Hse32 = 3,
}
impl From<PLLSRC> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSRC {
    type Ux = u8;
}
#[doc = "Field `PLLSRC` reader - Main PLL entry clock source"]
pub type PLLSRC_R = crate::FieldReader<PLLSRC>;
impl PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSRC {
        match self.bits {
            0 => PLLSRC::NoClock,
            1 => PLLSRC::Msi,
            2 => PLLSRC::Hsi16,
            3 => PLLSRC::Hse32,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PLLSRC::NoClock
    }
    #[doc = "MSI clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == PLLSRC::Msi
    }
    #[doc = "HSI16 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC::Hsi16
    }
    #[doc = "HSE32 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == PLLSRC::Hse32
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL entry clock source"]
pub type PLLSRC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLLSRC>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::NoClock)
    }
    #[doc = "MSI clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Msi)
    }
    #[doc = "HSI16 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi16)
    }
    #[doc = "HSE32 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn hse32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse32)
    }
}
#[doc = "Division factor for the main PLL input clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLM {
    #[doc = "0: VCO input = PLL input / PLLM"]
    Div1 = 0,
    #[doc = "1: VCO input = PLL input / PLLM"]
    Div2 = 1,
    #[doc = "2: VCO input = PLL input / PLLM"]
    Div3 = 2,
    #[doc = "3: VCO input = PLL input / PLLM"]
    Div4 = 3,
    #[doc = "4: VCO input = PLL input / PLLM"]
    Div5 = 4,
    #[doc = "5: VCO input = PLL input / PLLM"]
    Div6 = 5,
    #[doc = "6: VCO input = PLL input / PLLM"]
    Div7 = 6,
    #[doc = "7: VCO input = PLL input / PLLM"]
    Div8 = 7,
}
impl From<PLLM> for u8 {
    #[inline(always)]
    fn from(variant: PLLM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLM {
    type Ux = u8;
}
#[doc = "Field `PLLM` reader - Division factor for the main PLL input clock"]
pub type PLLM_R = crate::FieldReader<PLLM>;
impl PLLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLM {
        match self.bits {
            0 => PLLM::Div1,
            1 => PLLM::Div2,
            2 => PLLM::Div3,
            3 => PLLM::Div4,
            4 => PLLM::Div5,
            5 => PLLM::Div6,
            6 => PLLM::Div7,
            7 => PLLM::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLM::Div1
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLM::Div2
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLM::Div3
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLM::Div4
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLM::Div5
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLM::Div6
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLM::Div7
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLM::Div8
    }
}
#[doc = "Field `PLLM` writer - Division factor for the main PLL input clock"]
pub type PLLM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PLLM>;
impl<'a, REG> PLLM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div1)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div2)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div3)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div4)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div5)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div6)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div7)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div8)
    }
}
#[doc = "Field `PLLN` reader - Main PLL multiplication factor for VCO"]
pub type PLLN_R = crate::FieldReader;
#[doc = "Field `PLLN` writer - Main PLL multiplication factor for VCO"]
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Main PLL PLLPCLK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPEN {
    #[doc = "0: PLLCLK output disabled"]
    Disabled = 0,
    #[doc = "1: PLLCLK output enabled"]
    Enabled = 1,
}
impl From<PLLPEN> for bool {
    #[inline(always)]
    fn from(variant: PLLPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPEN` reader - Main PLL PLLPCLK output enable"]
pub type PLLPEN_R = crate::BitReader<PLLPEN>;
impl PLLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLPEN {
        match self.bits {
            false => PLLPEN::Disabled,
            true => PLLPEN::Enabled,
        }
    }
    #[doc = "PLLCLK output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLPEN::Disabled
    }
    #[doc = "PLLCLK output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLPEN::Enabled
    }
}
#[doc = "Field `PLLPEN` writer - Main PLL PLLPCLK output enable"]
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLPEN>;
impl<'a, REG> PLLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLCLK output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPEN::Disabled)
    }
    #[doc = "PLLCLK output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPEN::Enabled)
    }
}
#[doc = "Main PLL division factor for PLLPCLK.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLP {
    #[doc = "1: PLL = VCO/(N+1)"]
    Div2 = 1,
    #[doc = "2: PLL = VCO/(N+1)"]
    Div3 = 2,
    #[doc = "3: PLL = VCO/(N+1)"]
    Div4 = 3,
    #[doc = "4: PLL = VCO/(N+1)"]
    Div5 = 4,
    #[doc = "5: PLL = VCO/(N+1)"]
    Div6 = 5,
    #[doc = "6: PLL = VCO/(N+1)"]
    Div7 = 6,
    #[doc = "7: PLL = VCO/(N+1)"]
    Div8 = 7,
    #[doc = "8: PLL = VCO/(N+1)"]
    Div9 = 8,
    #[doc = "9: PLL = VCO/(N+1)"]
    Div10 = 9,
    #[doc = "10: PLL = VCO/(N+1)"]
    Div11 = 10,
    #[doc = "11: PLL = VCO/(N+1)"]
    Div12 = 11,
    #[doc = "12: PLL = VCO/(N+1)"]
    Div13 = 12,
    #[doc = "13: PLL = VCO/(N+1)"]
    Div14 = 13,
    #[doc = "14: PLL = VCO/(N+1)"]
    Div15 = 14,
    #[doc = "15: PLL = VCO/(N+1)"]
    Div16 = 15,
    #[doc = "16: PLL = VCO/(N+1)"]
    Div17 = 16,
    #[doc = "17: PLL = VCO/(N+1)"]
    Div18 = 17,
    #[doc = "18: PLL = VCO/(N+1)"]
    Div19 = 18,
    #[doc = "19: PLL = VCO/(N+1)"]
    Div20 = 19,
    #[doc = "20: PLL = VCO/(N+1)"]
    Div21 = 20,
    #[doc = "21: PLL = VCO/(N+1)"]
    Div22 = 21,
    #[doc = "22: PLL = VCO/(N+1)"]
    Div23 = 22,
    #[doc = "23: PLL = VCO/(N+1)"]
    Div24 = 23,
    #[doc = "24: PLL = VCO/(N+1)"]
    Div25 = 24,
    #[doc = "25: PLL = VCO/(N+1)"]
    Div26 = 25,
    #[doc = "26: PLL = VCO/(N+1)"]
    Div27 = 26,
    #[doc = "27: PLL = VCO/(N+1)"]
    Div28 = 27,
    #[doc = "28: PLL = VCO/(N+1)"]
    Div29 = 28,
    #[doc = "29: PLL = VCO/(N+1)"]
    Div30 = 29,
    #[doc = "30: PLL = VCO/(N+1)"]
    Div31 = 30,
    #[doc = "31: PLL = VCO/(N+1)"]
    Div32 = 31,
}
impl From<PLLP> for u8 {
    #[inline(always)]
    fn from(variant: PLLP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLP {
    type Ux = u8;
}
#[doc = "Field `PLLP` reader - Main PLL division factor for PLLPCLK."]
pub type PLLP_R = crate::FieldReader<PLLP>;
impl PLLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLP> {
        match self.bits {
            1 => Some(PLLP::Div2),
            2 => Some(PLLP::Div3),
            3 => Some(PLLP::Div4),
            4 => Some(PLLP::Div5),
            5 => Some(PLLP::Div6),
            6 => Some(PLLP::Div7),
            7 => Some(PLLP::Div8),
            8 => Some(PLLP::Div9),
            9 => Some(PLLP::Div10),
            10 => Some(PLLP::Div11),
            11 => Some(PLLP::Div12),
            12 => Some(PLLP::Div13),
            13 => Some(PLLP::Div14),
            14 => Some(PLLP::Div15),
            15 => Some(PLLP::Div16),
            16 => Some(PLLP::Div17),
            17 => Some(PLLP::Div18),
            18 => Some(PLLP::Div19),
            19 => Some(PLLP::Div20),
            20 => Some(PLLP::Div21),
            21 => Some(PLLP::Div22),
            22 => Some(PLLP::Div23),
            23 => Some(PLLP::Div24),
            24 => Some(PLLP::Div25),
            25 => Some(PLLP::Div26),
            26 => Some(PLLP::Div27),
            27 => Some(PLLP::Div28),
            28 => Some(PLLP::Div29),
            29 => Some(PLLP::Div30),
            30 => Some(PLLP::Div31),
            31 => Some(PLLP::Div32),
            _ => None,
        }
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLP::Div2
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLP::Div3
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLP::Div4
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLP::Div5
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLP::Div6
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLP::Div7
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLP::Div8
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLP::Div9
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLP::Div10
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLP::Div11
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLP::Div12
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLP::Div13
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLP::Div14
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLP::Div15
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLP::Div16
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLP::Div17
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLP::Div18
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLP::Div19
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLP::Div20
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLP::Div21
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLP::Div22
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLP::Div23
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLP::Div24
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLP::Div25
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLP::Div26
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLP::Div27
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLP::Div28
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLP::Div29
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLP::Div30
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLP::Div31
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLP::Div32
    }
}
#[doc = "Field `PLLP` writer - Main PLL division factor for PLLPCLK."]
pub type PLLP_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PLLP>;
impl<'a, REG> PLLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div2)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div3)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div4)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div5)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div6)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div7)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div8)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div9)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div10)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div11)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div12)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div13)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div14)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div15)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div16)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div17)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div18)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div19)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div20)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div21)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div22)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div23)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div24)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div25)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div26)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div27)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div28)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div29)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div30)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div31)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div32)
    }
}
#[doc = "Field `PLLQEN` reader - Main PLL PLLQCLK output enable"]
pub use PLLPEN_R as PLLQEN_R;
#[doc = "Field `PLLQEN` writer - Main PLL PLLQCLK output enable"]
pub use PLLPEN_W as PLLQEN_W;
#[doc = "Main PLL division factor for PLLQCLK\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLQ {
    #[doc = "1: PLL = VCO/(N+1)"]
    Div2 = 1,
    #[doc = "2: PLL = VCO/(N+1)"]
    Div3 = 2,
    #[doc = "3: PLL = VCO/(N+1)"]
    Div4 = 3,
    #[doc = "4: PLL = VCO/(N+1)"]
    Div5 = 4,
    #[doc = "5: PLL = VCO/(N+1)"]
    Div6 = 5,
    #[doc = "6: PLL = VCO/(N+1)"]
    Div7 = 6,
    #[doc = "7: PLL = VCO/(N+1)"]
    Div8 = 7,
}
impl From<PLLQ> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLQ {
    type Ux = u8;
}
#[doc = "Field `PLLQ` reader - Main PLL division factor for PLLQCLK"]
pub type PLLQ_R = crate::FieldReader<PLLQ>;
impl PLLQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLQ> {
        match self.bits {
            1 => Some(PLLQ::Div2),
            2 => Some(PLLQ::Div3),
            3 => Some(PLLQ::Div4),
            4 => Some(PLLQ::Div5),
            5 => Some(PLLQ::Div6),
            6 => Some(PLLQ::Div7),
            7 => Some(PLLQ::Div8),
            _ => None,
        }
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLQ::Div2
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLQ::Div3
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLQ::Div4
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLQ::Div5
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLQ::Div6
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLQ::Div7
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLQ::Div8
    }
}
#[doc = "Field `PLLQ` writer - Main PLL division factor for PLLQCLK"]
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PLLQ>;
impl<'a, REG> PLLQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div2)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div3)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div4)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div5)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div6)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div7)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div8)
    }
}
#[doc = "Field `PLLREN` reader - Main PLL PLLRCLK output enable"]
pub use PLLPEN_R as PLLREN_R;
#[doc = "Field `PLLREN` writer - Main PLL PLLRCLK output enable"]
pub use PLLPEN_W as PLLREN_W;
#[doc = "Field `PLLR` reader - Main PLL division factor for PLLRCLK"]
pub use PLLQ_R as PLLR_R;
#[doc = "Field `PLLR` writer - Main PLL division factor for PLLRCLK"]
pub use PLLQ_W as PLLR_W;
impl R {
    #[doc = "Bits 0:1 - Main PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Division factor for the main PLL input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Main PLL PLLPCLK output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - Main PLL division factor for PLLPCLK."]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Main PLL PLLQCLK output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Main PLL division factor for PLLQCLK"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Main PLL PLLRCLK output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Main PLL division factor for PLLRCLK"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Main PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<PLLCFGRrs> {
        PLLSRC_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Division factor for the main PLL input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<PLLCFGRrs> {
        PLLM_W::new(self, 4)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<PLLCFGRrs> {
        PLLN_W::new(self, 8)
    }
    #[doc = "Bit 16 - Main PLL PLLPCLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<PLLCFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    #[doc = "Bits 17:21 - Main PLL division factor for PLLPCLK."]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<PLLCFGRrs> {
        PLLP_W::new(self, 17)
    }
    #[doc = "Bit 24 - Main PLL PLLQCLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<PLLCFGRrs> {
        PLLQEN_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - Main PLL division factor for PLLQCLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<PLLCFGRrs> {
        PLLQ_W::new(self, 25)
    }
    #[doc = "Bit 28 - Main PLL PLLRCLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<PLLCFGRrs> {
        PLLREN_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - Main PLL division factor for PLLRCLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<PLLCFGRrs> {
        PLLR_W::new(self, 29)
    }
}
#[doc = "PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCFGRrs;
impl crate::RegisterSpec for PLLCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PLLCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PLLCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x2204_0100"]
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x2204_0100;
}
