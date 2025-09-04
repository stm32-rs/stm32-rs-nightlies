///Register `PLLSAI1CFGR` reader
pub type R = crate::R<PLLSAI1CFGRrs>;
///Register `PLLSAI1CFGR` writer
pub type W = crate::W<PLLSAI1CFGRrs>;
/**Division factor for PLLSAI1 input clock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI1M {
    ///0: PLLSAI1M = 1
    Div1 = 0,
    ///1: PLLSAI1M = 2
    Div2 = 1,
    ///2: PLLSAI1M = 3
    Div3 = 2,
    ///3: PLLSAI1M = 4
    Div4 = 3,
    ///4: PLLSAI1M = 5
    Div5 = 4,
    ///5: PLLSAI1M = 6
    Div6 = 5,
    ///6: PLLSAI1M = 7
    Div7 = 6,
    ///7: PLLSAI1M = 8
    Div8 = 7,
    ///8: PLLSAI1M = 9
    Div9 = 8,
    ///9: PLLSAI1M = 11
    Div10 = 9,
    ///10: PLLSAI1M = 12
    Div11 = 10,
    ///11: PLLSAI1M = 13
    Div12 = 11,
    ///12: PLLSAI1M = 13
    Div13 = 12,
    ///13: PLLSAI1M = 14
    Div14 = 13,
    ///14: PLLSAI1M = 15
    Div15 = 14,
    ///15: PLLSAI1M = 16
    Div16 = 15,
}
impl From<PLLSAI1M> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI1M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAI1M {
    type Ux = u8;
}
impl crate::IsEnum for PLLSAI1M {}
///Field `PLLSAI1M` reader - Division factor for PLLSAI1 input clock
pub type PLLSAI1M_R = crate::FieldReader<PLLSAI1M>;
impl PLLSAI1M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1M {
        match self.bits {
            0 => PLLSAI1M::Div1,
            1 => PLLSAI1M::Div2,
            2 => PLLSAI1M::Div3,
            3 => PLLSAI1M::Div4,
            4 => PLLSAI1M::Div5,
            5 => PLLSAI1M::Div6,
            6 => PLLSAI1M::Div7,
            7 => PLLSAI1M::Div8,
            8 => PLLSAI1M::Div9,
            9 => PLLSAI1M::Div10,
            10 => PLLSAI1M::Div11,
            11 => PLLSAI1M::Div12,
            12 => PLLSAI1M::Div13,
            13 => PLLSAI1M::Div14,
            14 => PLLSAI1M::Div15,
            15 => PLLSAI1M::Div16,
            _ => unreachable!(),
        }
    }
    ///PLLSAI1M = 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLSAI1M::Div1
    }
    ///PLLSAI1M = 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI1M::Div2
    }
    ///PLLSAI1M = 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAI1M::Div3
    }
    ///PLLSAI1M = 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI1M::Div4
    }
    ///PLLSAI1M = 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAI1M::Div5
    }
    ///PLLSAI1M = 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI1M::Div6
    }
    ///PLLSAI1M = 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI1M::Div7
    }
    ///PLLSAI1M = 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI1M::Div8
    }
    ///PLLSAI1M = 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAI1M::Div9
    }
    ///PLLSAI1M = 11
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAI1M::Div10
    }
    ///PLLSAI1M = 12
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAI1M::Div11
    }
    ///PLLSAI1M = 13
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAI1M::Div12
    }
    ///PLLSAI1M = 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAI1M::Div13
    }
    ///PLLSAI1M = 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAI1M::Div14
    }
    ///PLLSAI1M = 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAI1M::Div15
    }
    ///PLLSAI1M = 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI1M::Div16
    }
}
///Field `PLLSAI1M` writer - Division factor for PLLSAI1 input clock
pub type PLLSAI1M_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLLSAI1M, crate::Safe>;
impl<'a, REG> PLLSAI1M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLSAI1M = 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div1)
    }
    ///PLLSAI1M = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div2)
    }
    ///PLLSAI1M = 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div3)
    }
    ///PLLSAI1M = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div4)
    }
    ///PLLSAI1M = 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div5)
    }
    ///PLLSAI1M = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div6)
    }
    ///PLLSAI1M = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div7)
    }
    ///PLLSAI1M = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div8)
    }
    ///PLLSAI1M = 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div9)
    }
    ///PLLSAI1M = 11
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div10)
    }
    ///PLLSAI1M = 12
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div11)
    }
    ///PLLSAI1M = 13
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div12)
    }
    ///PLLSAI1M = 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div13)
    }
    ///PLLSAI1M = 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div14)
    }
    ///PLLSAI1M = 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div15)
    }
    ///PLLSAI1M = 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1M::Div16)
    }
}
///Field `PLLSAI1N` reader - SAI1PLL multiplication factor for VCO
pub type PLLSAI1N_R = crate::FieldReader;
///Field `PLLSAI1N` writer - SAI1PLL multiplication factor for VCO
pub type PLLSAI1N_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**SAI1PLL PLLSAI1CLK output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1PEN {
    ///0: PLLSAI1CLK output disable
    Disabled = 0,
    ///1: PLLSAI1CLK output enabled
    Enabled = 1,
}
impl From<PLLSAI1PEN> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1PEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1PEN` reader - SAI1PLL PLLSAI1CLK output enable
pub type PLLSAI1PEN_R = crate::BitReader<PLLSAI1PEN>;
impl PLLSAI1PEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1PEN {
        match self.bits {
            false => PLLSAI1PEN::Disabled,
            true => PLLSAI1PEN::Enabled,
        }
    }
    ///PLLSAI1CLK output disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1PEN::Disabled
    }
    ///PLLSAI1CLK output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1PEN::Enabled
    }
}
///Field `PLLSAI1PEN` writer - SAI1PLL PLLSAI1CLK output enable
pub type PLLSAI1PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI1PEN>;
impl<'a, REG> PLLSAI1PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLSAI1CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PEN::Disabled)
    }
    ///PLLSAI1CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PEN::Enabled)
    }
}
/**SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1P {
    ///0: PLLSAI1P = 7
    Div7 = 0,
    ///1: PLLSAI1P = 17
    Div17 = 1,
}
impl From<PLLSAI1P> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1P) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1P` reader - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
pub type PLLSAI1P_R = crate::BitReader<PLLSAI1P>;
impl PLLSAI1P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1P {
        match self.bits {
            false => PLLSAI1P::Div7,
            true => PLLSAI1P::Div17,
        }
    }
    ///PLLSAI1P = 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI1P::Div7
    }
    ///PLLSAI1P = 17
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAI1P::Div17
    }
}
///Field `PLLSAI1P` writer - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
pub type PLLSAI1P_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI1P>;
impl<'a, REG> PLLSAI1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLSAI1P = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1P::Div7)
    }
    ///PLLSAI1P = 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1P::Div17)
    }
}
/**SAI1PLL PLLUSB2CLK output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1QEN {
    ///0: PLL48M2CLK output disable
    Disabled = 0,
    ///1: PLL48M2CLK output enabled
    Enabled = 1,
}
impl From<PLLSAI1QEN> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1QEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1QEN` reader - SAI1PLL PLLUSB2CLK output enable
pub type PLLSAI1QEN_R = crate::BitReader<PLLSAI1QEN>;
impl PLLSAI1QEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1QEN {
        match self.bits {
            false => PLLSAI1QEN::Disabled,
            true => PLLSAI1QEN::Enabled,
        }
    }
    ///PLL48M2CLK output disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1QEN::Disabled
    }
    ///PLL48M2CLK output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1QEN::Enabled
    }
}
///Field `PLLSAI1QEN` writer - SAI1PLL PLLUSB2CLK output enable
pub type PLLSAI1QEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI1QEN>;
impl<'a, REG> PLLSAI1QEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL48M2CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1QEN::Disabled)
    }
    ///PLL48M2CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1QEN::Enabled)
    }
}
/**SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI1Q {
    ///0: PLLSAI1x = 2
    Div2 = 0,
    ///1: PLLSAI1x = 4
    Div4 = 1,
    ///2: PLLSAI1x = 6
    Div6 = 2,
    ///3: PLLSAI1x = 8
    Div8 = 3,
}
impl From<PLLSAI1Q> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI1Q) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAI1Q {
    type Ux = u8;
}
impl crate::IsEnum for PLLSAI1Q {}
///Field `PLLSAI1Q` reader - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
pub type PLLSAI1Q_R = crate::FieldReader<PLLSAI1Q>;
impl PLLSAI1Q_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1Q {
        match self.bits {
            0 => PLLSAI1Q::Div2,
            1 => PLLSAI1Q::Div4,
            2 => PLLSAI1Q::Div6,
            3 => PLLSAI1Q::Div8,
            _ => unreachable!(),
        }
    }
    ///PLLSAI1x = 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI1Q::Div2
    }
    ///PLLSAI1x = 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI1Q::Div4
    }
    ///PLLSAI1x = 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI1Q::Div6
    }
    ///PLLSAI1x = 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI1Q::Div8
    }
}
///Field `PLLSAI1Q` writer - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
pub type PLLSAI1Q_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLSAI1Q, crate::Safe>;
impl<'a, REG> PLLSAI1Q_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLSAI1x = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1Q::Div2)
    }
    ///PLLSAI1x = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1Q::Div4)
    }
    ///PLLSAI1x = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1Q::Div6)
    }
    ///PLLSAI1x = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1Q::Div8)
    }
}
/**PLLSAI1 PLLADC1CLK output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1REN {
    ///0: PLLADC1CLK output disable
    Disabled = 0,
    ///1: PLLADC1CLK output enabled
    Enabled = 1,
}
impl From<PLLSAI1REN> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1REN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1REN` reader - PLLSAI1 PLLADC1CLK output enable
pub type PLLSAI1REN_R = crate::BitReader<PLLSAI1REN>;
impl PLLSAI1REN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1REN {
        match self.bits {
            false => PLLSAI1REN::Disabled,
            true => PLLSAI1REN::Enabled,
        }
    }
    ///PLLADC1CLK output disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1REN::Disabled
    }
    ///PLLADC1CLK output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1REN::Enabled
    }
}
///Field `PLLSAI1REN` writer - PLLSAI1 PLLADC1CLK output enable
pub type PLLSAI1REN_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI1REN>;
impl<'a, REG> PLLSAI1REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLADC1CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1REN::Disabled)
    }
    ///PLLADC1CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1REN::Enabled)
    }
}
///Field `PLLSAI1R` reader - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
pub use PLLSAI1Q_R as PLLSAI1R_R;
///Field `PLLSAI1R` writer - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
pub use PLLSAI1Q_W as PLLSAI1R_W;
/**PLLSAI1 division factor for PLLSAI1CLK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI1PDIV {
    ///0: PLLSAI1CLK is controlled by the bit PLLSAI1P
    Pllsai1p = 0,
    ///2: PLLSAI1CLK = VCOSAI / 2
    Div2 = 2,
    ///3: PLLSAI1CLK = VCOSAI / 3
    Div3 = 3,
    ///4: PLLSAI1CLK = VCOSAI / 4
    Div4 = 4,
    ///5: PLLSAI1CLK = VCOSAI / 5
    Div5 = 5,
    ///6: PLLSAI1CLK = VCOSAI / 6
    Div6 = 6,
    ///7: PLLSAI1CLK = VCOSAI / 7
    Div7 = 7,
    ///8: PLLSAI1CLK = VCOSAI / 8
    Div8 = 8,
    ///9: PLLSAI1CLK = VCOSAI / 9
    Div9 = 9,
    ///10: PLLSAI1CLK = VCOSAI / 10
    Div10 = 10,
    ///11: PLLSAI1CLK = VCOSAI / 11
    Div11 = 11,
    ///12: PLLSAI1CLK = VCOSAI / 12
    Div12 = 12,
    ///13: PLLSAI1CLK = VCOSAI / 13
    Div13 = 13,
    ///14: PLLSAI1CLK = VCOSAI / 14
    Div14 = 14,
    ///15: PLLSAI1CLK = VCOSAI / 15
    Div15 = 15,
    ///16: PLLSAI1CLK = VCOSAI / 16
    Div16 = 16,
    ///17: PLLSAI1CLK = VCOSAI / 17
    Div17 = 17,
    ///18: PLLSAI1CLK = VCOSAI / 18
    Div18 = 18,
    ///19: PLLSAI1CLK = VCOSAI / 19
    Div19 = 19,
    ///20: PLLSAI1CLK = VCOSAI / 20
    Div20 = 20,
    ///21: PLLSAI1CLK = VCOSAI / 21
    Div21 = 21,
    ///22: PLLSAI1CLK = VCOSAI / 22
    Div22 = 22,
    ///23: PLLSAI1CLK = VCOSAI / 23
    Div23 = 23,
    ///24: PLLSAI1CLK = VCOSAI / 24
    Div24 = 24,
    ///25: PLLSAI1CLK = VCOSAI / 25
    Div25 = 25,
    ///26: PLLSAI1CLK = VCOSAI / 26
    Div26 = 26,
    ///27: PLLSAI1CLK = VCOSAI / 27
    Div27 = 27,
    ///28: PLLSAI1CLK = VCOSAI / 28
    Div28 = 28,
    ///29: PLLSAI1CLK = VCOSAI / 29
    Div29 = 29,
    ///30: PLLSAI1CLK = VCOSAI / 30
    Div30 = 30,
    ///31: PLLSAI1CLK = VCOSAI / 31
    Div31 = 31,
}
impl From<PLLSAI1PDIV> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI1PDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAI1PDIV {
    type Ux = u8;
}
impl crate::IsEnum for PLLSAI1PDIV {}
///Field `PLLSAI1PDIV` reader - PLLSAI1 division factor for PLLSAI1CLK
pub type PLLSAI1PDIV_R = crate::FieldReader<PLLSAI1PDIV>;
impl PLLSAI1PDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLSAI1PDIV> {
        match self.bits {
            0 => Some(PLLSAI1PDIV::Pllsai1p),
            2 => Some(PLLSAI1PDIV::Div2),
            3 => Some(PLLSAI1PDIV::Div3),
            4 => Some(PLLSAI1PDIV::Div4),
            5 => Some(PLLSAI1PDIV::Div5),
            6 => Some(PLLSAI1PDIV::Div6),
            7 => Some(PLLSAI1PDIV::Div7),
            8 => Some(PLLSAI1PDIV::Div8),
            9 => Some(PLLSAI1PDIV::Div9),
            10 => Some(PLLSAI1PDIV::Div10),
            11 => Some(PLLSAI1PDIV::Div11),
            12 => Some(PLLSAI1PDIV::Div12),
            13 => Some(PLLSAI1PDIV::Div13),
            14 => Some(PLLSAI1PDIV::Div14),
            15 => Some(PLLSAI1PDIV::Div15),
            16 => Some(PLLSAI1PDIV::Div16),
            17 => Some(PLLSAI1PDIV::Div17),
            18 => Some(PLLSAI1PDIV::Div18),
            19 => Some(PLLSAI1PDIV::Div19),
            20 => Some(PLLSAI1PDIV::Div20),
            21 => Some(PLLSAI1PDIV::Div21),
            22 => Some(PLLSAI1PDIV::Div22),
            23 => Some(PLLSAI1PDIV::Div23),
            24 => Some(PLLSAI1PDIV::Div24),
            25 => Some(PLLSAI1PDIV::Div25),
            26 => Some(PLLSAI1PDIV::Div26),
            27 => Some(PLLSAI1PDIV::Div27),
            28 => Some(PLLSAI1PDIV::Div28),
            29 => Some(PLLSAI1PDIV::Div29),
            30 => Some(PLLSAI1PDIV::Div30),
            31 => Some(PLLSAI1PDIV::Div31),
            _ => None,
        }
    }
    ///PLLSAI1CLK is controlled by the bit PLLSAI1P
    #[inline(always)]
    pub fn is_pllsai1p(&self) -> bool {
        *self == PLLSAI1PDIV::Pllsai1p
    }
    ///PLLSAI1CLK = VCOSAI / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI1PDIV::Div2
    }
    ///PLLSAI1CLK = VCOSAI / 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAI1PDIV::Div3
    }
    ///PLLSAI1CLK = VCOSAI / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI1PDIV::Div4
    }
    ///PLLSAI1CLK = VCOSAI / 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAI1PDIV::Div5
    }
    ///PLLSAI1CLK = VCOSAI / 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI1PDIV::Div6
    }
    ///PLLSAI1CLK = VCOSAI / 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI1PDIV::Div7
    }
    ///PLLSAI1CLK = VCOSAI / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI1PDIV::Div8
    }
    ///PLLSAI1CLK = VCOSAI / 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAI1PDIV::Div9
    }
    ///PLLSAI1CLK = VCOSAI / 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAI1PDIV::Div10
    }
    ///PLLSAI1CLK = VCOSAI / 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAI1PDIV::Div11
    }
    ///PLLSAI1CLK = VCOSAI / 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAI1PDIV::Div12
    }
    ///PLLSAI1CLK = VCOSAI / 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAI1PDIV::Div13
    }
    ///PLLSAI1CLK = VCOSAI / 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAI1PDIV::Div14
    }
    ///PLLSAI1CLK = VCOSAI / 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAI1PDIV::Div15
    }
    ///PLLSAI1CLK = VCOSAI / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI1PDIV::Div16
    }
    ///PLLSAI1CLK = VCOSAI / 17
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAI1PDIV::Div17
    }
    ///PLLSAI1CLK = VCOSAI / 18
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLSAI1PDIV::Div18
    }
    ///PLLSAI1CLK = VCOSAI / 19
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLSAI1PDIV::Div19
    }
    ///PLLSAI1CLK = VCOSAI / 20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLSAI1PDIV::Div20
    }
    ///PLLSAI1CLK = VCOSAI / 21
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLSAI1PDIV::Div21
    }
    ///PLLSAI1CLK = VCOSAI / 22
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLSAI1PDIV::Div22
    }
    ///PLLSAI1CLK = VCOSAI / 23
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLSAI1PDIV::Div23
    }
    ///PLLSAI1CLK = VCOSAI / 24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLSAI1PDIV::Div24
    }
    ///PLLSAI1CLK = VCOSAI / 25
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLSAI1PDIV::Div25
    }
    ///PLLSAI1CLK = VCOSAI / 26
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLSAI1PDIV::Div26
    }
    ///PLLSAI1CLK = VCOSAI / 27
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLSAI1PDIV::Div27
    }
    ///PLLSAI1CLK = VCOSAI / 28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLSAI1PDIV::Div28
    }
    ///PLLSAI1CLK = VCOSAI / 29
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLSAI1PDIV::Div29
    }
    ///PLLSAI1CLK = VCOSAI / 30
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLSAI1PDIV::Div30
    }
    ///PLLSAI1CLK = VCOSAI / 31
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLSAI1PDIV::Div31
    }
}
///Field `PLLSAI1PDIV` writer - PLLSAI1 division factor for PLLSAI1CLK
pub type PLLSAI1PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PLLSAI1PDIV>;
impl<'a, REG> PLLSAI1PDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLSAI1CLK is controlled by the bit PLLSAI1P
    #[inline(always)]
    pub fn pllsai1p(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Pllsai1p)
    }
    ///PLLSAI1CLK = VCOSAI / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div2)
    }
    ///PLLSAI1CLK = VCOSAI / 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div3)
    }
    ///PLLSAI1CLK = VCOSAI / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div4)
    }
    ///PLLSAI1CLK = VCOSAI / 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div5)
    }
    ///PLLSAI1CLK = VCOSAI / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div6)
    }
    ///PLLSAI1CLK = VCOSAI / 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div7)
    }
    ///PLLSAI1CLK = VCOSAI / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div8)
    }
    ///PLLSAI1CLK = VCOSAI / 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div9)
    }
    ///PLLSAI1CLK = VCOSAI / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div10)
    }
    ///PLLSAI1CLK = VCOSAI / 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div11)
    }
    ///PLLSAI1CLK = VCOSAI / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div12)
    }
    ///PLLSAI1CLK = VCOSAI / 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div13)
    }
    ///PLLSAI1CLK = VCOSAI / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div14)
    }
    ///PLLSAI1CLK = VCOSAI / 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div15)
    }
    ///PLLSAI1CLK = VCOSAI / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div16)
    }
    ///PLLSAI1CLK = VCOSAI / 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div17)
    }
    ///PLLSAI1CLK = VCOSAI / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div18)
    }
    ///PLLSAI1CLK = VCOSAI / 19
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div19)
    }
    ///PLLSAI1CLK = VCOSAI / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div20)
    }
    ///PLLSAI1CLK = VCOSAI / 21
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div21)
    }
    ///PLLSAI1CLK = VCOSAI / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div22)
    }
    ///PLLSAI1CLK = VCOSAI / 23
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div23)
    }
    ///PLLSAI1CLK = VCOSAI / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div24)
    }
    ///PLLSAI1CLK = VCOSAI / 25
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div25)
    }
    ///PLLSAI1CLK = VCOSAI / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div26)
    }
    ///PLLSAI1CLK = VCOSAI / 27
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div27)
    }
    ///PLLSAI1CLK = VCOSAI / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div28)
    }
    ///PLLSAI1CLK = VCOSAI / 29
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div29)
    }
    ///PLLSAI1CLK = VCOSAI / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div30)
    }
    ///PLLSAI1CLK = VCOSAI / 31
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1PDIV::Div31)
    }
}
impl R {
    ///Bits 4:7 - Division factor for PLLSAI1 input clock
    #[inline(always)]
    pub fn pllsai1m(&self) -> PLLSAI1M_R {
        PLLSAI1M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:14 - SAI1PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai1n(&self) -> PLLSAI1N_R {
        PLLSAI1N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - SAI1PLL PLLSAI1CLK output enable
    #[inline(always)]
    pub fn pllsai1pen(&self) -> PLLSAI1PEN_R {
        PLLSAI1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai1p(&self) -> PLLSAI1P_R {
        PLLSAI1P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - SAI1PLL PLLUSB2CLK output enable
    #[inline(always)]
    pub fn pllsai1qen(&self) -> PLLSAI1QEN_R {
        PLLSAI1QEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
    #[inline(always)]
    pub fn pllsai1q(&self) -> PLLSAI1Q_R {
        PLLSAI1Q_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - PLLSAI1 PLLADC1CLK output enable
    #[inline(always)]
    pub fn pllsai1ren(&self) -> PLLSAI1REN_R {
        PLLSAI1REN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
    #[inline(always)]
    pub fn pllsai1r(&self) -> PLLSAI1R_R {
        PLLSAI1R_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK
    #[inline(always)]
    pub fn pllsai1pdiv(&self) -> PLLSAI1PDIV_R {
        PLLSAI1PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLSAI1CFGR")
            .field("pllsai1pdiv", &self.pllsai1pdiv())
            .field("pllsai1q", &self.pllsai1q())
            .field("pllsai1r", &self.pllsai1r())
            .field("pllsai1ren", &self.pllsai1ren())
            .field("pllsai1qen", &self.pllsai1qen())
            .field("pllsai1p", &self.pllsai1p())
            .field("pllsai1pen", &self.pllsai1pen())
            .field("pllsai1n", &self.pllsai1n())
            .field("pllsai1m", &self.pllsai1m())
            .finish()
    }
}
impl W {
    ///Bits 4:7 - Division factor for PLLSAI1 input clock
    #[inline(always)]
    pub fn pllsai1m(&mut self) -> PLLSAI1M_W<PLLSAI1CFGRrs> {
        PLLSAI1M_W::new(self, 4)
    }
    ///Bits 8:14 - SAI1PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai1n(&mut self) -> PLLSAI1N_W<PLLSAI1CFGRrs> {
        PLLSAI1N_W::new(self, 8)
    }
    ///Bit 16 - SAI1PLL PLLSAI1CLK output enable
    #[inline(always)]
    pub fn pllsai1pen(&mut self) -> PLLSAI1PEN_W<PLLSAI1CFGRrs> {
        PLLSAI1PEN_W::new(self, 16)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai1p(&mut self) -> PLLSAI1P_W<PLLSAI1CFGRrs> {
        PLLSAI1P_W::new(self, 17)
    }
    ///Bit 20 - SAI1PLL PLLUSB2CLK output enable
    #[inline(always)]
    pub fn pllsai1qen(&mut self) -> PLLSAI1QEN_W<PLLSAI1CFGRrs> {
        PLLSAI1QEN_W::new(self, 20)
    }
    ///Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
    #[inline(always)]
    pub fn pllsai1q(&mut self) -> PLLSAI1Q_W<PLLSAI1CFGRrs> {
        PLLSAI1Q_W::new(self, 21)
    }
    ///Bit 24 - PLLSAI1 PLLADC1CLK output enable
    #[inline(always)]
    pub fn pllsai1ren(&mut self) -> PLLSAI1REN_W<PLLSAI1CFGRrs> {
        PLLSAI1REN_W::new(self, 24)
    }
    ///Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
    #[inline(always)]
    pub fn pllsai1r(&mut self) -> PLLSAI1R_W<PLLSAI1CFGRrs> {
        PLLSAI1R_W::new(self, 25)
    }
    ///Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK
    #[inline(always)]
    pub fn pllsai1pdiv(&mut self) -> PLLSAI1PDIV_W<PLLSAI1CFGRrs> {
        PLLSAI1PDIV_W::new(self, 27)
    }
}
/**PLLSAI1 configuration register

You can [`read`](crate::Reg::read) this register and get [`pllsai1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:PLLSAI1CFGR)*/
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
///`reset()` method sets PLLSAI1CFGR to value 0x1000
impl crate::Resettable for PLLSAI1CFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
