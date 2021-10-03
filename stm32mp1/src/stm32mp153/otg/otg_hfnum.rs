#[doc = "Register `OTG_HFNUM` reader"]
pub struct R(crate::R<OTG_HFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRNUM` reader - FRNUM"]
pub struct FRNUM_R(crate::FieldReader<u16, u16>);
impl FRNUM_R {
    pub(crate) fn new(bits: u16) -> Self {
        FRNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRNUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTREM` reader - FTREM"]
pub struct FTREM_R(crate::FieldReader<u16, u16>);
impl FTREM_R {
    pub(crate) fn new(bits: u16) -> Self {
        FTREM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTREM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - FRNUM"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - FTREM"]
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hfnum](index.html) module"]
pub struct OTG_HFNUM_SPEC;
impl crate::RegisterSpec for OTG_HFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hfnum::R](R) reader structure"]
impl crate::Readable for OTG_HFNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_HFNUM to value 0x3fff"]
impl crate::Resettable for OTG_HFNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff
    }
}
