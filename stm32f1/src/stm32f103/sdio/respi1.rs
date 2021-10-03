#[doc = "Register `RESPI1` reader"]
pub struct R(crate::R<RESPI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESPI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESPI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESPI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARDSTATUS1` reader - CARDSTATUS1"]
pub struct CARDSTATUS1_R(crate::FieldReader<u32, u32>);
impl CARDSTATUS1_R {
    pub(crate) fn new(bits: u32) -> Self {
        CARDSTATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARDSTATUS1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS1"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respi1](index.html) module"]
pub struct RESPI1_SPEC;
impl crate::RegisterSpec for RESPI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [respi1::R](R) reader structure"]
impl crate::Readable for RESPI1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESPI1 to value 0"]
impl crate::Resettable for RESPI1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
