#[doc = "Register `PLLSAI2CFGR` reader"]
pub type R = crate::R<PLLSAI2CFGRrs>;
#[doc = "Register `PLLSAI2CFGR` writer"]
pub type W = crate::W<PLLSAI2CFGRrs>;
#[doc = "Division factor for PLLSAI2 input clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI2M {
    #[doc = "0: PLLSAI2M = 1"]
    Div1 = 0,
    #[doc = "1: PLLSAI2M = 2"]
    Div2 = 1,
    #[doc = "2: PLLSAI2M = 3"]
    Div3 = 2,
    #[doc = "3: PLLSAI2M = 4"]
    Div4 = 3,
    #[doc = "4: PLLSAI2M = 5"]
    Div5 = 4,
    #[doc = "5: PLLSAI2M = 6"]
    Div6 = 5,
    #[doc = "6: PLLSAI2M = 7"]
    Div7 = 6,
    #[doc = "7: PLLSAI2M = 8"]
    Div8 = 7,
    #[doc = "8: PLLSAI2M = 9"]
    Div9 = 8,
    #[doc = "9: PLLSAI2M = 11"]
    Div10 = 9,
    #[doc = "10: PLLSAI2M = 12"]
    Div11 = 10,
    #[doc = "11: PLLSAI2M = 13"]
    Div12 = 11,
    #[doc = "12: PLLSAI2M = 13"]
    Div13 = 12,
    #[doc = "13: PLLSAI2M = 14"]
    Div14 = 13,
    #[doc = "14: PLLSAI2M = 15"]
    Div15 = 14,
    #[doc = "15: PLLSAI2M = 16"]
    Div16 = 15,
}
impl From<PLLSAI2M> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAI2M {
    type Ux = u8;
}
#[doc = "Field `PLLSAI2M` reader - Division factor for PLLSAI2 input clock"]
pub type PLLSAI2M_R = crate::FieldReader<PLLSAI2M>;
impl PLLSAI2M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2M {
        match self.bits {
            0 => PLLSAI2M::Div1,
            1 => PLLSAI2M::Div2,
            2 => PLLSAI2M::Div3,
            3 => PLLSAI2M::Div4,
            4 => PLLSAI2M::Div5,
            5 => PLLSAI2M::Div6,
            6 => PLLSAI2M::Div7,
            7 => PLLSAI2M::Div8,
            8 => PLLSAI2M::Div9,
            9 => PLLSAI2M::Div10,
            10 => PLLSAI2M::Div11,
            11 => PLLSAI2M::Div12,
            12 => PLLSAI2M::Div13,
            13 => PLLSAI2M::Div14,
            14 => PLLSAI2M::Div15,
            15 => PLLSAI2M::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLSAI2M = 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLSAI2M::Div1
    }
    #[doc = "PLLSAI2M = 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2M::Div2
    }
    #[doc = "PLLSAI2M = 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAI2M::Div3
    }
    #[doc = "PLLSAI2M = 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2M::Div4
    }
    #[doc = "PLLSAI2M = 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAI2M::Div5
    }
    #[doc = "PLLSAI2M = 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI2M::Div6
    }
    #[doc = "PLLSAI2M = 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI2M::Div7
    }
    #[doc = "PLLSAI2M = 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2M::Div8
    }
    #[doc = "PLLSAI2M = 9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAI2M::Div9
    }
    #[doc = "PLLSAI2M = 11"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAI2M::Div10
    }
    #[doc = "PLLSAI2M = 12"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAI2M::Div11
    }
    #[doc = "PLLSAI2M = 13"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAI2M::Div12
    }
    #[doc = "PLLSAI2M = 13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAI2M::Div13
    }
    #[doc = "PLLSAI2M = 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAI2M::Div14
    }
    #[doc = "PLLSAI2M = 15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAI2M::Div15
    }
    #[doc = "PLLSAI2M = 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI2M::Div16
    }
}
#[doc = "Field `PLLSAI2M` writer - Division factor for PLLSAI2 input clock"]
pub type PLLSAI2M_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, PLLSAI2M>;
impl<'a, REG> PLLSAI2M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLSAI2M = 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div1)
    }
    #[doc = "PLLSAI2M = 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div2)
    }
    #[doc = "PLLSAI2M = 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div3)
    }
    #[doc = "PLLSAI2M = 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div4)
    }
    #[doc = "PLLSAI2M = 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div5)
    }
    #[doc = "PLLSAI2M = 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div6)
    }
    #[doc = "PLLSAI2M = 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div7)
    }
    #[doc = "PLLSAI2M = 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div8)
    }
    #[doc = "PLLSAI2M = 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div9)
    }
    #[doc = "PLLSAI2M = 11"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div10)
    }
    #[doc = "PLLSAI2M = 12"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div11)
    }
    #[doc = "PLLSAI2M = 13"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div12)
    }
    #[doc = "PLLSAI2M = 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div13)
    }
    #[doc = "PLLSAI2M = 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div14)
    }
    #[doc = "PLLSAI2M = 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div15)
    }
    #[doc = "PLLSAI2M = 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2M::Div16)
    }
}
#[doc = "Field `PLLSAI2N` reader - SAI2PLL multiplication factor for VCO"]
pub type PLLSAI2N_R = crate::FieldReader;
#[doc = "Field `PLLSAI2N` writer - SAI2PLL multiplication factor for VCO"]
pub type PLLSAI2N_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "SAI2PLL PLLSAI2CLK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2PEN {
    #[doc = "0: PLLSAI2CLK output disable"]
    Disabled = 0,
    #[doc = "1: PLLSAI2CLK output enabled"]
    Enabled = 1,
}
impl From<PLLSAI2PEN> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2PEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI2PEN` reader - SAI2PLL PLLSAI2CLK output enable"]
pub type PLLSAI2PEN_R = crate::BitReader<PLLSAI2PEN>;
impl PLLSAI2PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2PEN {
        match self.bits {
            false => PLLSAI2PEN::Disabled,
            true => PLLSAI2PEN::Enabled,
        }
    }
    #[doc = "PLLSAI2CLK output disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2PEN::Disabled
    }
    #[doc = "PLLSAI2CLK output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2PEN::Enabled
    }
}
#[doc = "Field `PLLSAI2PEN` writer - SAI2PLL PLLSAI2CLK output enable"]
pub type PLLSAI2PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI2PEN>;
impl<'a, REG> PLLSAI2PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLSAI2CLK output disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PEN::Disabled)
    }
    #[doc = "PLLSAI2CLK output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PEN::Enabled)
    }
}
#[doc = "SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2P {
    #[doc = "0: PLLSAI2P = 7"]
    Div7 = 0,
    #[doc = "1: PLLSAI2P = 17"]
    Div17 = 1,
}
impl From<PLLSAI2P> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI2P` reader - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI2P_R = crate::BitReader<PLLSAI2P>;
impl PLLSAI2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2P {
        match self.bits {
            false => PLLSAI2P::Div7,
            true => PLLSAI2P::Div17,
        }
    }
    #[doc = "PLLSAI2P = 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI2P::Div7
    }
    #[doc = "PLLSAI2P = 17"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAI2P::Div17
    }
}
#[doc = "Field `PLLSAI2P` writer - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI2P_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI2P>;
impl<'a, REG> PLLSAI2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLSAI2P = 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2P::Div7)
    }
    #[doc = "PLLSAI2P = 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2P::Div17)
    }
}
#[doc = "PLLSAI2 division factor for PLLDISCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2QEN {
    #[doc = "0: PLLDSICLK output disable"]
    Disabled = 0,
    #[doc = "1: PLLDSICLK output enabled"]
    Enabled = 1,
}
impl From<PLLSAI2QEN> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2QEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI2QEN` reader - PLLSAI2 division factor for PLLDISCLK"]
pub type PLLSAI2QEN_R = crate::BitReader<PLLSAI2QEN>;
impl PLLSAI2QEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2QEN {
        match self.bits {
            false => PLLSAI2QEN::Disabled,
            true => PLLSAI2QEN::Enabled,
        }
    }
    #[doc = "PLLDSICLK output disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2QEN::Disabled
    }
    #[doc = "PLLDSICLK output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2QEN::Enabled
    }
}
#[doc = "Field `PLLSAI2QEN` writer - PLLSAI2 division factor for PLLDISCLK"]
pub type PLLSAI2QEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI2QEN>;
impl<'a, REG> PLLSAI2QEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLDSICLK output disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2QEN::Disabled)
    }
    #[doc = "PLLDSICLK output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2QEN::Enabled)
    }
}
#[doc = "SAI2PLL PLLSAI2CLK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI2Q {
    #[doc = "0: PLLSAI2x = 2"]
    Div2 = 0,
    #[doc = "1: PLLSAI2x = 4"]
    Div4 = 1,
    #[doc = "2: PLLSAI2x = 6"]
    Div6 = 2,
    #[doc = "3: PLLSAI2x = 8"]
    Div8 = 3,
}
impl From<PLLSAI2Q> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2Q) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAI2Q {
    type Ux = u8;
}
#[doc = "Field `PLLSAI2Q` reader - SAI2PLL PLLSAI2CLK output enable"]
pub type PLLSAI2Q_R = crate::FieldReader<PLLSAI2Q>;
impl PLLSAI2Q_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2Q {
        match self.bits {
            0 => PLLSAI2Q::Div2,
            1 => PLLSAI2Q::Div4,
            2 => PLLSAI2Q::Div6,
            3 => PLLSAI2Q::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLSAI2x = 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2Q::Div2
    }
    #[doc = "PLLSAI2x = 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2Q::Div4
    }
    #[doc = "PLLSAI2x = 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI2Q::Div6
    }
    #[doc = "PLLSAI2x = 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2Q::Div8
    }
}
#[doc = "Field `PLLSAI2Q` writer - SAI2PLL PLLSAI2CLK output enable"]
pub type PLLSAI2Q_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLLSAI2Q>;
impl<'a, REG> PLLSAI2Q_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLSAI2x = 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2Q::Div2)
    }
    #[doc = "PLLSAI2x = 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2Q::Div4)
    }
    #[doc = "PLLSAI2x = 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2Q::Div6)
    }
    #[doc = "PLLSAI2x = 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2Q::Div8)
    }
}
#[doc = "PLLSAI2 PLLADC2CLK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2REN {
    #[doc = "0: PLLLCDCLK output disable"]
    Disabled = 0,
    #[doc = "1: PLLLCDCLK output enabled"]
    Enabled = 1,
}
impl From<PLLSAI2REN> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2REN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI2REN` reader - PLLSAI2 PLLADC2CLK output enable"]
pub type PLLSAI2REN_R = crate::BitReader<PLLSAI2REN>;
impl PLLSAI2REN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2REN {
        match self.bits {
            false => PLLSAI2REN::Disabled,
            true => PLLSAI2REN::Enabled,
        }
    }
    #[doc = "PLLLCDCLK output disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2REN::Disabled
    }
    #[doc = "PLLLCDCLK output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2REN::Enabled
    }
}
#[doc = "Field `PLLSAI2REN` writer - PLLSAI2 PLLADC2CLK output enable"]
pub type PLLSAI2REN_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI2REN>;
impl<'a, REG> PLLSAI2REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLLCDCLK output disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2REN::Disabled)
    }
    #[doc = "PLLLCDCLK output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2REN::Enabled)
    }
}
#[doc = "Field `PLLSAI2R` reader - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub use PLLSAI2Q_R as PLLSAI2R_R;
#[doc = "Field `PLLSAI2R` writer - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub use PLLSAI2Q_W as PLLSAI2R_W;
#[doc = "PLLSAI2 division factor for PLLSAI2CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI2PDIV {
    #[doc = "0: PLLSAI2CLK is controlled by the bit PLLSAI2P"]
    Pllsai1p = 0,
    #[doc = "2: PLLSAI2CLK = VCOSAI2 / 2"]
    Div2 = 2,
    #[doc = "3: PLLSAI2CLK = VCOSAI2 / 3"]
    Div3 = 3,
    #[doc = "4: PLLSAI2CLK = VCOSAI2 / 4"]
    Div4 = 4,
    #[doc = "5: PLLSAI2CLK = VCOSAI2 / 5"]
    Div5 = 5,
    #[doc = "6: PLLSAI2CLK = VCOSAI2 / 6"]
    Div6 = 6,
    #[doc = "7: PLLSAI2CLK = VCOSAI2 / 7"]
    Div7 = 7,
    #[doc = "8: PLLSAI2CLK = VCOSAI2 / 8"]
    Div8 = 8,
    #[doc = "9: PLLSAI2CLK = VCOSAI2 / 9"]
    Div9 = 9,
    #[doc = "10: PLLSAI2CLK = VCOSAI2 / 10"]
    Div10 = 10,
    #[doc = "11: PLLSAI2CLK = VCOSAI2 / 11"]
    Div11 = 11,
    #[doc = "12: PLLSAI2CLK = VCOSAI2 / 12"]
    Div12 = 12,
    #[doc = "13: PLLSAI2CLK = VCOSAI2 / 13"]
    Div13 = 13,
    #[doc = "14: PLLSAI2CLK = VCOSAI2 / 14"]
    Div14 = 14,
    #[doc = "15: PLLSAI2CLK = VCOSAI2 / 15"]
    Div15 = 15,
    #[doc = "16: PLLSAI2CLK = VCOSAI2 / 16"]
    Div16 = 16,
    #[doc = "17: PLLSAI2CLK = VCOSAI2 / 17"]
    Div17 = 17,
    #[doc = "18: PLLSAI2CLK = VCOSAI2 / 18"]
    Div18 = 18,
    #[doc = "19: PLLSAI2CLK = VCOSAI2 / 19"]
    Div19 = 19,
    #[doc = "20: PLLSAI2CLK = VCOSAI2 / 20"]
    Div20 = 20,
    #[doc = "21: PLLSAI2CLK = VCOSAI2 / 21"]
    Div21 = 21,
    #[doc = "22: PLLSAI2CLK = VCOSAI2 / 22"]
    Div22 = 22,
    #[doc = "23: PLLSAI2CLK = VCOSAI2 / 23"]
    Div23 = 23,
    #[doc = "24: PLLSAI2CLK = VCOSAI2 / 24"]
    Div24 = 24,
    #[doc = "25: PLLSAI2CLK = VCOSAI2 / 25"]
    Div25 = 25,
    #[doc = "26: PLLSAI2CLK = VCOSAI2 / 26"]
    Div26 = 26,
    #[doc = "27: PLLSAI2CLK = VCOSAI2 / 27"]
    Div27 = 27,
    #[doc = "28: PLLSAI2CLK = VCOSAI2 / 28"]
    Div28 = 28,
    #[doc = "29: PLLSAI2CLK = VCOSAI2 / 29"]
    Div29 = 29,
    #[doc = "30: PLLSAI2CLK = VCOSAI2 / 30"]
    Div30 = 30,
    #[doc = "31: PLLSAI2CLK = VCOSAI2 / 31"]
    Div31 = 31,
}
impl From<PLLSAI2PDIV> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2PDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAI2PDIV {
    type Ux = u8;
}
#[doc = "Field `PLLSAI2PDIV` reader - PLLSAI2 division factor for PLLSAI2CLK"]
pub type PLLSAI2PDIV_R = crate::FieldReader<PLLSAI2PDIV>;
impl PLLSAI2PDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLSAI2PDIV> {
        match self.bits {
            0 => Some(PLLSAI2PDIV::Pllsai1p),
            2 => Some(PLLSAI2PDIV::Div2),
            3 => Some(PLLSAI2PDIV::Div3),
            4 => Some(PLLSAI2PDIV::Div4),
            5 => Some(PLLSAI2PDIV::Div5),
            6 => Some(PLLSAI2PDIV::Div6),
            7 => Some(PLLSAI2PDIV::Div7),
            8 => Some(PLLSAI2PDIV::Div8),
            9 => Some(PLLSAI2PDIV::Div9),
            10 => Some(PLLSAI2PDIV::Div10),
            11 => Some(PLLSAI2PDIV::Div11),
            12 => Some(PLLSAI2PDIV::Div12),
            13 => Some(PLLSAI2PDIV::Div13),
            14 => Some(PLLSAI2PDIV::Div14),
            15 => Some(PLLSAI2PDIV::Div15),
            16 => Some(PLLSAI2PDIV::Div16),
            17 => Some(PLLSAI2PDIV::Div17),
            18 => Some(PLLSAI2PDIV::Div18),
            19 => Some(PLLSAI2PDIV::Div19),
            20 => Some(PLLSAI2PDIV::Div20),
            21 => Some(PLLSAI2PDIV::Div21),
            22 => Some(PLLSAI2PDIV::Div22),
            23 => Some(PLLSAI2PDIV::Div23),
            24 => Some(PLLSAI2PDIV::Div24),
            25 => Some(PLLSAI2PDIV::Div25),
            26 => Some(PLLSAI2PDIV::Div26),
            27 => Some(PLLSAI2PDIV::Div27),
            28 => Some(PLLSAI2PDIV::Div28),
            29 => Some(PLLSAI2PDIV::Div29),
            30 => Some(PLLSAI2PDIV::Div30),
            31 => Some(PLLSAI2PDIV::Div31),
            _ => None,
        }
    }
    #[doc = "PLLSAI2CLK is controlled by the bit PLLSAI2P"]
    #[inline(always)]
    pub fn is_pllsai1p(&self) -> bool {
        *self == PLLSAI2PDIV::Pllsai1p
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2PDIV::Div2
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAI2PDIV::Div3
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2PDIV::Div4
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAI2PDIV::Div5
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI2PDIV::Div6
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI2PDIV::Div7
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2PDIV::Div8
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAI2PDIV::Div9
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAI2PDIV::Div10
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAI2PDIV::Div11
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAI2PDIV::Div12
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAI2PDIV::Div13
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAI2PDIV::Div14
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAI2PDIV::Div15
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI2PDIV::Div16
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 17"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAI2PDIV::Div17
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 18"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLSAI2PDIV::Div18
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 19"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLSAI2PDIV::Div19
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 20"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLSAI2PDIV::Div20
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 21"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLSAI2PDIV::Div21
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 22"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLSAI2PDIV::Div22
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 23"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLSAI2PDIV::Div23
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 24"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLSAI2PDIV::Div24
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 25"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLSAI2PDIV::Div25
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 26"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLSAI2PDIV::Div26
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 27"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLSAI2PDIV::Div27
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 28"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLSAI2PDIV::Div28
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 29"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLSAI2PDIV::Div29
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 30"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLSAI2PDIV::Div30
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 31"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLSAI2PDIV::Div31
    }
}
#[doc = "Field `PLLSAI2PDIV` writer - PLLSAI2 division factor for PLLSAI2CLK"]
pub type PLLSAI2PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PLLSAI2PDIV>;
impl<'a, REG> PLLSAI2PDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLSAI2CLK is controlled by the bit PLLSAI2P"]
    #[inline(always)]
    pub fn pllsai1p(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Pllsai1p)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div2)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div3)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div4)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div5)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div6)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div7)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div8)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div9)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div10)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div11)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div12)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div13)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div14)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div15)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div16)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div17)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div18)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div19)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div20)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div21)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div22)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div23)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div24)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div25)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div26)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div27)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div28)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div29)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div30)
    }
    #[doc = "PLLSAI2CLK = VCOSAI2 / 31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2PDIV::Div31)
    }
}
impl R {
    #[doc = "Bits 4:7 - Division factor for PLLSAI2 input clock"]
    #[inline(always)]
    pub fn pllsai2m(&self) -> PLLSAI2M_R {
        PLLSAI2M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&self) -> PLLSAI2N_R {
        PLLSAI2N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&self) -> PLLSAI2PEN_R {
        PLLSAI2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&self) -> PLLSAI2P_R {
        PLLSAI2P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - PLLSAI2 division factor for PLLDISCLK"]
    #[inline(always)]
    pub fn pllsai2qen(&self) -> PLLSAI2QEN_R {
        PLLSAI2QEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2q(&self) -> PLLSAI2Q_R {
        PLLSAI2Q_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&self) -> PLLSAI2REN_R {
        PLLSAI2REN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&self) -> PLLSAI2R_R {
        PLLSAI2R_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&self) -> PLLSAI2PDIV_R {
        PLLSAI2PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Division factor for PLLSAI2 input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2m(&mut self) -> PLLSAI2M_W<PLLSAI2CFGRrs> {
        PLLSAI2M_W::new(self, 4)
    }
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2n(&mut self) -> PLLSAI2N_W<PLLSAI2CFGRrs> {
        PLLSAI2N_W::new(self, 8)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2pen(&mut self) -> PLLSAI2PEN_W<PLLSAI2CFGRrs> {
        PLLSAI2PEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2p(&mut self) -> PLLSAI2P_W<PLLSAI2CFGRrs> {
        PLLSAI2P_W::new(self, 17)
    }
    #[doc = "Bit 20 - PLLSAI2 division factor for PLLDISCLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2qen(&mut self) -> PLLSAI2QEN_W<PLLSAI2CFGRrs> {
        PLLSAI2QEN_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2q(&mut self) -> PLLSAI2Q_W<PLLSAI2CFGRrs> {
        PLLSAI2Q_W::new(self, 21)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2ren(&mut self) -> PLLSAI2REN_W<PLLSAI2CFGRrs> {
        PLLSAI2REN_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2r(&mut self) -> PLLSAI2R_W<PLLSAI2CFGRrs> {
        PLLSAI2R_W::new(self, 25)
    }
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2pdiv(&mut self) -> PLLSAI2PDIV_W<PLLSAI2CFGRrs> {
        PLLSAI2PDIV_W::new(self, 27)
    }
}
#[doc = "PLLSAI2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsai2cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsai2cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLSAI2CFGRrs;
impl crate::RegisterSpec for PLLSAI2CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsai2cfgr::R`](R) reader structure"]
impl crate::Readable for PLLSAI2CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pllsai2cfgr::W`](W) writer structure"]
impl crate::Writable for PLLSAI2CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLSAI2CFGR to value 0x1000"]
impl crate::Resettable for PLLSAI2CFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
