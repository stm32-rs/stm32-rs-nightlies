#[doc = "Register `DCKCFGR1` reader"]
pub type R = crate::R<DCKCFGR1rs>;
#[doc = "Register `DCKCFGR1` writer"]
pub type W = crate::W<DCKCFGR1rs>;
#[doc = "PLLI2S division factor for SAI1 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLI2SDIVQ {
    #[doc = "0: PLLI2SDIVQ = /1"]
    Div1 = 0,
    #[doc = "1: PLLI2SDIVQ = /2"]
    Div2 = 1,
    #[doc = "2: PLLI2SDIVQ = /3"]
    Div3 = 2,
    #[doc = "3: PLLI2SDIVQ = /4"]
    Div4 = 3,
    #[doc = "4: PLLI2SDIVQ = /5"]
    Div5 = 4,
    #[doc = "5: PLLI2SDIVQ = /6"]
    Div6 = 5,
    #[doc = "6: PLLI2SDIVQ = /7"]
    Div7 = 6,
    #[doc = "7: PLLI2SDIVQ = /8"]
    Div8 = 7,
    #[doc = "8: PLLI2SDIVQ = /9"]
    Div9 = 8,
    #[doc = "9: PLLI2SDIVQ = /10"]
    Div10 = 9,
    #[doc = "10: PLLI2SDIVQ = /11"]
    Div11 = 10,
    #[doc = "11: PLLI2SDIVQ = /12"]
    Div12 = 11,
    #[doc = "12: PLLI2SDIVQ = /13"]
    Div13 = 12,
    #[doc = "13: PLLI2SDIVQ = /14"]
    Div14 = 13,
    #[doc = "14: PLLI2SDIVQ = /15"]
    Div15 = 14,
    #[doc = "15: PLLI2SDIVQ = /16"]
    Div16 = 15,
    #[doc = "16: PLLI2SDIVQ = /17"]
    Div17 = 16,
    #[doc = "17: PLLI2SDIVQ = /18"]
    Div18 = 17,
    #[doc = "18: PLLI2SDIVQ = /19"]
    Div19 = 18,
    #[doc = "19: PLLI2SDIVQ = /20"]
    Div20 = 19,
    #[doc = "20: PLLI2SDIVQ = /21"]
    Div21 = 20,
    #[doc = "21: PLLI2SDIVQ = /22"]
    Div22 = 21,
    #[doc = "22: PLLI2SDIVQ = /23"]
    Div23 = 22,
    #[doc = "23: PLLI2SDIVQ = /24"]
    Div24 = 23,
    #[doc = "24: PLLI2SDIVQ = /25"]
    Div25 = 24,
    #[doc = "25: PLLI2SDIVQ = /26"]
    Div26 = 25,
    #[doc = "26: PLLI2SDIVQ = /27"]
    Div27 = 26,
    #[doc = "27: PLLI2SDIVQ = /28"]
    Div28 = 27,
    #[doc = "28: PLLI2SDIVQ = /29"]
    Div29 = 28,
    #[doc = "29: PLLI2SDIVQ = /30"]
    Div30 = 29,
    #[doc = "30: PLLI2SDIVQ = /31"]
    Div31 = 30,
    #[doc = "31: PLLI2SDIVQ = /32"]
    Div32 = 31,
}
impl From<PLLI2SDIVQ> for u8 {
    #[inline(always)]
    fn from(variant: PLLI2SDIVQ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLI2SDIVQ {
    type Ux = u8;
}
#[doc = "Field `PLLI2SDIVQ` reader - PLLI2S division factor for SAI1 clock"]
pub type PLLI2SDIVQ_R = crate::FieldReader<PLLI2SDIVQ>;
impl PLLI2SDIVQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLI2SDIVQ {
        match self.bits {
            0 => PLLI2SDIVQ::Div1,
            1 => PLLI2SDIVQ::Div2,
            2 => PLLI2SDIVQ::Div3,
            3 => PLLI2SDIVQ::Div4,
            4 => PLLI2SDIVQ::Div5,
            5 => PLLI2SDIVQ::Div6,
            6 => PLLI2SDIVQ::Div7,
            7 => PLLI2SDIVQ::Div8,
            8 => PLLI2SDIVQ::Div9,
            9 => PLLI2SDIVQ::Div10,
            10 => PLLI2SDIVQ::Div11,
            11 => PLLI2SDIVQ::Div12,
            12 => PLLI2SDIVQ::Div13,
            13 => PLLI2SDIVQ::Div14,
            14 => PLLI2SDIVQ::Div15,
            15 => PLLI2SDIVQ::Div16,
            16 => PLLI2SDIVQ::Div17,
            17 => PLLI2SDIVQ::Div18,
            18 => PLLI2SDIVQ::Div19,
            19 => PLLI2SDIVQ::Div20,
            20 => PLLI2SDIVQ::Div21,
            21 => PLLI2SDIVQ::Div22,
            22 => PLLI2SDIVQ::Div23,
            23 => PLLI2SDIVQ::Div24,
            24 => PLLI2SDIVQ::Div25,
            25 => PLLI2SDIVQ::Div26,
            26 => PLLI2SDIVQ::Div27,
            27 => PLLI2SDIVQ::Div28,
            28 => PLLI2SDIVQ::Div29,
            29 => PLLI2SDIVQ::Div30,
            30 => PLLI2SDIVQ::Div31,
            31 => PLLI2SDIVQ::Div32,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLI2SDIVQ = /1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLI2SDIVQ::Div1
    }
    #[doc = "PLLI2SDIVQ = /2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLI2SDIVQ::Div2
    }
    #[doc = "PLLI2SDIVQ = /3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLI2SDIVQ::Div3
    }
    #[doc = "PLLI2SDIVQ = /4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLI2SDIVQ::Div4
    }
    #[doc = "PLLI2SDIVQ = /5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLI2SDIVQ::Div5
    }
    #[doc = "PLLI2SDIVQ = /6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLI2SDIVQ::Div6
    }
    #[doc = "PLLI2SDIVQ = /7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLI2SDIVQ::Div7
    }
    #[doc = "PLLI2SDIVQ = /8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLI2SDIVQ::Div8
    }
    #[doc = "PLLI2SDIVQ = /9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLI2SDIVQ::Div9
    }
    #[doc = "PLLI2SDIVQ = /10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLI2SDIVQ::Div10
    }
    #[doc = "PLLI2SDIVQ = /11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLI2SDIVQ::Div11
    }
    #[doc = "PLLI2SDIVQ = /12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLI2SDIVQ::Div12
    }
    #[doc = "PLLI2SDIVQ = /13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLI2SDIVQ::Div13
    }
    #[doc = "PLLI2SDIVQ = /14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLI2SDIVQ::Div14
    }
    #[doc = "PLLI2SDIVQ = /15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLI2SDIVQ::Div15
    }
    #[doc = "PLLI2SDIVQ = /16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLI2SDIVQ::Div16
    }
    #[doc = "PLLI2SDIVQ = /17"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLI2SDIVQ::Div17
    }
    #[doc = "PLLI2SDIVQ = /18"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLI2SDIVQ::Div18
    }
    #[doc = "PLLI2SDIVQ = /19"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLI2SDIVQ::Div19
    }
    #[doc = "PLLI2SDIVQ = /20"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLI2SDIVQ::Div20
    }
    #[doc = "PLLI2SDIVQ = /21"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLI2SDIVQ::Div21
    }
    #[doc = "PLLI2SDIVQ = /22"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLI2SDIVQ::Div22
    }
    #[doc = "PLLI2SDIVQ = /23"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLI2SDIVQ::Div23
    }
    #[doc = "PLLI2SDIVQ = /24"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLI2SDIVQ::Div24
    }
    #[doc = "PLLI2SDIVQ = /25"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLI2SDIVQ::Div25
    }
    #[doc = "PLLI2SDIVQ = /26"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLI2SDIVQ::Div26
    }
    #[doc = "PLLI2SDIVQ = /27"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLI2SDIVQ::Div27
    }
    #[doc = "PLLI2SDIVQ = /28"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLI2SDIVQ::Div28
    }
    #[doc = "PLLI2SDIVQ = /29"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLI2SDIVQ::Div29
    }
    #[doc = "PLLI2SDIVQ = /30"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLI2SDIVQ::Div30
    }
    #[doc = "PLLI2SDIVQ = /31"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLI2SDIVQ::Div31
    }
    #[doc = "PLLI2SDIVQ = /32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLI2SDIVQ::Div32
    }
}
#[doc = "Field `PLLI2SDIVQ` writer - PLLI2S division factor for SAI1 clock"]
pub type PLLI2SDIVQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, PLLI2SDIVQ>;
impl<'a, REG> PLLI2SDIVQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLI2SDIVQ = /1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div1)
    }
    #[doc = "PLLI2SDIVQ = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div2)
    }
    #[doc = "PLLI2SDIVQ = /3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div3)
    }
    #[doc = "PLLI2SDIVQ = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div4)
    }
    #[doc = "PLLI2SDIVQ = /5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div5)
    }
    #[doc = "PLLI2SDIVQ = /6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div6)
    }
    #[doc = "PLLI2SDIVQ = /7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div7)
    }
    #[doc = "PLLI2SDIVQ = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div8)
    }
    #[doc = "PLLI2SDIVQ = /9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div9)
    }
    #[doc = "PLLI2SDIVQ = /10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div10)
    }
    #[doc = "PLLI2SDIVQ = /11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div11)
    }
    #[doc = "PLLI2SDIVQ = /12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div12)
    }
    #[doc = "PLLI2SDIVQ = /13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div13)
    }
    #[doc = "PLLI2SDIVQ = /14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div14)
    }
    #[doc = "PLLI2SDIVQ = /15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div15)
    }
    #[doc = "PLLI2SDIVQ = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div16)
    }
    #[doc = "PLLI2SDIVQ = /17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div17)
    }
    #[doc = "PLLI2SDIVQ = /18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div18)
    }
    #[doc = "PLLI2SDIVQ = /19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div19)
    }
    #[doc = "PLLI2SDIVQ = /20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div20)
    }
    #[doc = "PLLI2SDIVQ = /21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div21)
    }
    #[doc = "PLLI2SDIVQ = /22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div22)
    }
    #[doc = "PLLI2SDIVQ = /23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div23)
    }
    #[doc = "PLLI2SDIVQ = /24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div24)
    }
    #[doc = "PLLI2SDIVQ = /25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div25)
    }
    #[doc = "PLLI2SDIVQ = /26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div26)
    }
    #[doc = "PLLI2SDIVQ = /27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div27)
    }
    #[doc = "PLLI2SDIVQ = /28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div28)
    }
    #[doc = "PLLI2SDIVQ = /29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div29)
    }
    #[doc = "PLLI2SDIVQ = /30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div30)
    }
    #[doc = "PLLI2SDIVQ = /31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div31)
    }
    #[doc = "PLLI2SDIVQ = /32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SDIVQ::Div32)
    }
}
#[doc = "PLLSAI division factor for SAI1 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAIDIVQ {
    #[doc = "0: PLLSAIDIVQ = /1"]
    Div1 = 0,
    #[doc = "1: PLLSAIDIVQ = /2"]
    Div2 = 1,
    #[doc = "2: PLLSAIDIVQ = /3"]
    Div3 = 2,
    #[doc = "3: PLLSAIDIVQ = /4"]
    Div4 = 3,
    #[doc = "4: PLLSAIDIVQ = /5"]
    Div5 = 4,
    #[doc = "5: PLLSAIDIVQ = /6"]
    Div6 = 5,
    #[doc = "6: PLLSAIDIVQ = /7"]
    Div7 = 6,
    #[doc = "7: PLLSAIDIVQ = /8"]
    Div8 = 7,
    #[doc = "8: PLLSAIDIVQ = /9"]
    Div9 = 8,
    #[doc = "9: PLLSAIDIVQ = /10"]
    Div10 = 9,
    #[doc = "10: PLLSAIDIVQ = /11"]
    Div11 = 10,
    #[doc = "11: PLLSAIDIVQ = /12"]
    Div12 = 11,
    #[doc = "12: PLLSAIDIVQ = /13"]
    Div13 = 12,
    #[doc = "13: PLLSAIDIVQ = /14"]
    Div14 = 13,
    #[doc = "14: PLLSAIDIVQ = /15"]
    Div15 = 14,
    #[doc = "15: PLLSAIDIVQ = /16"]
    Div16 = 15,
    #[doc = "16: PLLSAIDIVQ = /17"]
    Div17 = 16,
    #[doc = "17: PLLSAIDIVQ = /18"]
    Div18 = 17,
    #[doc = "18: PLLSAIDIVQ = /19"]
    Div19 = 18,
    #[doc = "19: PLLSAIDIVQ = /20"]
    Div20 = 19,
    #[doc = "20: PLLSAIDIVQ = /21"]
    Div21 = 20,
    #[doc = "21: PLLSAIDIVQ = /22"]
    Div22 = 21,
    #[doc = "22: PLLSAIDIVQ = /23"]
    Div23 = 22,
    #[doc = "23: PLLSAIDIVQ = /24"]
    Div24 = 23,
    #[doc = "24: PLLSAIDIVQ = /25"]
    Div25 = 24,
    #[doc = "25: PLLSAIDIVQ = /26"]
    Div26 = 25,
    #[doc = "26: PLLSAIDIVQ = /27"]
    Div27 = 26,
    #[doc = "27: PLLSAIDIVQ = /28"]
    Div28 = 27,
    #[doc = "28: PLLSAIDIVQ = /29"]
    Div29 = 28,
    #[doc = "29: PLLSAIDIVQ = /30"]
    Div30 = 29,
    #[doc = "30: PLLSAIDIVQ = /31"]
    Div31 = 30,
    #[doc = "31: PLLSAIDIVQ = /32"]
    Div32 = 31,
}
impl From<PLLSAIDIVQ> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVQ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAIDIVQ {
    type Ux = u8;
}
#[doc = "Field `PLLSAIDIVQ` reader - PLLSAI division factor for SAI1 clock"]
pub type PLLSAIDIVQ_R = crate::FieldReader<PLLSAIDIVQ>;
impl PLLSAIDIVQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAIDIVQ {
        match self.bits {
            0 => PLLSAIDIVQ::Div1,
            1 => PLLSAIDIVQ::Div2,
            2 => PLLSAIDIVQ::Div3,
            3 => PLLSAIDIVQ::Div4,
            4 => PLLSAIDIVQ::Div5,
            5 => PLLSAIDIVQ::Div6,
            6 => PLLSAIDIVQ::Div7,
            7 => PLLSAIDIVQ::Div8,
            8 => PLLSAIDIVQ::Div9,
            9 => PLLSAIDIVQ::Div10,
            10 => PLLSAIDIVQ::Div11,
            11 => PLLSAIDIVQ::Div12,
            12 => PLLSAIDIVQ::Div13,
            13 => PLLSAIDIVQ::Div14,
            14 => PLLSAIDIVQ::Div15,
            15 => PLLSAIDIVQ::Div16,
            16 => PLLSAIDIVQ::Div17,
            17 => PLLSAIDIVQ::Div18,
            18 => PLLSAIDIVQ::Div19,
            19 => PLLSAIDIVQ::Div20,
            20 => PLLSAIDIVQ::Div21,
            21 => PLLSAIDIVQ::Div22,
            22 => PLLSAIDIVQ::Div23,
            23 => PLLSAIDIVQ::Div24,
            24 => PLLSAIDIVQ::Div25,
            25 => PLLSAIDIVQ::Div26,
            26 => PLLSAIDIVQ::Div27,
            27 => PLLSAIDIVQ::Div28,
            28 => PLLSAIDIVQ::Div29,
            29 => PLLSAIDIVQ::Div30,
            30 => PLLSAIDIVQ::Div31,
            31 => PLLSAIDIVQ::Div32,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLSAIDIVQ = /1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLSAIDIVQ::Div1
    }
    #[doc = "PLLSAIDIVQ = /2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVQ::Div2
    }
    #[doc = "PLLSAIDIVQ = /3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAIDIVQ::Div3
    }
    #[doc = "PLLSAIDIVQ = /4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVQ::Div4
    }
    #[doc = "PLLSAIDIVQ = /5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAIDIVQ::Div5
    }
    #[doc = "PLLSAIDIVQ = /6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAIDIVQ::Div6
    }
    #[doc = "PLLSAIDIVQ = /7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAIDIVQ::Div7
    }
    #[doc = "PLLSAIDIVQ = /8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVQ::Div8
    }
    #[doc = "PLLSAIDIVQ = /9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAIDIVQ::Div9
    }
    #[doc = "PLLSAIDIVQ = /10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAIDIVQ::Div10
    }
    #[doc = "PLLSAIDIVQ = /11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAIDIVQ::Div11
    }
    #[doc = "PLLSAIDIVQ = /12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAIDIVQ::Div12
    }
    #[doc = "PLLSAIDIVQ = /13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAIDIVQ::Div13
    }
    #[doc = "PLLSAIDIVQ = /14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAIDIVQ::Div14
    }
    #[doc = "PLLSAIDIVQ = /15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAIDIVQ::Div15
    }
    #[doc = "PLLSAIDIVQ = /16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVQ::Div16
    }
    #[doc = "PLLSAIDIVQ = /17"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAIDIVQ::Div17
    }
    #[doc = "PLLSAIDIVQ = /18"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLSAIDIVQ::Div18
    }
    #[doc = "PLLSAIDIVQ = /19"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLSAIDIVQ::Div19
    }
    #[doc = "PLLSAIDIVQ = /20"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLSAIDIVQ::Div20
    }
    #[doc = "PLLSAIDIVQ = /21"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLSAIDIVQ::Div21
    }
    #[doc = "PLLSAIDIVQ = /22"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLSAIDIVQ::Div22
    }
    #[doc = "PLLSAIDIVQ = /23"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLSAIDIVQ::Div23
    }
    #[doc = "PLLSAIDIVQ = /24"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLSAIDIVQ::Div24
    }
    #[doc = "PLLSAIDIVQ = /25"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLSAIDIVQ::Div25
    }
    #[doc = "PLLSAIDIVQ = /26"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLSAIDIVQ::Div26
    }
    #[doc = "PLLSAIDIVQ = /27"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLSAIDIVQ::Div27
    }
    #[doc = "PLLSAIDIVQ = /28"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLSAIDIVQ::Div28
    }
    #[doc = "PLLSAIDIVQ = /29"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLSAIDIVQ::Div29
    }
    #[doc = "PLLSAIDIVQ = /30"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLSAIDIVQ::Div30
    }
    #[doc = "PLLSAIDIVQ = /31"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLSAIDIVQ::Div31
    }
    #[doc = "PLLSAIDIVQ = /32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLSAIDIVQ::Div32
    }
}
#[doc = "Field `PLLSAIDIVQ` writer - PLLSAI division factor for SAI1 clock"]
pub type PLLSAIDIVQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, PLLSAIDIVQ>;
impl<'a, REG> PLLSAIDIVQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLSAIDIVQ = /1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div1)
    }
    #[doc = "PLLSAIDIVQ = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div2)
    }
    #[doc = "PLLSAIDIVQ = /3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div3)
    }
    #[doc = "PLLSAIDIVQ = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div4)
    }
    #[doc = "PLLSAIDIVQ = /5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div5)
    }
    #[doc = "PLLSAIDIVQ = /6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div6)
    }
    #[doc = "PLLSAIDIVQ = /7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div7)
    }
    #[doc = "PLLSAIDIVQ = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div8)
    }
    #[doc = "PLLSAIDIVQ = /9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div9)
    }
    #[doc = "PLLSAIDIVQ = /10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div10)
    }
    #[doc = "PLLSAIDIVQ = /11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div11)
    }
    #[doc = "PLLSAIDIVQ = /12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div12)
    }
    #[doc = "PLLSAIDIVQ = /13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div13)
    }
    #[doc = "PLLSAIDIVQ = /14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div14)
    }
    #[doc = "PLLSAIDIVQ = /15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div15)
    }
    #[doc = "PLLSAIDIVQ = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div16)
    }
    #[doc = "PLLSAIDIVQ = /17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div17)
    }
    #[doc = "PLLSAIDIVQ = /18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div18)
    }
    #[doc = "PLLSAIDIVQ = /19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div19)
    }
    #[doc = "PLLSAIDIVQ = /20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div20)
    }
    #[doc = "PLLSAIDIVQ = /21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div21)
    }
    #[doc = "PLLSAIDIVQ = /22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div22)
    }
    #[doc = "PLLSAIDIVQ = /23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div23)
    }
    #[doc = "PLLSAIDIVQ = /24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div24)
    }
    #[doc = "PLLSAIDIVQ = /25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div25)
    }
    #[doc = "PLLSAIDIVQ = /26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div26)
    }
    #[doc = "PLLSAIDIVQ = /27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div27)
    }
    #[doc = "PLLSAIDIVQ = /28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div28)
    }
    #[doc = "PLLSAIDIVQ = /29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div29)
    }
    #[doc = "PLLSAIDIVQ = /30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div30)
    }
    #[doc = "PLLSAIDIVQ = /31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div31)
    }
    #[doc = "PLLSAIDIVQ = /32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVQ::Div32)
    }
}
#[doc = "division factor for LCD_CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAIDIVR {
    #[doc = "0: PLLSAIDIVR = /2"]
    Div2 = 0,
    #[doc = "1: PLLSAIDIVR = /4"]
    Div4 = 1,
    #[doc = "2: PLLSAIDIVR = /8"]
    Div8 = 2,
    #[doc = "3: PLLSAIDIVR = /16"]
    Div16 = 3,
}
impl From<PLLSAIDIVR> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAIDIVR {
    type Ux = u8;
}
#[doc = "Field `PLLSAIDIVR` reader - division factor for LCD_CLK"]
pub type PLLSAIDIVR_R = crate::FieldReader<PLLSAIDIVR>;
impl PLLSAIDIVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAIDIVR {
        match self.bits {
            0 => PLLSAIDIVR::Div2,
            1 => PLLSAIDIVR::Div4,
            2 => PLLSAIDIVR::Div8,
            3 => PLLSAIDIVR::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLSAIDIVR = /2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVR::Div2
    }
    #[doc = "PLLSAIDIVR = /4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVR::Div4
    }
    #[doc = "PLLSAIDIVR = /8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVR::Div8
    }
    #[doc = "PLLSAIDIVR = /16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVR::Div16
    }
}
#[doc = "Field `PLLSAIDIVR` writer - division factor for LCD_CLK"]
pub type PLLSAIDIVR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLLSAIDIVR>;
impl<'a, REG> PLLSAIDIVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLSAIDIVR = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVR::Div2)
    }
    #[doc = "PLLSAIDIVR = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVR::Div4)
    }
    #[doc = "PLLSAIDIVR = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVR::Div8)
    }
    #[doc = "PLLSAIDIVR = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIDIVR::Div16)
    }
}
#[doc = "SAI1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL {
    #[doc = "0: SAI1 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    Pllsai = 0,
    #[doc = "1: SAI1 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    Plli2s = 1,
    #[doc = "2: SAI1 clock frequency = Alternate function input frequency"]
    Afif = 2,
    #[doc = "3: SAI1 clock frequency = HSI or HSE"]
    HsiHse = 3,
}
impl From<SAI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI1SEL {
    type Ux = u8;
}
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub type SAI1SEL_R = crate::FieldReader<SAI1SEL>;
impl SAI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAI1SEL {
        match self.bits {
            0 => SAI1SEL::Pllsai,
            1 => SAI1SEL::Plli2s,
            2 => SAI1SEL::Afif,
            3 => SAI1SEL::HsiHse,
            _ => unreachable!(),
        }
    }
    #[doc = "SAI1 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1SEL::Pllsai
    }
    #[doc = "SAI1 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1SEL::Plli2s
    }
    #[doc = "SAI1 clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn is_afif(&self) -> bool {
        *self == SAI1SEL::Afif
    }
    #[doc = "SAI1 clock frequency = HSI or HSE"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == SAI1SEL::HsiHse
    }
}
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub type SAI1SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SAI1SEL>;
impl<'a, REG> SAI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SAI1 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pllsai)
    }
    #[doc = "SAI1 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Plli2s)
    }
    #[doc = "SAI1 clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn afif(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Afif)
    }
    #[doc = "SAI1 clock frequency = HSI or HSE"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::HsiHse)
    }
}
#[doc = "SAI2 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI2SEL {
    #[doc = "0: SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    Pllsai = 0,
    #[doc = "1: SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    Plli2s = 1,
    #[doc = "2: SAI2 clock frequency = Alternate function input frequency"]
    Afif = 2,
    #[doc = "3: SAI2 clock frequency = HSI or HSE"]
    HsiHse = 3,
}
impl From<SAI2SEL> for u8 {
    #[inline(always)]
    fn from(variant: SAI2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI2SEL {
    type Ux = u8;
}
#[doc = "Field `SAI2SEL` reader - SAI2 clock source selection"]
pub type SAI2SEL_R = crate::FieldReader<SAI2SEL>;
impl SAI2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAI2SEL {
        match self.bits {
            0 => SAI2SEL::Pllsai,
            1 => SAI2SEL::Plli2s,
            2 => SAI2SEL::Afif,
            3 => SAI2SEL::HsiHse,
            _ => unreachable!(),
        }
    }
    #[doc = "SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI2SEL::Pllsai
    }
    #[doc = "SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI2SEL::Plli2s
    }
    #[doc = "SAI2 clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn is_afif(&self) -> bool {
        *self == SAI2SEL::Afif
    }
    #[doc = "SAI2 clock frequency = HSI or HSE"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == SAI2SEL::HsiHse
    }
}
#[doc = "Field `SAI2SEL` writer - SAI2 clock source selection"]
pub type SAI2SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SAI2SEL>;
impl<'a, REG> SAI2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::Pllsai)
    }
    #[doc = "SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::Plli2s)
    }
    #[doc = "SAI2 clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn afif(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::Afif)
    }
    #[doc = "SAI2 clock frequency = HSI or HSE"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::HsiHse)
    }
}
#[doc = "Timers clocks prescalers selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE {
    #[doc = "0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    Mul1or2 = 0,
    #[doc = "1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    Mul1or4 = 1,
}
impl From<TIMPRE> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMPRE` reader - Timers clocks prescalers selection"]
pub type TIMPRE_R = crate::BitReader<TIMPRE>;
impl TIMPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMPRE {
        match self.bits {
            false => TIMPRE::Mul1or2,
            true => TIMPRE::Mul1or4,
        }
    }
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    #[inline(always)]
    pub fn is_mul1or2(&self) -> bool {
        *self == TIMPRE::Mul1or2
    }
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    #[inline(always)]
    pub fn is_mul1or4(&self) -> bool {
        *self == TIMPRE::Mul1or4
    }
}
#[doc = "Field `TIMPRE` writer - Timers clocks prescalers selection"]
pub type TIMPRE_W<'a, REG> = crate::BitWriter<'a, REG, TIMPRE>;
impl<'a, REG> TIMPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    #[inline(always)]
    pub fn mul1or2(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::Mul1or2)
    }
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    #[inline(always)]
    pub fn mul1or4(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::Mul1or4)
    }
}
#[doc = "DFSDM1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1SEL {
    #[doc = "0: APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source"]
    Apb2 = 0,
    #[doc = "1: System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source"]
    Sysclk = 1,
}
impl From<DFSDM1SEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFSDM1SEL` reader - DFSDM1 clock source selection"]
pub type DFSDM1SEL_R = crate::BitReader<DFSDM1SEL>;
impl DFSDM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1SEL {
        match self.bits {
            false => DFSDM1SEL::Apb2,
            true => DFSDM1SEL::Sysclk,
        }
    }
    #[doc = "APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == DFSDM1SEL::Apb2
    }
    #[doc = "System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == DFSDM1SEL::Sysclk
    }
}
#[doc = "Field `DFSDM1SEL` writer - DFSDM1 clock source selection"]
pub type DFSDM1SEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1SEL>;
impl<'a, REG> DFSDM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1SEL::Apb2)
    }
    #[doc = "System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1SEL::Sysclk)
    }
}
#[doc = "DFSDM1 AUDIO clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADFSDM1SEL {
    #[doc = "0: SAI1 clock selected as DFSDM1 Audio clock source"]
    Sai1 = 0,
    #[doc = "1: SAI2 clock selected as DFSDM1 Audio clock source"]
    Sai2 = 1,
}
impl From<ADFSDM1SEL> for bool {
    #[inline(always)]
    fn from(variant: ADFSDM1SEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADFSDM1SEL` reader - DFSDM1 AUDIO clock source selection"]
pub type ADFSDM1SEL_R = crate::BitReader<ADFSDM1SEL>;
impl ADFSDM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADFSDM1SEL {
        match self.bits {
            false => ADFSDM1SEL::Sai1,
            true => ADFSDM1SEL::Sai2,
        }
    }
    #[doc = "SAI1 clock selected as DFSDM1 Audio clock source"]
    #[inline(always)]
    pub fn is_sai1(&self) -> bool {
        *self == ADFSDM1SEL::Sai1
    }
    #[doc = "SAI2 clock selected as DFSDM1 Audio clock source"]
    #[inline(always)]
    pub fn is_sai2(&self) -> bool {
        *self == ADFSDM1SEL::Sai2
    }
}
#[doc = "Field `ADFSDM1SEL` writer - DFSDM1 AUDIO clock source selection"]
pub type ADFSDM1SEL_W<'a, REG> = crate::BitWriter<'a, REG, ADFSDM1SEL>;
impl<'a, REG> ADFSDM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAI1 clock selected as DFSDM1 Audio clock source"]
    #[inline(always)]
    pub fn sai1(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSDM1SEL::Sai1)
    }
    #[doc = "SAI2 clock selected as DFSDM1 Audio clock source"]
    #[inline(always)]
    pub fn sai2(self) -> &'a mut crate::W<REG> {
        self.variant(ADFSDM1SEL::Sai2)
    }
}
impl R {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sdivq(&self) -> PLLI2SDIVQ_R {
        PLLI2SDIVQ_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaidivq(&self) -> PLLSAIDIVQ_R {
        PLLSAIDIVQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    pub fn pllsaidivr(&self) -> PLLSAIDIVR_R {
        PLLSAIDIVR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DFSDM1 clock source selection"]
    #[inline(always)]
    pub fn dfsdm1sel(&self) -> DFSDM1SEL_R {
        DFSDM1SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DFSDM1 AUDIO clock source selection"]
    #[inline(always)]
    pub fn adfsdm1sel(&self) -> ADFSDM1SEL_R {
        ADFSDM1SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sdivq(&mut self) -> PLLI2SDIVQ_W<DCKCFGR1rs> {
        PLLI2SDIVQ_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaidivq(&mut self) -> PLLSAIDIVQ_W<DCKCFGR1rs> {
        PLLSAIDIVQ_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaidivr(&mut self) -> PLLSAIDIVR_W<DCKCFGR1rs> {
        PLLSAIDIVR_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<DCKCFGR1rs> {
        SAI1SEL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<DCKCFGR1rs> {
        SAI2SEL_W::new(self, 22)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<DCKCFGR1rs> {
        TIMPRE_W::new(self, 24)
    }
    #[doc = "Bit 25 - DFSDM1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1sel(&mut self) -> DFSDM1SEL_W<DCKCFGR1rs> {
        DFSDM1SEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - DFSDM1 AUDIO clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adfsdm1sel(&mut self) -> ADFSDM1SEL_W<DCKCFGR1rs> {
        ADFSDM1SEL_W::new(self, 26)
    }
}
#[doc = "dedicated clocks configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dckcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dckcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCKCFGR1rs;
impl crate::RegisterSpec for DCKCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dckcfgr1::R`](R) reader structure"]
impl crate::Readable for DCKCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`dckcfgr1::W`](W) writer structure"]
impl crate::Writable for DCKCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCKCFGR1 to value 0"]
impl crate::Resettable for DCKCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
