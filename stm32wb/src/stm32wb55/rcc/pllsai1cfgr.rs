///Register `PLLSAI1CFGR` reader
pub type R = crate::R<PLLSAI1CFGRrs>;
///Register `PLLSAI1CFGR` writer
pub type W = crate::W<PLLSAI1CFGRrs>;
///Field `PLLN` reader - SAIPLL multiplication factor for VCO
pub type PLLN_R = crate::FieldReader;
///Field `PLLN` writer - SAIPLL multiplication factor for VCO
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**SAIPLL PLLSAI1CLK output enable

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
///Field `PLLPEN` reader - SAIPLL PLLSAI1CLK output enable
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
///Field `PLLPEN` writer - SAIPLL PLLSAI1CLK output enable
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
/**SAI1PLL division factor P for PLLSAICLK (SAI1clock)

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
///Field `PLLP` reader - SAI1PLL division factor P for PLLSAICLK (SAI1clock)
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
///Field `PLLP` writer - SAI1PLL division factor P for PLLSAICLK (SAI1clock)
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
///Field `PLLQEN` reader - SAIPLL PLLSAIUSBCLK output enable
pub use PLLPEN_R as PLLQEN_R;
///Field `PLLQEN` writer - SAIPLL PLLSAIUSBCLK output enable
pub use PLLPEN_W as PLLQEN_W;
/**SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)

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
///Field `PLLQ` reader - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)
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
///Field `PLLQ` writer - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)
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
///Field `PLLREN` reader - PLLSAI PLLADC1CLK output enable
pub use PLLPEN_R as PLLREN_R;
///Field `PLLREN` writer - PLLSAI PLLADC1CLK output enable
pub use PLLPEN_W as PLLREN_W;
///Field `PLLR` reader - PLLSAI division factor R for PLLADC1CLK (ADC clock)
pub use PLLQ_R as PLLR_R;
///Field `PLLR` writer - PLLSAI division factor R for PLLADC1CLK (ADC clock)
pub use PLLQ_W as PLLR_W;
impl R {
    ///Bits 8:14 - SAIPLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - SAIPLL PLLSAI1CLK output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21 - SAI1PLL division factor P for PLLSAICLK (SAI1clock)
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 24 - SAIPLL PLLSAIUSBCLK output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - PLLSAI PLLADC1CLK output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - PLLSAI division factor R for PLLADC1CLK (ADC clock)
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLSAI1CFGR")
            .field("pllq", &self.pllq())
            .field("pllr", &self.pllr())
            .field("pllpen", &self.pllpen())
            .field("pllren", &self.pllren())
            .field("pllqen", &self.pllqen())
            .field("pllp", &self.pllp())
            .field("plln", &self.plln())
            .finish()
    }
}
impl W {
    ///Bits 8:14 - SAIPLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W<'_, PLLSAI1CFGRrs> {
        PLLN_W::new(self, 8)
    }
    ///Bit 16 - SAIPLL PLLSAI1CLK output enable
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W<'_, PLLSAI1CFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    ///Bits 17:21 - SAI1PLL division factor P for PLLSAICLK (SAI1clock)
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W<'_, PLLSAI1CFGRrs> {
        PLLP_W::new(self, 17)
    }
    ///Bit 24 - SAIPLL PLLSAIUSBCLK output enable
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W<'_, PLLSAI1CFGRrs> {
        PLLQEN_W::new(self, 24)
    }
    ///Bits 25:27 - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W<'_, PLLSAI1CFGRrs> {
        PLLQ_W::new(self, 25)
    }
    ///Bit 28 - PLLSAI PLLADC1CLK output enable
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W<'_, PLLSAI1CFGRrs> {
        PLLREN_W::new(self, 28)
    }
    ///Bits 29:31 - PLLSAI division factor R for PLLADC1CLK (ADC clock)
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W<'_, PLLSAI1CFGRrs> {
        PLLR_W::new(self, 29)
    }
}
/**PLLSAI1 configuration register

You can [`read`](crate::Reg::read) this register and get [`pllsai1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:PLLSAI1CFGR)*/
pub struct PLLSAI1CFGRrs;
impl crate::RegisterSpec for PLLSAI1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllsai1cfgr::R`](R) reader structure
impl crate::Readable for PLLSAI1CFGRrs {}
///`write(|w| ..)` method takes [`pllsai1cfgr::W`](W) writer structure
impl crate::Writable for PLLSAI1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLSAI1CFGR to value 0x2204_0100
impl crate::Resettable for PLLSAI1CFGRrs {
    const RESET_VALUE: u32 = 0x2204_0100;
}
