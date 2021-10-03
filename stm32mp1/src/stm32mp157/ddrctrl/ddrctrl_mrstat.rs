#[doc = "Register `DDRCTRL_MRSTAT` reader"]
pub struct R(crate::R<DDRCTRL_MRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_MRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_MRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_MRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MR_WR_BUSY` reader - MR_WR_BUSY"]
pub struct MR_WR_BUSY_R(crate::FieldReader<bool, bool>);
impl MR_WR_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR_WR_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_WR_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - MR_WR_BUSY"]
    #[inline(always)]
    pub fn mr_wr_busy(&self) -> MR_WR_BUSY_R {
        MR_WR_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DDRCTRL mode register read/write status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mrstat](index.html) module"]
pub struct DDRCTRL_MRSTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_MRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_mrstat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_MRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_MRSTAT to value 0"]
impl crate::Resettable for DDRCTRL_MRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
