#[doc = "Register `DDRCTRL_ZQSTAT` reader"]
pub struct R(crate::R<DDRCTRL_ZQSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ZQSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ZQSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ZQSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZQ_RESET_BUSY` reader - ZQ_RESET_BUSY"]
pub struct ZQ_RESET_BUSY_R(crate::FieldReader<bool, bool>);
impl ZQ_RESET_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZQ_RESET_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZQ_RESET_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ZQ_RESET_BUSY"]
    #[inline(always)]
    pub fn zq_reset_busy(&self) -> ZQ_RESET_BUSY_R {
        ZQ_RESET_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DDRCTRL ZQ status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqstat](index.html) module"]
pub struct DDRCTRL_ZQSTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_ZQSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_zqstat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_ZQSTAT to value 0"]
impl crate::Resettable for DDRCTRL_ZQSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
