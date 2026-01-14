///Register `PLLCFGR` reader
pub type R = crate::R<PLLCFGRrs>;
///Register `PLLCFGR` writer
pub type W = crate::W<PLLCFGRrs>;
/**Main PLL, PLLSAI1 and PLLSAI2 entry clock source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC {
    ///0: No clock sent to PLL
    NoClock = 0,
    ///1: MSI clock selected as PLL and PLLSAI1 clock entry
    Msi = 1,
    ///2: HSI16 clock selected as PLL and PLLSAI1 clock entry
    Hsi16 = 2,
    ///3: HSE32 clock selected as PLL and PLLSAI1 clock entry
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
impl crate::IsEnum for PLLSRC {}
///Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
pub type PLLSRC_R = crate::FieldReader<PLLSRC>;
impl PLLSRC_R {
    ///Get enumerated values variant
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
    ///No clock sent to PLL
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PLLSRC::NoClock
    }
    ///MSI clock selected as PLL and PLLSAI1 clock entry
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == PLLSRC::Msi
    }
    ///HSI16 clock selected as PLL and PLLSAI1 clock entry
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC::Hsi16
    }
    ///HSE32 clock selected as PLL and PLLSAI1 clock entry
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == PLLSRC::Hse32
    }
}
///Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
pub type PLLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLSRC, crate::Safe>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock sent to PLL
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::NoClock)
    }
    ///MSI clock selected as PLL and PLLSAI1 clock entry
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Msi)
    }
    ///HSI16 clock selected as PLL and PLLSAI1 clock entry
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi16)
    }
    ///HSE32 clock selected as PLL and PLLSAI1 clock entry
    #[inline(always)]
    pub fn hse32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse32)
    }
}
/**Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLM {
    ///0: VCO input = PLL input / PLLM
    Div1 = 0,
    ///1: VCO input = PLL input / PLLM
    Div2 = 1,
    ///2: VCO input = PLL input / PLLM
    Div3 = 2,
    ///3: VCO input = PLL input / PLLM
    Div4 = 3,
    ///4: VCO input = PLL input / PLLM
    Div5 = 4,
    ///5: VCO input = PLL input / PLLM
    Div6 = 5,
    ///6: VCO input = PLL input / PLLM
    Div7 = 6,
    ///7: VCO input = PLL input / PLLM
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
impl crate::IsEnum for PLLM {}
///Field `PLLM` reader - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
pub type PLLM_R = crate::FieldReader<PLLM>;
impl PLLM_R {
    ///Get enumerated values variant
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
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLM::Div1
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLM::Div2
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLM::Div3
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLM::Div4
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLM::Div5
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLM::Div6
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLM::Div7
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLM::Div8
    }
}
///Field `PLLM` writer - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PLLM, crate::Safe>;
impl<'a, REG> PLLM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div1)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div2)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div3)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div4)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div5)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div6)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div7)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div8)
    }
}
///Field `PLLN` reader - Main PLLSYS multiplication factor N
pub type PLLN_R = crate::FieldReader;
///Field `PLLN` writer - Main PLLSYS multiplication factor N
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Main PLLSYSP output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPEN {
    ///0: PLLCLK output disabled
    Disabled = 0,
    ///1: PLLCLK output enabled
    Enabled = 1,
}
impl From<PLLPEN> for bool {
    #[inline(always)]
    fn from(variant: PLLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLPEN` reader - Main PLLSYSP output enable
pub type PLLPEN_R = crate::BitReader<PLLPEN>;
impl PLLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLPEN {
        match self.bits {
            false => PLLPEN::Disabled,
            true => PLLPEN::Enabled,
        }
    }
    ///PLLCLK output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLPEN::Disabled
    }
    ///PLLCLK output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLPEN::Enabled
    }
}
///Field `PLLPEN` writer - Main PLLSYSP output enable
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLPEN>;
impl<'a, REG> PLLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLCLK output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPEN::Disabled)
    }
    ///PLLCLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPEN::Enabled)
    }
}
/**Main PLL division factor P for PPLSYSSAICLK

Value on reset: 2*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLP {
    ///1: PLL = VCO/(N+1)
    Div2 = 1,
    ///2: PLL = VCO/(N+1)
    Div3 = 2,
    ///3: PLL = VCO/(N+1)
    Div4 = 3,
    ///4: PLL = VCO/(N+1)
    Div5 = 4,
    ///5: PLL = VCO/(N+1)
    Div6 = 5,
    ///6: PLL = VCO/(N+1)
    Div7 = 6,
    ///7: PLL = VCO/(N+1)
    Div8 = 7,
    ///8: PLL = VCO/(N+1)
    Div9 = 8,
    ///9: PLL = VCO/(N+1)
    Div10 = 9,
    ///10: PLL = VCO/(N+1)
    Div11 = 10,
    ///11: PLL = VCO/(N+1)
    Div12 = 11,
    ///12: PLL = VCO/(N+1)
    Div13 = 12,
    ///13: PLL = VCO/(N+1)
    Div14 = 13,
    ///14: PLL = VCO/(N+1)
    Div15 = 14,
    ///15: PLL = VCO/(N+1)
    Div16 = 15,
    ///16: PLL = VCO/(N+1)
    Div17 = 16,
    ///17: PLL = VCO/(N+1)
    Div18 = 17,
    ///18: PLL = VCO/(N+1)
    Div19 = 18,
    ///19: PLL = VCO/(N+1)
    Div20 = 19,
    ///20: PLL = VCO/(N+1)
    Div21 = 20,
    ///21: PLL = VCO/(N+1)
    Div22 = 21,
    ///22: PLL = VCO/(N+1)
    Div23 = 22,
    ///23: PLL = VCO/(N+1)
    Div24 = 23,
    ///24: PLL = VCO/(N+1)
    Div25 = 24,
    ///25: PLL = VCO/(N+1)
    Div26 = 25,
    ///26: PLL = VCO/(N+1)
    Div27 = 26,
    ///27: PLL = VCO/(N+1)
    Div28 = 27,
    ///28: PLL = VCO/(N+1)
    Div29 = 28,
    ///29: PLL = VCO/(N+1)
    Div30 = 29,
    ///30: PLL = VCO/(N+1)
    Div31 = 30,
    ///31: PLL = VCO/(N+1)
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
impl crate::IsEnum for PLLP {}
///Field `PLLP` reader - Main PLL division factor P for PPLSYSSAICLK
pub type PLLP_R = crate::FieldReader<PLLP>;
impl PLLP_R {
    ///Get enumerated values variant
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
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLP::Div2
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLP::Div3
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLP::Div4
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLP::Div5
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLP::Div6
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLP::Div7
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLP::Div8
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLP::Div9
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLP::Div10
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLP::Div11
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLP::Div12
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLP::Div13
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLP::Div14
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLP::Div15
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLP::Div16
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLP::Div17
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLP::Div18
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLP::Div19
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLP::Div20
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLP::Div21
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLP::Div22
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLP::Div23
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLP::Div24
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLP::Div25
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLP::Div26
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLP::Div27
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLP::Div28
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLP::Div29
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLP::Div30
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLP::Div31
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLP::Div32
    }
}
///Field `PLLP` writer - Main PLL division factor P for PPLSYSSAICLK
pub type PLLP_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PLLP>;
impl<'a, REG> PLLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div2)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div3)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div4)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div5)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div6)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div7)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div8)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div9)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div10)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div11)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div12)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div13)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div14)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div15)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div16)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div17)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div18)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div19)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div20)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div21)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div22)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div23)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div24)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div25)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div26)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div27)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div28)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div29)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div30)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div31)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div32)
    }
}
///Field `PLLQEN` reader - Main PLLSYSQ output enable
pub use PLLPEN_R as PLLQEN_R;
///Field `PLLQEN` writer - Main PLLSYSQ output enable
pub use PLLPEN_W as PLLQEN_W;
/**Main PLLSYS division factor Q for PLLSYSUSBCLK

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLQ {
    ///1: PLL = VCO/(N+1)
    Div2 = 1,
    ///2: PLL = VCO/(N+1)
    Div3 = 2,
    ///3: PLL = VCO/(N+1)
    Div4 = 3,
    ///4: PLL = VCO/(N+1)
    Div5 = 4,
    ///5: PLL = VCO/(N+1)
    Div6 = 5,
    ///6: PLL = VCO/(N+1)
    Div7 = 6,
    ///7: PLL = VCO/(N+1)
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
impl crate::IsEnum for PLLQ {}
///Field `PLLQ` reader - Main PLLSYS division factor Q for PLLSYSUSBCLK
pub type PLLQ_R = crate::FieldReader<PLLQ>;
impl PLLQ_R {
    ///Get enumerated values variant
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
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLQ::Div2
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLQ::Div3
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLQ::Div4
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLQ::Div5
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLQ::Div6
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLQ::Div7
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLQ::Div8
    }
}
///Field `PLLQ` writer - Main PLLSYS division factor Q for PLLSYSUSBCLK
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PLLQ>;
impl<'a, REG> PLLQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div2)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div3)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div4)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div5)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div6)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div7)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div8)
    }
}
///Field `PLLREN` reader - Main PLLSYSR PLLCLK output enable
pub use PLLPEN_R as PLLREN_R;
///Field `PLLREN` writer - Main PLLSYSR PLLCLK output enable
pub use PLLPEN_W as PLLREN_W;
///Field `PLLR` reader - Main PLLSYS division factor R for SYSCLK (system clock)
pub use PLLQ_R as PLLR_R;
///Field `PLLR` writer - Main PLLSYS division factor R for SYSCLK (system clock)
pub use PLLQ_W as PLLR_W;
impl R {
    ///Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:14 - Main PLLSYS multiplication factor N
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Main PLLSYSP output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21 - Main PLL division factor P for PPLSYSSAICLK
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 24 - Main PLLSYSQ output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - Main PLLSYS division factor Q for PLLSYSUSBCLK
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - Main PLLSYSR PLLCLK output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - Main PLLSYS division factor R for SYSCLK (system clock)
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pllq", &self.pllq())
            .field("pllr", &self.pllr())
            .field("pllpen", &self.pllpen())
            .field("pllren", &self.pllren())
            .field("pllqen", &self.pllqen())
            .field("pllp", &self.pllp())
            .field("plln", &self.plln())
            .field("pllm", &self.pllm())
            .field("pllsrc", &self.pllsrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<'_, PLLCFGRrs> {
        PLLSRC_W::new(self, 0)
    }
    ///Bits 4:6 - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W<'_, PLLCFGRrs> {
        PLLM_W::new(self, 4)
    }
    ///Bits 8:14 - Main PLLSYS multiplication factor N
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W<'_, PLLCFGRrs> {
        PLLN_W::new(self, 8)
    }
    ///Bit 16 - Main PLLSYSP output enable
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W<'_, PLLCFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    ///Bits 17:21 - Main PLL division factor P for PPLSYSSAICLK
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W<'_, PLLCFGRrs> {
        PLLP_W::new(self, 17)
    }
    ///Bit 24 - Main PLLSYSQ output enable
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W<'_, PLLCFGRrs> {
        PLLQEN_W::new(self, 24)
    }
    ///Bits 25:27 - Main PLLSYS division factor Q for PLLSYSUSBCLK
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W<'_, PLLCFGRrs> {
        PLLQ_W::new(self, 25)
    }
    ///Bit 28 - Main PLLSYSR PLLCLK output enable
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W<'_, PLLCFGRrs> {
        PLLREN_W::new(self, 28)
    }
    ///Bits 29:31 - Main PLLSYS division factor R for SYSCLK (system clock)
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W<'_, PLLCFGRrs> {
        PLLR_W::new(self, 29)
    }
}
/**PLLSYS configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:PLLCFGR)*/
pub struct PLLCFGRrs;
impl crate::RegisterSpec for PLLCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllcfgr::R`](R) reader structure
impl crate::Readable for PLLCFGRrs {}
///`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure
impl crate::Writable for PLLCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCFGR to value 0x2204_0100
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x2204_0100;
}
