#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PREDIV division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREDIV_A {
    #[doc = "0: PREDIV input clock not divided"]
    DIV1 = 0,
    #[doc = "1: PREDIV input clock divided by 2"]
    DIV2 = 1,
    #[doc = "2: PREDIV input clock divided by 3"]
    DIV3 = 2,
    #[doc = "3: PREDIV input clock divided by 4"]
    DIV4 = 3,
    #[doc = "4: PREDIV input clock divided by 5"]
    DIV5 = 4,
    #[doc = "5: PREDIV input clock divided by 6"]
    DIV6 = 5,
    #[doc = "6: PREDIV input clock divided by 7"]
    DIV7 = 6,
    #[doc = "7: PREDIV input clock divided by 8"]
    DIV8 = 7,
    #[doc = "8: PREDIV input clock divided by 9"]
    DIV9 = 8,
    #[doc = "9: PREDIV input clock divided by 10"]
    DIV10 = 9,
    #[doc = "10: PREDIV input clock divided by 11"]
    DIV11 = 10,
    #[doc = "11: PREDIV input clock divided by 12"]
    DIV12 = 11,
    #[doc = "12: PREDIV input clock divided by 13"]
    DIV13 = 12,
    #[doc = "13: PREDIV input clock divided by 14"]
    DIV14 = 13,
    #[doc = "14: PREDIV input clock divided by 15"]
    DIV15 = 14,
    #[doc = "15: PREDIV input clock divided by 16"]
    DIV16 = 15,
}
impl From<PREDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PREDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PREDIV` reader - PREDIV division factor"]
pub struct PREDIV_R(crate::FieldReader<u8, PREDIV_A>);
impl PREDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PREDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREDIV_A {
        match self.bits {
            0 => PREDIV_A::DIV1,
            1 => PREDIV_A::DIV2,
            2 => PREDIV_A::DIV3,
            3 => PREDIV_A::DIV4,
            4 => PREDIV_A::DIV5,
            5 => PREDIV_A::DIV6,
            6 => PREDIV_A::DIV7,
            7 => PREDIV_A::DIV8,
            8 => PREDIV_A::DIV9,
            9 => PREDIV_A::DIV10,
            10 => PREDIV_A::DIV11,
            11 => PREDIV_A::DIV12,
            12 => PREDIV_A::DIV13,
            13 => PREDIV_A::DIV14,
            14 => PREDIV_A::DIV15,
            15 => PREDIV_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PREDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PREDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        **self == PREDIV_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PREDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        **self == PREDIV_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PREDIV_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        **self == PREDIV_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PREDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        **self == PREDIV_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == PREDIV_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        **self == PREDIV_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        **self == PREDIV_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        **self == PREDIV_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        **self == PREDIV_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        **self == PREDIV_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PREDIV_A::DIV16
    }
}
impl core::ops::Deref for PREDIV_R {
    type Target = crate::FieldReader<u8, PREDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREDIV` writer - PREDIV division factor"]
pub struct PREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PREDIV input clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV1)
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV2)
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV3)
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV4)
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV5)
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV6)
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV7)
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV8)
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV9)
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV10)
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV11)
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV12)
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV13)
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV14)
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV15)
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&mut self) -> PREDIV_W {
        PREDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
