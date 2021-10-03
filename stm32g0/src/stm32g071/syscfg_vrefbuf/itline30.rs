#[doc = "Register `ITLINE30` reader"]
pub struct R(crate::R<ITLINE30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USART2` reader - CEC"]
pub struct USART2_R(crate::FieldReader<bool, bool>);
impl USART2_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CEC"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 30 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline30](index.html) module"]
pub struct ITLINE30_SPEC;
impl crate::RegisterSpec for ITLINE30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline30::R](R) reader structure"]
impl crate::Readable for ITLINE30_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE30 to value 0"]
impl crate::Resettable for ITLINE30_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
