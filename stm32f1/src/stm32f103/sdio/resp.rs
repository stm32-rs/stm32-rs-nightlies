#[doc = "Register `RESP%s` reader"]
pub struct R(crate::R<RESP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARDSTATUS2` reader - CARDSTATUS2"]
pub struct CARDSTATUS2_R(crate::FieldReader<u32, u32>);
impl CARDSTATUS2_R {
    pub(crate) fn new(bits: u32) -> Self {
        CARDSTATUS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARDSTATUS2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS2"]
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp](index.html) module"]
pub struct RESP_SPEC;
impl crate::RegisterSpec for RESP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp::R](R) reader structure"]
impl crate::Readable for RESP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP%s to value 0"]
impl crate::Resettable for RESP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
