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
    ///1: MSI clock selected as PLL clock entry
    Msi = 1,
    ///2: HSI16 clock selected as PLL clock entry
    Hsi16 = 2,
    ///3: HSE clock selected as PLL clock entry
    Hse = 3,
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
            3 => PLLSRC::Hse,
            _ => unreachable!(),
        }
    }
    ///No clock sent to PLL
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PLLSRC::NoClock
    }
    ///MSI clock selected as PLL clock entry
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == PLLSRC::Msi
    }
    ///HSI16 clock selected as PLL clock entry
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC::Hsi16
    }
    ///HSE clock selected as PLL clock entry
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC::Hse
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
    ///MSI clock selected as PLL clock entry
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Msi)
    }
    ///HSI16 clock selected as PLL clock entry
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi16)
    }
    ///HSE clock selected as PLL clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse)
    }
}
/**Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLM {
    ///0: PLLM = 1
    Div1 = 0,
    ///1: PLLM = 2
    Div2 = 1,
    ///2: PLLM = 3
    Div3 = 2,
    ///3: PLLM = 4
    Div4 = 3,
    ///4: PLLM = 5
    Div5 = 4,
    ///5: PLLM = 6
    Div6 = 5,
    ///6: PLLM = 7
    Div7 = 6,
    ///7: PLLM = 8
    Div8 = 7,
    ///8: PLLM = 9
    Div9 = 8,
    ///9: PLLM = 11
    Div10 = 9,
    ///10: PLLM = 12
    Div11 = 10,
    ///11: PLLM = 13
    Div12 = 11,
    ///12: PLLM = 13
    Div13 = 12,
    ///13: PLLM = 14
    Div14 = 13,
    ///14: PLLM = 15
    Div15 = 14,
    ///15: PLLM = 16
    Div16 = 15,
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
///Field `PLLM` reader - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
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
            8 => PLLM::Div9,
            9 => PLLM::Div10,
            10 => PLLM::Div11,
            11 => PLLM::Div12,
            12 => PLLM::Div13,
            13 => PLLM::Div14,
            14 => PLLM::Div15,
            15 => PLLM::Div16,
            _ => unreachable!(),
        }
    }
    ///PLLM = 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLM::Div1
    }
    ///PLLM = 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLM::Div2
    }
    ///PLLM = 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLM::Div3
    }
    ///PLLM = 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLM::Div4
    }
    ///PLLM = 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLM::Div5
    }
    ///PLLM = 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLM::Div6
    }
    ///PLLM = 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLM::Div7
    }
    ///PLLM = 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLM::Div8
    }
    ///PLLM = 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLM::Div9
    }
    ///PLLM = 11
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLM::Div10
    }
    ///PLLM = 12
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLM::Div11
    }
    ///PLLM = 13
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLM::Div12
    }
    ///PLLM = 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLM::Div13
    }
    ///PLLM = 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLM::Div14
    }
    ///PLLM = 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLM::Div15
    }
    ///PLLM = 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLM::Div16
    }
}
///Field `PLLM` writer - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLLM, crate::Safe>;
impl<'a, REG> PLLM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLM = 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div1)
    }
    ///PLLM = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div2)
    }
    ///PLLM = 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div3)
    }
    ///PLLM = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div4)
    }
    ///PLLM = 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div5)
    }
    ///PLLM = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div6)
    }
    ///PLLM = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div7)
    }
    ///PLLM = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div8)
    }
    ///PLLM = 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div9)
    }
    ///PLLM = 11
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div10)
    }
    ///PLLM = 12
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div11)
    }
    ///PLLM = 13
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div12)
    }
    ///PLLM = 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div13)
    }
    ///PLLM = 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div14)
    }
    ///PLLM = 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div15)
    }
    ///PLLM = 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div16)
    }
}
///Field `PLLN` reader - Main PLL multiplication factor for VCO
pub type PLLN_R = crate::FieldReader;
///Field `PLLN` writer - Main PLL multiplication factor for VCO
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Main PLL PLLSAI3CLK output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPEN {
    ///0: PLLSAI3CLK output disable
    Disabled = 0,
    ///1: PLLSAI3CLK output enabled
    Enabled = 1,
}
impl From<PLLPEN> for bool {
    #[inline(always)]
    fn from(variant: PLLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLPEN` reader - Main PLL PLLSAI3CLK output enable
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
    ///PLLSAI3CLK output disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLPEN::Disabled
    }
    ///PLLSAI3CLK output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLPEN::Enabled
    }
}
///Field `PLLPEN` writer - Main PLL PLLSAI3CLK output enable
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLPEN>;
impl<'a, REG> PLLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLSAI3CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPEN::Disabled)
    }
    ///PLLSAI3CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPEN::Enabled)
    }
}
/**Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLP {
    ///0: PLLP = 7
    Div7 = 0,
    ///1: PLLP = 17
    Div17 = 1,
}
impl From<PLLP> for bool {
    #[inline(always)]
    fn from(variant: PLLP) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLP` reader - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
pub type PLLP_R = crate::BitReader<PLLP>;
impl PLLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLP {
        match self.bits {
            false => PLLP::Div7,
            true => PLLP::Div17,
        }
    }
    ///PLLP = 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLP::Div7
    }
    ///PLLP = 17
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLP::Div17
    }
}
///Field `PLLP` writer - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
pub type PLLP_W<'a, REG> = crate::BitWriter<'a, REG, PLLP>;
impl<'a, REG> PLLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLP = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div7)
    }
    ///PLLP = 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div17)
    }
}
/**Main PLL PLLUSB1CLK output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLQEN {
    ///0: PLL48M1CLK output disable
    Disabled = 0,
    ///1: PLL48M1CLK output enabled
    Enabled = 1,
}
impl From<PLLQEN> for bool {
    #[inline(always)]
    fn from(variant: PLLQEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLQEN` reader - Main PLL PLLUSB1CLK output enable
pub type PLLQEN_R = crate::BitReader<PLLQEN>;
impl PLLQEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLQEN {
        match self.bits {
            false => PLLQEN::Disabled,
            true => PLLQEN::Enabled,
        }
    }
    ///PLL48M1CLK output disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLQEN::Disabled
    }
    ///PLL48M1CLK output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLQEN::Enabled
    }
}
///Field `PLLQEN` writer - Main PLL PLLUSB1CLK output enable
pub type PLLQEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLQEN>;
impl<'a, REG> PLLQEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL48M1CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQEN::Disabled)
    }
    ///PLL48M1CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQEN::Enabled)
    }
}
/**Main PLL division factor for PLLUSB1CLK(48 MHz clock)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLQ {
    ///0: PLLx = 2
    Div2 = 0,
    ///1: PLLx = 4
    Div4 = 1,
    ///2: PLLx = 6
    Div6 = 2,
    ///3: PLLx = 8
    Div8 = 3,
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
///Field `PLLQ` reader - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
pub type PLLQ_R = crate::FieldReader<PLLQ>;
impl PLLQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLQ {
        match self.bits {
            0 => PLLQ::Div2,
            1 => PLLQ::Div4,
            2 => PLLQ::Div6,
            3 => PLLQ::Div8,
            _ => unreachable!(),
        }
    }
    ///PLLx = 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLQ::Div2
    }
    ///PLLx = 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLQ::Div4
    }
    ///PLLx = 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLQ::Div6
    }
    ///PLLx = 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLQ::Div8
    }
}
///Field `PLLQ` writer - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLQ, crate::Safe>;
impl<'a, REG> PLLQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLx = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div2)
    }
    ///PLLx = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div4)
    }
    ///PLLx = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div6)
    }
    ///PLLx = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div8)
    }
}
/**Main PLL PLLCLK output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLREN {
    ///0: PLLCLK output disable
    Disabled = 0,
    ///1: PLLCLK output enabled
    Enabled = 1,
}
impl From<PLLREN> for bool {
    #[inline(always)]
    fn from(variant: PLLREN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLREN` reader - Main PLL PLLCLK output enable
pub type PLLREN_R = crate::BitReader<PLLREN>;
impl PLLREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLREN {
        match self.bits {
            false => PLLREN::Disabled,
            true => PLLREN::Enabled,
        }
    }
    ///PLLCLK output disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLREN::Disabled
    }
    ///PLLCLK output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLREN::Enabled
    }
}
///Field `PLLREN` writer - Main PLL PLLCLK output enable
pub type PLLREN_W<'a, REG> = crate::BitWriter<'a, REG, PLLREN>;
impl<'a, REG> PLLREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLCLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLREN::Disabled)
    }
    ///PLLCLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLREN::Enabled)
    }
}
///Field `PLLR` reader - Main PLL division factor for PLLCLK (system clock)
pub use PLLQ_R as PLLR_R;
///Field `PLLR` writer - Main PLL division factor for PLLCLK (system clock)
pub use PLLQ_W as PLLR_W;
/**Main PLL division factor for PLLSAI2CLK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLPDIV {
    ///0: PLLSAI3CLK is controlled by the bit PLLP
    Pllp = 0,
    ///2: PLLSAI3CLK = VCO / 2
    Div2 = 2,
    ///3: PLLSAI3CLK = VCO / 3
    Div3 = 3,
    ///4: PLLSAI3CLK = VCO / 4
    Div4 = 4,
    ///5: PLLSAI3CLK = VCO / 5
    Div5 = 5,
    ///6: PLLSAI3CLK = VCO / 6
    Div6 = 6,
    ///7: PLLSAI3CLK = VCO / 7
    Div7 = 7,
    ///8: PLLSAI3CLK = VCO / 8
    Div8 = 8,
    ///9: PLLSAI3CLK = VCO / 9
    Div9 = 9,
    ///10: PLLSAI3CLK = VCO / 10
    Div10 = 10,
    ///11: PLLSAI3CLK = VCO / 11
    Div11 = 11,
    ///12: PLLSAI3CLK = VCO / 12
    Div12 = 12,
    ///13: PLLSAI3CLK = VCO / 13
    Div13 = 13,
    ///14: PLLSAI3CLK = VCO / 14
    Div14 = 14,
    ///15: PLLSAI3CLK = VCO / 15
    Div15 = 15,
    ///16: PLLSAI3CLK = VCO / 16
    Div16 = 16,
    ///17: PLLSAI3CLK = VCO / 17
    Div17 = 17,
    ///18: PLLSAI3CLK = VCO / 18
    Div18 = 18,
    ///19: PLLSAI3CLK = VCO / 19
    Div19 = 19,
    ///20: PLLSAI3CLK = VCO / 20
    Div20 = 20,
    ///21: PLLSAI3CLK = VCO / 21
    Div21 = 21,
    ///22: PLLSAI3CLK = VCO / 22
    Div22 = 22,
    ///23: PLLSAI3CLK = VCO / 23
    Div23 = 23,
    ///24: PLLSAI3CLK = VCO / 24
    Div24 = 24,
    ///25: PLLSAI3CLK = VCO / 25
    Div25 = 25,
    ///26: PLLSAI3CLK = VCO / 26
    Div26 = 26,
    ///27: PLLSAI3CLK = VCO / 27
    Div27 = 27,
    ///28: PLLSAI3CLK = VCO / 28
    Div28 = 28,
    ///29: PLLSAI3CLK = VCO / 29
    Div29 = 29,
    ///30: PLLSAI3CLK = VCO / 30
    Div30 = 30,
    ///31: PLLSAI3CLK = VCO / 31
    Div31 = 31,
}
impl From<PLLPDIV> for u8 {
    #[inline(always)]
    fn from(variant: PLLPDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLPDIV {
    type Ux = u8;
}
impl crate::IsEnum for PLLPDIV {}
///Field `PLLPDIV` reader - Main PLL division factor for PLLSAI2CLK
pub type PLLPDIV_R = crate::FieldReader<PLLPDIV>;
impl PLLPDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLPDIV> {
        match self.bits {
            0 => Some(PLLPDIV::Pllp),
            2 => Some(PLLPDIV::Div2),
            3 => Some(PLLPDIV::Div3),
            4 => Some(PLLPDIV::Div4),
            5 => Some(PLLPDIV::Div5),
            6 => Some(PLLPDIV::Div6),
            7 => Some(PLLPDIV::Div7),
            8 => Some(PLLPDIV::Div8),
            9 => Some(PLLPDIV::Div9),
            10 => Some(PLLPDIV::Div10),
            11 => Some(PLLPDIV::Div11),
            12 => Some(PLLPDIV::Div12),
            13 => Some(PLLPDIV::Div13),
            14 => Some(PLLPDIV::Div14),
            15 => Some(PLLPDIV::Div15),
            16 => Some(PLLPDIV::Div16),
            17 => Some(PLLPDIV::Div17),
            18 => Some(PLLPDIV::Div18),
            19 => Some(PLLPDIV::Div19),
            20 => Some(PLLPDIV::Div20),
            21 => Some(PLLPDIV::Div21),
            22 => Some(PLLPDIV::Div22),
            23 => Some(PLLPDIV::Div23),
            24 => Some(PLLPDIV::Div24),
            25 => Some(PLLPDIV::Div25),
            26 => Some(PLLPDIV::Div26),
            27 => Some(PLLPDIV::Div27),
            28 => Some(PLLPDIV::Div28),
            29 => Some(PLLPDIV::Div29),
            30 => Some(PLLPDIV::Div30),
            31 => Some(PLLPDIV::Div31),
            _ => None,
        }
    }
    ///PLLSAI3CLK is controlled by the bit PLLP
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == PLLPDIV::Pllp
    }
    ///PLLSAI3CLK = VCO / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLPDIV::Div2
    }
    ///PLLSAI3CLK = VCO / 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLPDIV::Div3
    }
    ///PLLSAI3CLK = VCO / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLPDIV::Div4
    }
    ///PLLSAI3CLK = VCO / 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLPDIV::Div5
    }
    ///PLLSAI3CLK = VCO / 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLPDIV::Div6
    }
    ///PLLSAI3CLK = VCO / 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLPDIV::Div7
    }
    ///PLLSAI3CLK = VCO / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLPDIV::Div8
    }
    ///PLLSAI3CLK = VCO / 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLPDIV::Div9
    }
    ///PLLSAI3CLK = VCO / 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLPDIV::Div10
    }
    ///PLLSAI3CLK = VCO / 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLPDIV::Div11
    }
    ///PLLSAI3CLK = VCO / 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLPDIV::Div12
    }
    ///PLLSAI3CLK = VCO / 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLPDIV::Div13
    }
    ///PLLSAI3CLK = VCO / 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLPDIV::Div14
    }
    ///PLLSAI3CLK = VCO / 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLPDIV::Div15
    }
    ///PLLSAI3CLK = VCO / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLPDIV::Div16
    }
    ///PLLSAI3CLK = VCO / 17
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLPDIV::Div17
    }
    ///PLLSAI3CLK = VCO / 18
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLPDIV::Div18
    }
    ///PLLSAI3CLK = VCO / 19
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLPDIV::Div19
    }
    ///PLLSAI3CLK = VCO / 20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLPDIV::Div20
    }
    ///PLLSAI3CLK = VCO / 21
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLPDIV::Div21
    }
    ///PLLSAI3CLK = VCO / 22
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLPDIV::Div22
    }
    ///PLLSAI3CLK = VCO / 23
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLPDIV::Div23
    }
    ///PLLSAI3CLK = VCO / 24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLPDIV::Div24
    }
    ///PLLSAI3CLK = VCO / 25
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLPDIV::Div25
    }
    ///PLLSAI3CLK = VCO / 26
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLPDIV::Div26
    }
    ///PLLSAI3CLK = VCO / 27
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLPDIV::Div27
    }
    ///PLLSAI3CLK = VCO / 28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLPDIV::Div28
    }
    ///PLLSAI3CLK = VCO / 29
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLPDIV::Div29
    }
    ///PLLSAI3CLK = VCO / 30
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLPDIV::Div30
    }
    ///PLLSAI3CLK = VCO / 31
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLPDIV::Div31
    }
}
///Field `PLLPDIV` writer - Main PLL division factor for PLLSAI2CLK
pub type PLLPDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PLLPDIV>;
impl<'a, REG> PLLPDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLSAI3CLK is controlled by the bit PLLP
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Pllp)
    }
    ///PLLSAI3CLK = VCO / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div2)
    }
    ///PLLSAI3CLK = VCO / 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div3)
    }
    ///PLLSAI3CLK = VCO / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div4)
    }
    ///PLLSAI3CLK = VCO / 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div5)
    }
    ///PLLSAI3CLK = VCO / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div6)
    }
    ///PLLSAI3CLK = VCO / 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div7)
    }
    ///PLLSAI3CLK = VCO / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div8)
    }
    ///PLLSAI3CLK = VCO / 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div9)
    }
    ///PLLSAI3CLK = VCO / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div10)
    }
    ///PLLSAI3CLK = VCO / 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div11)
    }
    ///PLLSAI3CLK = VCO / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div12)
    }
    ///PLLSAI3CLK = VCO / 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div13)
    }
    ///PLLSAI3CLK = VCO / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div14)
    }
    ///PLLSAI3CLK = VCO / 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div15)
    }
    ///PLLSAI3CLK = VCO / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div16)
    }
    ///PLLSAI3CLK = VCO / 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div17)
    }
    ///PLLSAI3CLK = VCO / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div18)
    }
    ///PLLSAI3CLK = VCO / 19
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div19)
    }
    ///PLLSAI3CLK = VCO / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div20)
    }
    ///PLLSAI3CLK = VCO / 21
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div21)
    }
    ///PLLSAI3CLK = VCO / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div22)
    }
    ///PLLSAI3CLK = VCO / 23
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div23)
    }
    ///PLLSAI3CLK = VCO / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div24)
    }
    ///PLLSAI3CLK = VCO / 25
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div25)
    }
    ///PLLSAI3CLK = VCO / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div26)
    }
    ///PLLSAI3CLK = VCO / 27
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div27)
    }
    ///PLLSAI3CLK = VCO / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div28)
    }
    ///PLLSAI3CLK = VCO / 29
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div29)
    }
    ///PLLSAI3CLK = VCO / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div30)
    }
    ///PLLSAI3CLK = VCO / 31
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div31)
    }
}
impl R {
    ///Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Main PLL PLLSAI3CLK output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Main PLL PLLUSB1CLK output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - Main PLL PLLCLK output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Main PLL division factor for PLLCLK (system clock)
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:31 - Main PLL division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllpdiv(&self) -> PLLPDIV_R {
        PLLPDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pllpdiv", &self.pllpdiv())
            .field("pllq", &self.pllq())
            .field("pllr", &self.pllr())
            .field("pllren", &self.pllren())
            .field("pllqen", &self.pllqen())
            .field("pllp", &self.pllp())
            .field("pllpen", &self.pllpen())
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
    ///Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W<'_, PLLCFGRrs> {
        PLLM_W::new(self, 4)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W<'_, PLLCFGRrs> {
        PLLN_W::new(self, 8)
    }
    ///Bit 16 - Main PLL PLLSAI3CLK output enable
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W<'_, PLLCFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    ///Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W<'_, PLLCFGRrs> {
        PLLP_W::new(self, 17)
    }
    ///Bit 20 - Main PLL PLLUSB1CLK output enable
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W<'_, PLLCFGRrs> {
        PLLQEN_W::new(self, 20)
    }
    ///Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W<'_, PLLCFGRrs> {
        PLLQ_W::new(self, 21)
    }
    ///Bit 24 - Main PLL PLLCLK output enable
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W<'_, PLLCFGRrs> {
        PLLREN_W::new(self, 24)
    }
    ///Bits 25:26 - Main PLL division factor for PLLCLK (system clock)
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W<'_, PLLCFGRrs> {
        PLLR_W::new(self, 25)
    }
    ///Bits 27:31 - Main PLL division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllpdiv(&mut self) -> PLLPDIV_W<'_, PLLCFGRrs> {
        PLLPDIV_W::new(self, 27)
    }
}
/**PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:PLLCFGR)*/
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
///`reset()` method sets PLLCFGR to value 0x1000
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
