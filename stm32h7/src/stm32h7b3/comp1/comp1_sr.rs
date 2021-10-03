#[doc = "Register `COMP1_SR` reader"]
pub struct R(crate::R<COMP1_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `C1VAL` reader - COMP channel 1 output status bit"]
pub struct C1VAL_R(crate::FieldReader<bool, bool>);
impl C1VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C1VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2VAL` reader - COMP channel 2 output status bit"]
pub struct C2VAL_R(crate::FieldReader<bool, bool>);
impl C2VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C1IF` reader - COMP channel 1 Interrupt Flag"]
pub struct C1IF_R(crate::FieldReader<bool, bool>);
impl C1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2IF` reader - COMP channel 2 Interrupt Flag"]
pub struct C2IF_R(crate::FieldReader<bool, bool>);
impl C2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - COMP channel 1 output status bit"]
    #[inline(always)]
    pub fn c1val(&self) -> C1VAL_R {
        C1VAL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COMP channel 2 output status bit"]
    #[inline(always)]
    pub fn c2val(&self) -> C2VAL_R {
        C2VAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - COMP channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn c1if(&self) -> C1IF_R {
        C1IF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - COMP channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn c2if(&self) -> C2IF_R {
        C2IF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Comparator status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_sr](index.html) module"]
pub struct COMP1_SR_SPEC;
impl crate::RegisterSpec for COMP1_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_sr::R](R) reader structure"]
impl crate::Readable for COMP1_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMP1_SR to value 0"]
impl crate::Resettable for COMP1_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
