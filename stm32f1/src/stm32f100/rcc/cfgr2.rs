#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "PREDIV1 division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREDIV1 {
    #[doc = "0: PREDIV input clock not divided"]
    Div1 = 0,
    #[doc = "1: PREDIV input clock divided by 2"]
    Div2 = 1,
    #[doc = "2: PREDIV input clock divided by 3"]
    Div3 = 2,
    #[doc = "3: PREDIV input clock divided by 4"]
    Div4 = 3,
    #[doc = "4: PREDIV input clock divided by 5"]
    Div5 = 4,
    #[doc = "5: PREDIV input clock divided by 6"]
    Div6 = 5,
    #[doc = "6: PREDIV input clock divided by 7"]
    Div7 = 6,
    #[doc = "7: PREDIV input clock divided by 8"]
    Div8 = 7,
    #[doc = "8: PREDIV input clock divided by 9"]
    Div9 = 8,
    #[doc = "9: PREDIV input clock divided by 10"]
    Div10 = 9,
    #[doc = "10: PREDIV input clock divided by 11"]
    Div11 = 10,
    #[doc = "11: PREDIV input clock divided by 12"]
    Div12 = 11,
    #[doc = "12: PREDIV input clock divided by 13"]
    Div13 = 12,
    #[doc = "13: PREDIV input clock divided by 14"]
    Div14 = 13,
    #[doc = "14: PREDIV input clock divided by 15"]
    Div15 = 14,
    #[doc = "15: PREDIV input clock divided by 16"]
    Div16 = 15,
}
impl From<PREDIV1> for u8 {
    #[inline(always)]
    fn from(variant: PREDIV1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PREDIV1 {
    type Ux = u8;
}
#[doc = "Field `PREDIV1` reader - PREDIV1 division factor"]
pub type PREDIV1_R = crate::FieldReader<PREDIV1>;
impl PREDIV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PREDIV1 {
        match self.bits {
            0 => PREDIV1::Div1,
            1 => PREDIV1::Div2,
            2 => PREDIV1::Div3,
            3 => PREDIV1::Div4,
            4 => PREDIV1::Div5,
            5 => PREDIV1::Div6,
            6 => PREDIV1::Div7,
            7 => PREDIV1::Div8,
            8 => PREDIV1::Div9,
            9 => PREDIV1::Div10,
            10 => PREDIV1::Div11,
            11 => PREDIV1::Div12,
            12 => PREDIV1::Div13,
            13 => PREDIV1::Div14,
            14 => PREDIV1::Div15,
            15 => PREDIV1::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "PREDIV input clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PREDIV1::Div1
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PREDIV1::Div2
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PREDIV1::Div3
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PREDIV1::Div4
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PREDIV1::Div5
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PREDIV1::Div6
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PREDIV1::Div7
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PREDIV1::Div8
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PREDIV1::Div9
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PREDIV1::Div10
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PREDIV1::Div11
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PREDIV1::Div12
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PREDIV1::Div13
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PREDIV1::Div14
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PREDIV1::Div15
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PREDIV1::Div16
    }
}
#[doc = "Field `PREDIV1` writer - PREDIV1 division factor"]
pub type PREDIV1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, PREDIV1>;
impl<'a, REG> PREDIV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PREDIV input clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div1)
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div2)
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div3)
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div4)
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div5)
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div6)
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div7)
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div8)
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div9)
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div10)
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div11)
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div12)
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div13)
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div14)
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div15)
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div16)
    }
}
impl R {
    #[doc = "Bits 0:3 - PREDIV1 division factor"]
    #[inline(always)]
    pub fn prediv1(&self) -> PREDIV1_R {
        PREDIV1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDIV1 division factor"]
    #[inline(always)]
    #[must_use]
    pub fn prediv1(&mut self) -> PREDIV1_W<CFGR2rs> {
        PREDIV1_W::new(self, 0)
    }
}
#[doc = "Clock configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
