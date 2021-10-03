#[doc = "Register `SMPR` reader"]
pub struct R(crate::R<SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR` writer"]
pub struct W(crate::W<SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR_SPEC>;
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
impl From<crate::W<SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    CYCLES1_5 = 0,
    #[doc = "1: 3.5 ADC clock cycles"]
    CYCLES3_5 = 1,
    #[doc = "2: 7.5 ADC clock cycles"]
    CYCLES7_5 = 2,
    #[doc = "3: 12.5 ADC clock cycles"]
    CYCLES12_5 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    CYCLES19_5 = 4,
    #[doc = "5: 39.5 ADC clock cycles"]
    CYCLES39_5 = 5,
    #[doc = "6: 79.5 ADC clock cycles"]
    CYCLES79_5 = 6,
    #[doc = "7: 160.5 ADC clock cycles"]
    CYCLES160_5 = 7,
}
impl From<SMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMP` reader - Sampling time selection"]
pub struct SMP_R(crate::FieldReader<u8, SMP_A>);
impl SMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            0 => SMP_A::CYCLES1_5,
            1 => SMP_A::CYCLES3_5,
            2 => SMP_A::CYCLES7_5,
            3 => SMP_A::CYCLES12_5,
            4 => SMP_A::CYCLES19_5,
            5 => SMP_A::CYCLES39_5,
            6 => SMP_A::CYCLES79_5,
            7 => SMP_A::CYCLES160_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES1_5`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        **self == SMP_A::CYCLES1_5
    }
    #[doc = "Checks if the value of the field is `CYCLES3_5`"]
    #[inline(always)]
    pub fn is_cycles3_5(&self) -> bool {
        **self == SMP_A::CYCLES3_5
    }
    #[doc = "Checks if the value of the field is `CYCLES7_5`"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        **self == SMP_A::CYCLES7_5
    }
    #[doc = "Checks if the value of the field is `CYCLES12_5`"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        **self == SMP_A::CYCLES12_5
    }
    #[doc = "Checks if the value of the field is `CYCLES19_5`"]
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        **self == SMP_A::CYCLES19_5
    }
    #[doc = "Checks if the value of the field is `CYCLES39_5`"]
    #[inline(always)]
    pub fn is_cycles39_5(&self) -> bool {
        **self == SMP_A::CYCLES39_5
    }
    #[doc = "Checks if the value of the field is `CYCLES79_5`"]
    #[inline(always)]
    pub fn is_cycles79_5(&self) -> bool {
        **self == SMP_A::CYCLES79_5
    }
    #[doc = "Checks if the value of the field is `CYCLES160_5`"]
    #[inline(always)]
    pub fn is_cycles160_5(&self) -> bool {
        **self == SMP_A::CYCLES160_5
    }
}
impl core::ops::Deref for SMP_R {
    type Target = crate::FieldReader<u8, SMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP` writer - Sampling time selection"]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES1_5)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles3_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES3_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES7_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES12_5)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES19_5)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles39_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES39_5)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles79_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES79_5)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles160_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES160_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sampling time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr](index.html) module"]
pub struct SMPR_SPEC;
impl crate::RegisterSpec for SMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr::R](R) reader structure"]
impl crate::Readable for SMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr::W](W) writer structure"]
impl crate::Writable for SMPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SMPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
