#[doc = "Register `EXTICR2` reader"]
pub struct R(crate::R<EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR2` writer"]
pub struct W(crate::W<EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR2_SPEC>;
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
impl From<crate::W<EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO port selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI0_7_A {
    #[doc = "0: GPIO port A selected"]
    PA = 0,
    #[doc = "1: GPIO port B selected"]
    PB = 1,
    #[doc = "2: GPIO port C selected"]
    PC = 2,
    #[doc = "3: GPIO port D selected"]
    PD = 3,
    #[doc = "5: GPIO port F selected"]
    PF = 5,
}
impl From<EXTI0_7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI0_7` reader - GPIO port selection"]
pub struct EXTI0_7_R(crate::FieldReader<u8, EXTI0_7_A>);
impl EXTI0_7_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI0_7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI0_7_A> {
        match self.bits {
            0 => Some(EXTI0_7_A::PA),
            1 => Some(EXTI0_7_A::PB),
            2 => Some(EXTI0_7_A::PC),
            3 => Some(EXTI0_7_A::PD),
            5 => Some(EXTI0_7_A::PF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA`"]
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        **self == EXTI0_7_A::PA
    }
    #[doc = "Checks if the value of the field is `PB`"]
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        **self == EXTI0_7_A::PB
    }
    #[doc = "Checks if the value of the field is `PC`"]
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        **self == EXTI0_7_A::PC
    }
    #[doc = "Checks if the value of the field is `PD`"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        **self == EXTI0_7_A::PD
    }
    #[doc = "Checks if the value of the field is `PF`"]
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        **self == EXTI0_7_A::PF
    }
}
impl core::ops::Deref for EXTI0_7_R {
    type Target = crate::FieldReader<u8, EXTI0_7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI0_7` writer - GPIO port selection"]
pub struct EXTI0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI0_7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PA)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PB)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PC)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PD)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "GPIO port selection"]
pub type EXTI8_15_A = EXTI0_7_A;
#[doc = "Field `EXTI8_15` reader - GPIO port selection"]
pub type EXTI8_15_R = EXTI0_7_R;
#[doc = "Field `EXTI8_15` writer - GPIO port selection"]
pub struct EXTI8_15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI8_15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut W {
        self.variant(EXTI8_15_A::PA)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut W {
        self.variant(EXTI8_15_A::PB)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut W {
        self.variant(EXTI8_15_A::PC)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(EXTI8_15_A::PD)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut W {
        self.variant(EXTI8_15_A::PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "GPIO port selection"]
pub type EXTI16_23_A = EXTI0_7_A;
#[doc = "Field `EXTI16_23` reader - GPIO port selection"]
pub type EXTI16_23_R = EXTI0_7_R;
#[doc = "Field `EXTI16_23` writer - GPIO port selection"]
pub struct EXTI16_23_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI16_23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI16_23_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut W {
        self.variant(EXTI16_23_A::PA)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut W {
        self.variant(EXTI16_23_A::PB)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut W {
        self.variant(EXTI16_23_A::PC)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(EXTI16_23_A::PD)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut W {
        self.variant(EXTI16_23_A::PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "GPIO port selection"]
pub type EXTI24_31_A = EXTI0_7_A;
#[doc = "Field `EXTI24_31` reader - GPIO port selection"]
pub type EXTI24_31_R = EXTI0_7_R;
#[doc = "Field `EXTI24_31` writer - GPIO port selection"]
pub struct EXTI24_31_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI24_31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI24_31_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut W {
        self.variant(EXTI24_31_A::PA)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut W {
        self.variant(EXTI24_31_A::PB)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut W {
        self.variant(EXTI24_31_A::PC)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(EXTI24_31_A::PD)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut W {
        self.variant(EXTI24_31_A::PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&mut self) -> EXTI0_7_W {
        EXTI0_7_W { w: self }
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&mut self) -> EXTI8_15_W {
        EXTI8_15_W { w: self }
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&mut self) -> EXTI16_23_W {
        EXTI16_23_W { w: self }
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&mut self) -> EXTI24_31_W {
        EXTI24_31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](index.html) module"]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr2::R](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr2::W](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
