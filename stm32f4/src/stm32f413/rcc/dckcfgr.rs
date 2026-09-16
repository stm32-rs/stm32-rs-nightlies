///Register `DCKCFGR` reader
pub type R = crate::R<DCKCFGRrs>;
///Register `DCKCFGR` writer
pub type W = crate::W<DCKCFGRrs>;
/**PLLI2S division factor for SAI1 A/B clock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLI2SDIVR {
    ///0: PLLI2SDIVQ = /1
    Div1 = 0,
    ///1: PLLI2SDIVQ = /2
    Div2 = 1,
    ///2: PLLI2SDIVQ = /3
    Div3 = 2,
    ///3: PLLI2SDIVQ = /4
    Div4 = 3,
    ///4: PLLI2SDIVQ = /5
    Div5 = 4,
    ///5: PLLI2SDIVQ = /6
    Div6 = 5,
    ///6: PLLI2SDIVQ = /7
    Div7 = 6,
    ///7: PLLI2SDIVQ = /8
    Div8 = 7,
    ///8: PLLI2SDIVQ = /9
    Div9 = 8,
    ///9: PLLI2SDIVQ = /10
    Div10 = 9,
    ///10: PLLI2SDIVQ = /11
    Div11 = 10,
    ///11: PLLI2SDIVQ = /12
    Div12 = 11,
    ///12: PLLI2SDIVQ = /13
    Div13 = 12,
    ///13: PLLI2SDIVQ = /14
    Div14 = 13,
    ///14: PLLI2SDIVQ = /15
    Div15 = 14,
    ///15: PLLI2SDIVQ = /16
    Div16 = 15,
    ///16: PLLI2SDIVQ = /17
    Div17 = 16,
    ///17: PLLI2SDIVQ = /18
    Div18 = 17,
    ///18: PLLI2SDIVQ = /19
    Div19 = 18,
    ///19: PLLI2SDIVQ = /20
    Div20 = 19,
    ///20: PLLI2SDIVQ = /21
    Div21 = 20,
    ///21: PLLI2SDIVQ = /22
    Div22 = 21,
    ///22: PLLI2SDIVQ = /23
    Div23 = 22,
    ///23: PLLI2SDIVQ = /24
    Div24 = 23,
    ///24: PLLI2SDIVQ = /25
    Div25 = 24,
    ///25: PLLI2SDIVQ = /26
    Div26 = 25,
    ///26: PLLI2SDIVQ = /27
    Div27 = 26,
    ///27: PLLI2SDIVQ = /28
    Div28 = 27,
    ///28: PLLI2SDIVQ = /29
    Div29 = 28,
    ///29: PLLI2SDIVQ = /30
    Div30 = 29,
    ///30: PLLI2SDIVQ = /31
    Div31 = 30,
    ///31: PLLI2SDIVQ = /32
    Div32 = 31,
}
impl From<PLLI2SDIVR> for u8 {
    #[inline(always)]
    fn from(variant: PLLI2SDIVR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLI2SDIVR {
    type Ux = u8;
}
impl crate::IsEnum for PLLI2SDIVR {}
///Field `PLLI2SDIVR` reader - PLLI2S division factor for SAI1 A/B clock
pub type PLLI2SDIVR_R = crate::FieldReader<PLLI2SDIVR>;
impl PLLI2SDIVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLI2SDIVR {
        match self.bits {
            0 => PLLI2SDIVR::Div1,
            1 => PLLI2SDIVR::Div2,
            2 => PLLI2SDIVR::Div3,
            3 => PLLI2SDIVR::Div4,
            4 => PLLI2SDIVR::Div5,
            5 => PLLI2SDIVR::Div6,
            6 => PLLI2SDIVR::Div7,
            7 => PLLI2SDIVR::Div8,
            8 => PLLI2SDIVR::Div9,
            9 => PLLI2SDIVR::Div10,
            10 => PLLI2SDIVR::Div11,
            11 => PLLI2SDIVR::Div12,
            12 => PLLI2SDIVR::Div13,
            13 => PLLI2SDIVR::Div14,
            14 => PLLI2SDIVR::Div15,
            15 => PLLI2SDIVR::Div16,
            16 => PLLI2SDIVR::Div17,
            17 => PLLI2SDIVR::Div18,
            18 => PLLI2SDIVR::Div19,
            19 => PLLI2SDIVR::Div20,
            20 => PLLI2SDIVR::Div21,
            21 => PLLI2SDIVR::Div22,
            22 => PLLI2SDIVR::Div23,
            23 => PLLI2SDIVR::Div24,
            24 => PLLI2SDIVR::Div25,
            25 => PLLI2SDIVR::Div26,
            26 => PLLI2SDIVR::Div27,
            27 => PLLI2SDIVR::Div28,
            28 => PLLI2SDIVR::Div29,
            29 => PLLI2SDIVR::Div30,
            30 => PLLI2SDIVR::Div31,
            31 => PLLI2SDIVR::Div32,
            _ => unreachable!(),
        }
    }
    ///PLLI2SDIVQ = /1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLI2SDIVR::Div1
    }
    ///PLLI2SDIVQ = /2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLI2SDIVR::Div2
    }
    ///PLLI2SDIVQ = /3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLI2SDIVR::Div3
    }
    ///PLLI2SDIVQ = /4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLI2SDIVR::Div4
    }
    ///PLLI2SDIVQ = /5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLI2SDIVR::Div5
    }
    ///PLLI2SDIVQ = /6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLI2SDIVR::Div6
    }
    ///PLLI2SDIVQ = /7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLI2SDIVR::Div7
    }
    ///PLLI2SDIVQ = /8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLI2SDIVR::Div8
    }
    ///PLLI2SDIVQ = /9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLI2SDIVR::Div9
    }
    ///PLLI2SDIVQ = /10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLI2SDIVR::Div10
    }
    ///PLLI2SDIVQ = /11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLI2SDIVR::Div11
    }
    ///PLLI2SDIVQ = /12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLI2SDIVR::Div12
    }
    ///PLLI2SDIVQ = /13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLI2SDIVR::Div13
    }
    ///PLLI2SDIVQ = /14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLI2SDIVR::Div14
    }
    ///PLLI2SDIVQ = /15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLI2SDIVR::Div15
    }
    ///PLLI2SDIVQ = /16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLI2SDIVR::Div16
    }
    ///PLLI2SDIVQ = /17
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLI2SDIVR::Div17
    }
    ///PLLI2SDIVQ = /18
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLI2SDIVR::Div18
    }
    ///PLLI2SDIVQ = /19
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLI2SDIVR::Div19
    }
    ///PLLI2SDIVQ = /20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLI2SDIVR::Div20
    }
    ///PLLI2SDIVQ = /21
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLI2SDIVR::Div21
    }
    ///PLLI2SDIVQ = /22
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLI2SDIVR::Div22
    }
    ///PLLI2SDIVQ = /23
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLI2SDIVR::Div23
    }
    ///PLLI2SDIVQ = /24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLI2SDIVR::Div24
    }
    ///PLLI2SDIVQ = /25
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLI2SDIVR::Div25
    }
    ///PLLI2SDIVQ = /26
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLI2SDIVR::Div26
    }
    ///PLLI2SDIVQ = /27
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLI2SDIVR::Div27
    }
    ///PLLI2SDIVQ = /28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLI2SDIVR::Div28
    }
    ///PLLI2SDIVQ = /29
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLI2SDIVR::Div29
    }
    ///PLLI2SDIVQ = /30
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLI2SDIVR::Div30
    }
    ///PLLI2SDIVQ = /31
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLI2SDIVR::Div31
    }
    ///PLLI2SDIVQ = /32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLI2SDIVR::Div32
    }
}
///Field `PLLI2SDIVR` writer - PLLI2S division factor for SAI1 A/B clock
pub type PLLI2SDIVR_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PLLI2SDIVR, crate::Safe>;
impl<'a, REG> PLLI2SDIVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLI2SDIVQ = /1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div1)
    }
    ///PLLI2SDIVQ = /2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div2)
    }
    ///PLLI2SDIVQ = /3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div3)
    }
    ///PLLI2SDIVQ = /4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div4)
    }
    ///PLLI2SDIVQ = /5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div5)
    }
    ///PLLI2SDIVQ = /6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div6)
    }
    ///PLLI2SDIVQ = /7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div7)
    }
    ///PLLI2SDIVQ = /8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div8)
    }
    ///PLLI2SDIVQ = /9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div9)
    }
    ///PLLI2SDIVQ = /10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div10)
    }
    ///PLLI2SDIVQ = /11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div11)
    }
    ///PLLI2SDIVQ = /12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div12)
    }
    ///PLLI2SDIVQ = /13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div13)
    }
    ///PLLI2SDIVQ = /14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div14)
    }
    ///PLLI2SDIVQ = /15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div15)
    }
    ///PLLI2SDIVQ = /16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div16)
    }
    ///PLLI2SDIVQ = /17
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div17)
    }
    ///PLLI2SDIVQ = /18
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div18)
    }
    ///PLLI2SDIVQ = /19
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div19)
    }
    ///PLLI2SDIVQ = /20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div20)
    }
    ///PLLI2SDIVQ = /21
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div21)
    }
    ///PLLI2SDIVQ = /22
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div22)
    }
    ///PLLI2SDIVQ = /23
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div23)
    }
    ///PLLI2SDIVQ = /24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div24)
    }
    ///PLLI2SDIVQ = /25
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div25)
    }
    ///PLLI2SDIVQ = /26
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div26)
    }
    ///PLLI2SDIVQ = /27
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div27)
    }
    ///PLLI2SDIVQ = /28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div28)
    }
    ///PLLI2SDIVQ = /29
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div29)
    }
    ///PLLI2SDIVQ = /30
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div30)
    }
    ///PLLI2SDIVQ = /31
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div31)
    }
    ///PLLI2SDIVQ = /32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVR::Div32)
    }
}
/**PLL division factor for SAI1 A/B clock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLDIVR {
    ///0: PLLSAIDIVQ = /1
    Div1 = 0,
    ///1: PLLSAIDIVQ = /2
    Div2 = 1,
    ///2: PLLSAIDIVQ = /3
    Div3 = 2,
    ///3: PLLSAIDIVQ = /4
    Div4 = 3,
    ///4: PLLSAIDIVQ = /5
    Div5 = 4,
    ///5: PLLSAIDIVQ = /6
    Div6 = 5,
    ///6: PLLSAIDIVQ = /7
    Div7 = 6,
    ///7: PLLSAIDIVQ = /8
    Div8 = 7,
    ///8: PLLSAIDIVQ = /9
    Div9 = 8,
    ///9: PLLSAIDIVQ = /10
    Div10 = 9,
    ///10: PLLSAIDIVQ = /11
    Div11 = 10,
    ///11: PLLSAIDIVQ = /12
    Div12 = 11,
    ///12: PLLSAIDIVQ = /13
    Div13 = 12,
    ///13: PLLSAIDIVQ = /14
    Div14 = 13,
    ///14: PLLSAIDIVQ = /15
    Div15 = 14,
    ///15: PLLSAIDIVQ = /16
    Div16 = 15,
    ///16: PLLSAIDIVQ = /17
    Div17 = 16,
    ///17: PLLSAIDIVQ = /18
    Div18 = 17,
    ///18: PLLSAIDIVQ = /19
    Div19 = 18,
    ///19: PLLSAIDIVQ = /20
    Div20 = 19,
    ///20: PLLSAIDIVQ = /21
    Div21 = 20,
    ///21: PLLSAIDIVQ = /22
    Div22 = 21,
    ///22: PLLSAIDIVQ = /23
    Div23 = 22,
    ///23: PLLSAIDIVQ = /24
    Div24 = 23,
    ///24: PLLSAIDIVQ = /25
    Div25 = 24,
    ///25: PLLSAIDIVQ = /26
    Div26 = 25,
    ///26: PLLSAIDIVQ = /27
    Div27 = 26,
    ///27: PLLSAIDIVQ = /28
    Div28 = 27,
    ///28: PLLSAIDIVQ = /29
    Div29 = 28,
    ///29: PLLSAIDIVQ = /30
    Div30 = 29,
    ///30: PLLSAIDIVQ = /31
    Div31 = 30,
    ///31: PLLSAIDIVQ = /32
    Div32 = 31,
}
impl From<PLLDIVR> for u8 {
    #[inline(always)]
    fn from(variant: PLLDIVR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLDIVR {
    type Ux = u8;
}
impl crate::IsEnum for PLLDIVR {}
///Field `PLLDIVR` reader - PLL division factor for SAI1 A/B clock
pub type PLLDIVR_R = crate::FieldReader<PLLDIVR>;
impl PLLDIVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLDIVR {
        match self.bits {
            0 => PLLDIVR::Div1,
            1 => PLLDIVR::Div2,
            2 => PLLDIVR::Div3,
            3 => PLLDIVR::Div4,
            4 => PLLDIVR::Div5,
            5 => PLLDIVR::Div6,
            6 => PLLDIVR::Div7,
            7 => PLLDIVR::Div8,
            8 => PLLDIVR::Div9,
            9 => PLLDIVR::Div10,
            10 => PLLDIVR::Div11,
            11 => PLLDIVR::Div12,
            12 => PLLDIVR::Div13,
            13 => PLLDIVR::Div14,
            14 => PLLDIVR::Div15,
            15 => PLLDIVR::Div16,
            16 => PLLDIVR::Div17,
            17 => PLLDIVR::Div18,
            18 => PLLDIVR::Div19,
            19 => PLLDIVR::Div20,
            20 => PLLDIVR::Div21,
            21 => PLLDIVR::Div22,
            22 => PLLDIVR::Div23,
            23 => PLLDIVR::Div24,
            24 => PLLDIVR::Div25,
            25 => PLLDIVR::Div26,
            26 => PLLDIVR::Div27,
            27 => PLLDIVR::Div28,
            28 => PLLDIVR::Div29,
            29 => PLLDIVR::Div30,
            30 => PLLDIVR::Div31,
            31 => PLLDIVR::Div32,
            _ => unreachable!(),
        }
    }
    ///PLLSAIDIVQ = /1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLDIVR::Div1
    }
    ///PLLSAIDIVQ = /2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLDIVR::Div2
    }
    ///PLLSAIDIVQ = /3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLDIVR::Div3
    }
    ///PLLSAIDIVQ = /4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLDIVR::Div4
    }
    ///PLLSAIDIVQ = /5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLDIVR::Div5
    }
    ///PLLSAIDIVQ = /6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLDIVR::Div6
    }
    ///PLLSAIDIVQ = /7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLDIVR::Div7
    }
    ///PLLSAIDIVQ = /8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLDIVR::Div8
    }
    ///PLLSAIDIVQ = /9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLDIVR::Div9
    }
    ///PLLSAIDIVQ = /10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLDIVR::Div10
    }
    ///PLLSAIDIVQ = /11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLDIVR::Div11
    }
    ///PLLSAIDIVQ = /12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLDIVR::Div12
    }
    ///PLLSAIDIVQ = /13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLDIVR::Div13
    }
    ///PLLSAIDIVQ = /14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLDIVR::Div14
    }
    ///PLLSAIDIVQ = /15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLDIVR::Div15
    }
    ///PLLSAIDIVQ = /16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLDIVR::Div16
    }
    ///PLLSAIDIVQ = /17
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLDIVR::Div17
    }
    ///PLLSAIDIVQ = /18
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLDIVR::Div18
    }
    ///PLLSAIDIVQ = /19
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLDIVR::Div19
    }
    ///PLLSAIDIVQ = /20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLDIVR::Div20
    }
    ///PLLSAIDIVQ = /21
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLDIVR::Div21
    }
    ///PLLSAIDIVQ = /22
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLDIVR::Div22
    }
    ///PLLSAIDIVQ = /23
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLDIVR::Div23
    }
    ///PLLSAIDIVQ = /24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLDIVR::Div24
    }
    ///PLLSAIDIVQ = /25
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLDIVR::Div25
    }
    ///PLLSAIDIVQ = /26
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLDIVR::Div26
    }
    ///PLLSAIDIVQ = /27
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLDIVR::Div27
    }
    ///PLLSAIDIVQ = /28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLDIVR::Div28
    }
    ///PLLSAIDIVQ = /29
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLDIVR::Div29
    }
    ///PLLSAIDIVQ = /30
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLDIVR::Div30
    }
    ///PLLSAIDIVQ = /31
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLDIVR::Div31
    }
    ///PLLSAIDIVQ = /32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLDIVR::Div32
    }
}
///Field `PLLDIVR` writer - PLL division factor for SAI1 A/B clock
pub type PLLDIVR_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PLLDIVR, crate::Safe>;
impl<'a, REG> PLLDIVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLSAIDIVQ = /1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div1)
    }
    ///PLLSAIDIVQ = /2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div2)
    }
    ///PLLSAIDIVQ = /3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div3)
    }
    ///PLLSAIDIVQ = /4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div4)
    }
    ///PLLSAIDIVQ = /5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div5)
    }
    ///PLLSAIDIVQ = /6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div6)
    }
    ///PLLSAIDIVQ = /7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div7)
    }
    ///PLLSAIDIVQ = /8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div8)
    }
    ///PLLSAIDIVQ = /9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div9)
    }
    ///PLLSAIDIVQ = /10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div10)
    }
    ///PLLSAIDIVQ = /11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div11)
    }
    ///PLLSAIDIVQ = /12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div12)
    }
    ///PLLSAIDIVQ = /13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div13)
    }
    ///PLLSAIDIVQ = /14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div14)
    }
    ///PLLSAIDIVQ = /15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div15)
    }
    ///PLLSAIDIVQ = /16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div16)
    }
    ///PLLSAIDIVQ = /17
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div17)
    }
    ///PLLSAIDIVQ = /18
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div18)
    }
    ///PLLSAIDIVQ = /19
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div19)
    }
    ///PLLSAIDIVQ = /20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div20)
    }
    ///PLLSAIDIVQ = /21
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div21)
    }
    ///PLLSAIDIVQ = /22
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div22)
    }
    ///PLLSAIDIVQ = /23
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div23)
    }
    ///PLLSAIDIVQ = /24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div24)
    }
    ///PLLSAIDIVQ = /25
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div25)
    }
    ///PLLSAIDIVQ = /26
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div26)
    }
    ///PLLSAIDIVQ = /27
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div27)
    }
    ///PLLSAIDIVQ = /28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div28)
    }
    ///PLLSAIDIVQ = /29
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div29)
    }
    ///PLLSAIDIVQ = /30
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div30)
    }
    ///PLLSAIDIVQ = /31
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div31)
    }
    ///PLLSAIDIVQ = /32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLDIVR::Div32)
    }
}
/**DFSDM2 audio clock selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKDFSDM2ASEL {
    ///0: CK_I2S_APB1 selected as audio clock
    I2s1 = 0,
    ///1: CK_I2S_APB2 selected as audio clock
    I2s2 = 1,
}
impl From<CKDFSDM2ASEL> for bool {
    #[inline(always)]
    fn from(variant: CKDFSDM2ASEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CKDFSDM2ASEL` reader - DFSDM2 audio clock selection
pub type CKDFSDM2ASEL_R = crate::BitReader<CKDFSDM2ASEL>;
impl CKDFSDM2ASEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKDFSDM2ASEL {
        match self.bits {
            false => CKDFSDM2ASEL::I2s1,
            true => CKDFSDM2ASEL::I2s2,
        }
    }
    ///CK_I2S_APB1 selected as audio clock
    #[inline(always)]
    pub fn is_i2s1(&self) -> bool {
        *self == CKDFSDM2ASEL::I2s1
    }
    ///CK_I2S_APB2 selected as audio clock
    #[inline(always)]
    pub fn is_i2s2(&self) -> bool {
        *self == CKDFSDM2ASEL::I2s2
    }
}
///Field `CKDFSDM2ASEL` writer - DFSDM2 audio clock selection
pub type CKDFSDM2ASEL_W<'a, REG> = crate::BitWriter<'a, REG, CKDFSDM2ASEL>;
impl<'a, REG> CKDFSDM2ASEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CK_I2S_APB1 selected as audio clock
    #[inline(always)]
    pub fn i2s1(self) -> &'a mut crate::W<REG> {
        self.variant(CKDFSDM2ASEL::I2s1)
    }
    ///CK_I2S_APB2 selected as audio clock
    #[inline(always)]
    pub fn i2s2(self) -> &'a mut crate::W<REG> {
        self.variant(CKDFSDM2ASEL::I2s2)
    }
}
///Field `CKDFSDM1ASEL` reader - DFSDM1 audio clock selection.
pub use CKDFSDM2ASEL_R as CKDFSDM1ASEL_R;
///Field `CKDFSDM1ASEL` writer - DFSDM1 audio clock selection.
pub use CKDFSDM2ASEL_W as CKDFSDM1ASEL_W;
/**SAI1-A clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1ASRC {
    ///0: SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    Pllsai = 0,
    ///1: SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    Plli2s = 1,
    ///2: SAI1-A clock frequency = Alternate function input frequency
    I2sCkin = 2,
}
impl From<SAI1ASRC> for u8 {
    #[inline(always)]
    fn from(variant: SAI1ASRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI1ASRC {
    type Ux = u8;
}
impl crate::IsEnum for SAI1ASRC {}
///Field `SAI1ASRC` reader - SAI1-A clock source selection
pub type SAI1ASRC_R = crate::FieldReader<SAI1ASRC>;
impl SAI1ASRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAI1ASRC> {
        match self.bits {
            0 => Some(SAI1ASRC::Pllsai),
            1 => Some(SAI1ASRC::Plli2s),
            2 => Some(SAI1ASRC::I2sCkin),
            _ => None,
        }
    }
    ///SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1ASRC::Pllsai
    }
    ///SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1ASRC::Plli2s
    }
    ///SAI1-A clock frequency = Alternate function input frequency
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1ASRC::I2sCkin
    }
}
///Field `SAI1ASRC` writer - SAI1-A clock source selection
pub type SAI1ASRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SAI1ASRC>;
impl<'a, REG> SAI1ASRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1ASRC::Pllsai)
    }
    ///SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1ASRC::Plli2s)
    }
    ///SAI1-A clock frequency = Alternate function input frequency
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1ASRC::I2sCkin)
    }
}
/**SAI1-B clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1BSRC {
    ///0: SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    Pllsai = 0,
    ///1: SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    Plli2s = 1,
    ///2: SAI1-B clock frequency = Alternate function input frequency
    I2sCkin = 2,
}
impl From<SAI1BSRC> for u8 {
    #[inline(always)]
    fn from(variant: SAI1BSRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI1BSRC {
    type Ux = u8;
}
impl crate::IsEnum for SAI1BSRC {}
///Field `SAI1BSRC` reader - SAI1-B clock source selection
pub type SAI1BSRC_R = crate::FieldReader<SAI1BSRC>;
impl SAI1BSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAI1BSRC> {
        match self.bits {
            0 => Some(SAI1BSRC::Pllsai),
            1 => Some(SAI1BSRC::Plli2s),
            2 => Some(SAI1BSRC::I2sCkin),
            _ => None,
        }
    }
    ///SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1BSRC::Pllsai
    }
    ///SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1BSRC::Plli2s
    }
    ///SAI1-B clock frequency = Alternate function input frequency
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1BSRC::I2sCkin
    }
}
///Field `SAI1BSRC` writer - SAI1-B clock source selection
pub type SAI1BSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SAI1BSRC>;
impl<'a, REG> SAI1BSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1BSRC::Pllsai)
    }
    ///SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1BSRC::Plli2s)
    }
    ///SAI1-B clock frequency = Alternate function input frequency
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1BSRC::I2sCkin)
    }
}
/**Timers clocks prescalers selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE {
    ///0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    Mul1or2 = 0,
    ///1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    Mul1or4 = 1,
}
impl From<TIMPRE> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMPRE` reader - Timers clocks prescalers selection
pub type TIMPRE_R = crate::BitReader<TIMPRE>;
impl TIMPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMPRE {
        match self.bits {
            false => TIMPRE::Mul1or2,
            true => TIMPRE::Mul1or4,
        }
    }
    ///If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    #[inline(always)]
    pub fn is_mul1or2(&self) -> bool {
        *self == TIMPRE::Mul1or2
    }
    ///If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    #[inline(always)]
    pub fn is_mul1or4(&self) -> bool {
        *self == TIMPRE::Mul1or4
    }
}
///Field `TIMPRE` writer - Timers clocks prescalers selection
pub type TIMPRE_W<'a, REG> = crate::BitWriter<'a, REG, TIMPRE>;
impl<'a, REG> TIMPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    #[inline(always)]
    pub fn mul1or2(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::Mul1or2)
    }
    ///If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    #[inline(always)]
    pub fn mul1or4(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::Mul1or4)
    }
}
/**I2S APB1 clocks source selection (I2S2/3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2S1SRC {
    ///0: I2Sx clock frequency = f(PLLI2S_R)
    Plli2sr = 0,
    ///1: I2Sx clock frequency = I2S_CKIN Alternate function input frequency
    I2sCkin = 1,
    ///2: I2Sx clock frequency = f(PLL_R)
    Pllr = 2,
    ///3: I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\[22\])
    HsiHse = 3,
}
impl From<I2S1SRC> for u8 {
    #[inline(always)]
    fn from(variant: I2S1SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2S1SRC {
    type Ux = u8;
}
impl crate::IsEnum for I2S1SRC {}
///Field `I2S1SRC` reader - I2S APB1 clocks source selection (I2S2/3)
pub type I2S1SRC_R = crate::FieldReader<I2S1SRC>;
impl I2S1SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2S1SRC {
        match self.bits {
            0 => I2S1SRC::Plli2sr,
            1 => I2S1SRC::I2sCkin,
            2 => I2S1SRC::Pllr,
            3 => I2S1SRC::HsiHse,
            _ => unreachable!(),
        }
    }
    ///I2Sx clock frequency = f(PLLI2S_R)
    #[inline(always)]
    pub fn is_plli2sr(&self) -> bool {
        *self == I2S1SRC::Plli2sr
    }
    ///I2Sx clock frequency = I2S_CKIN Alternate function input frequency
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == I2S1SRC::I2sCkin
    }
    ///I2Sx clock frequency = f(PLL_R)
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == I2S1SRC::Pllr
    }
    ///I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\[22\])
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == I2S1SRC::HsiHse
    }
}
///Field `I2S1SRC` writer - I2S APB1 clocks source selection (I2S2/3)
pub type I2S1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2S1SRC, crate::Safe>;
impl<'a, REG> I2S1SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///I2Sx clock frequency = f(PLLI2S_R)
    #[inline(always)]
    pub fn plli2sr(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SRC::Plli2sr)
    }
    ///I2Sx clock frequency = I2S_CKIN Alternate function input frequency
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SRC::I2sCkin)
    }
    ///I2Sx clock frequency = f(PLL_R)
    #[inline(always)]
    pub fn pllr(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SRC::Pllr)
    }
    ///I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\[22\])
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SRC::HsiHse)
    }
}
///Field `I2S2SRC` reader - I2S APB2 clocks source selection (I2S1/4/5)
pub use I2S1SRC_R as I2S2SRC_R;
///Field `I2S2SRC` writer - I2S APB2 clocks source selection (I2S1/4/5)
pub use I2S1SRC_W as I2S2SRC_W;
/**DFSDM1 Kernel clock selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKDFSDM1SEL {
    ///0: APB2 clock used as Kernel clock
    Apb2 = 0,
    ///1: System clock used as Kernel clock
    Sysclk = 1,
}
impl From<CKDFSDM1SEL> for bool {
    #[inline(always)]
    fn from(variant: CKDFSDM1SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CKDFSDM1SEL` reader - DFSDM1 Kernel clock selection
pub type CKDFSDM1SEL_R = crate::BitReader<CKDFSDM1SEL>;
impl CKDFSDM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKDFSDM1SEL {
        match self.bits {
            false => CKDFSDM1SEL::Apb2,
            true => CKDFSDM1SEL::Sysclk,
        }
    }
    ///APB2 clock used as Kernel clock
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == CKDFSDM1SEL::Apb2
    }
    ///System clock used as Kernel clock
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKDFSDM1SEL::Sysclk
    }
}
///Field `CKDFSDM1SEL` writer - DFSDM1 Kernel clock selection
pub type CKDFSDM1SEL_W<'a, REG> = crate::BitWriter<'a, REG, CKDFSDM1SEL>;
impl<'a, REG> CKDFSDM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///APB2 clock used as Kernel clock
    #[inline(always)]
    pub fn apb2(self) -> &'a mut crate::W<REG> {
        self.variant(CKDFSDM1SEL::Apb2)
    }
    ///System clock used as Kernel clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKDFSDM1SEL::Sysclk)
    }
}
impl R {
    ///Bits 0:4 - PLLI2S division factor for SAI1 A/B clock
    #[inline(always)]
    pub fn plli2sdivr(&self) -> PLLI2SDIVR_R {
        PLLI2SDIVR_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - PLL division factor for SAI1 A/B clock
    #[inline(always)]
    pub fn plldivr(&self) -> PLLDIVR_R {
        PLLDIVR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 14 - DFSDM2 audio clock selection
    #[inline(always)]
    pub fn ckdfsdm2asel(&self) -> CKDFSDM2ASEL_R {
        CKDFSDM2ASEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DFSDM1 audio clock selection.
    #[inline(always)]
    pub fn ckdfsdm1asel(&self) -> CKDFSDM1ASEL_R {
        CKDFSDM1ASEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 20:21 - SAI1-A clock source selection
    #[inline(always)]
    pub fn sai1asrc(&self) -> SAI1ASRC_R {
        SAI1ASRC_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - SAI1-B clock source selection
    #[inline(always)]
    pub fn sai1bsrc(&self) -> SAI1BSRC_R {
        SAI1BSRC_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)
    #[inline(always)]
    pub fn i2s1src(&self) -> I2S1SRC_R {
        I2S1SRC_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bit 31 - DFSDM1 Kernel clock selection
    #[inline(always)]
    pub fn ckdfsdm1sel(&self) -> CKDFSDM1SEL_R {
        CKDFSDM1SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCKCFGR")
            .field("ckdfsdm2asel", &self.ckdfsdm2asel())
            .field("ckdfsdm1asel", &self.ckdfsdm1asel())
            .field("timpre", &self.timpre())
            .field("i2s1src", &self.i2s1src())
            .field("i2s2src", &self.i2s2src())
            .field("plli2sdivr", &self.plli2sdivr())
            .field("plldivr", &self.plldivr())
            .field("sai1asrc", &self.sai1asrc())
            .field("sai1bsrc", &self.sai1bsrc())
            .field("ckdfsdm1sel", &self.ckdfsdm1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - PLLI2S division factor for SAI1 A/B clock
    #[inline(always)]
    pub fn plli2sdivr(&mut self) -> PLLI2SDIVR_W<'_, DCKCFGRrs> {
        PLLI2SDIVR_W::new(self, 0)
    }
    ///Bits 8:12 - PLL division factor for SAI1 A/B clock
    #[inline(always)]
    pub fn plldivr(&mut self) -> PLLDIVR_W<'_, DCKCFGRrs> {
        PLLDIVR_W::new(self, 8)
    }
    ///Bit 14 - DFSDM2 audio clock selection
    #[inline(always)]
    pub fn ckdfsdm2asel(&mut self) -> CKDFSDM2ASEL_W<'_, DCKCFGRrs> {
        CKDFSDM2ASEL_W::new(self, 14)
    }
    ///Bit 15 - DFSDM1 audio clock selection.
    #[inline(always)]
    pub fn ckdfsdm1asel(&mut self) -> CKDFSDM1ASEL_W<'_, DCKCFGRrs> {
        CKDFSDM1ASEL_W::new(self, 15)
    }
    ///Bits 20:21 - SAI1-A clock source selection
    #[inline(always)]
    pub fn sai1asrc(&mut self) -> SAI1ASRC_W<'_, DCKCFGRrs> {
        SAI1ASRC_W::new(self, 20)
    }
    ///Bits 22:23 - SAI1-B clock source selection
    #[inline(always)]
    pub fn sai1bsrc(&mut self) -> SAI1BSRC_W<'_, DCKCFGRrs> {
        SAI1BSRC_W::new(self, 22)
    }
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W<'_, DCKCFGRrs> {
        TIMPRE_W::new(self, 24)
    }
    ///Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)
    #[inline(always)]
    pub fn i2s1src(&mut self) -> I2S1SRC_W<'_, DCKCFGRrs> {
        I2S1SRC_W::new(self, 25)
    }
    ///Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)
    #[inline(always)]
    pub fn i2s2src(&mut self) -> I2S2SRC_W<'_, DCKCFGRrs> {
        I2S2SRC_W::new(self, 27)
    }
    ///Bit 31 - DFSDM1 Kernel clock selection
    #[inline(always)]
    pub fn ckdfsdm1sel(&mut self) -> CKDFSDM1SEL_W<'_, DCKCFGRrs> {
        CKDFSDM1SEL_W::new(self, 31)
    }
}
/**Dedicated Clocks Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dckcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dckcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#RCC:DCKCFGR)*/
pub struct DCKCFGRrs;
impl crate::RegisterSpec for DCKCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`dckcfgr::R`](R) reader structure
impl crate::Readable for DCKCFGRrs {}
///`write(|w| ..)` method takes [`dckcfgr::W`](W) writer structure
impl crate::Writable for DCKCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCKCFGR to value 0
impl crate::Resettable for DCKCFGRrs {}
