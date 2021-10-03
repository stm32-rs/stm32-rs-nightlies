#[doc = "Register `DDRCTRL_POISONSTAT` reader"]
pub struct R(crate::R<DDRCTRL_POISONSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_POISONSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_POISONSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_POISONSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WR_POISON_INTR_0` reader - WR_POISON_INTR_0"]
pub struct WR_POISON_INTR_0_R(crate::FieldReader<bool, bool>);
impl WR_POISON_INTR_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_POISON_INTR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_POISON_INTR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_POISON_INTR_1` reader - WR_POISON_INTR_1"]
pub struct WR_POISON_INTR_1_R(crate::FieldReader<bool, bool>);
impl WR_POISON_INTR_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_POISON_INTR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_POISON_INTR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_POISON_INTR_0` reader - RD_POISON_INTR_0"]
pub struct RD_POISON_INTR_0_R(crate::FieldReader<bool, bool>);
impl RD_POISON_INTR_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_POISON_INTR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_POISON_INTR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_POISON_INTR_1` reader - RD_POISON_INTR_1"]
pub struct RD_POISON_INTR_1_R(crate::FieldReader<bool, bool>);
impl RD_POISON_INTR_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_POISON_INTR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_POISON_INTR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - WR_POISON_INTR_0"]
    #[inline(always)]
    pub fn wr_poison_intr_0(&self) -> WR_POISON_INTR_0_R {
        WR_POISON_INTR_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WR_POISON_INTR_1"]
    #[inline(always)]
    pub fn wr_poison_intr_1(&self) -> WR_POISON_INTR_1_R {
        WR_POISON_INTR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RD_POISON_INTR_0"]
    #[inline(always)]
    pub fn rd_poison_intr_0(&self) -> RD_POISON_INTR_0_R {
        RD_POISON_INTR_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RD_POISON_INTR_1"]
    #[inline(always)]
    pub fn rd_poison_intr_1(&self) -> RD_POISON_INTR_1_R {
        RD_POISON_INTR_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "DDRCTRL AXI Poison status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_poisonstat](index.html) module"]
pub struct DDRCTRL_POISONSTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_POISONSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_poisonstat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_POISONSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_POISONSTAT to value 0"]
impl crate::Resettable for DDRCTRL_POISONSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
