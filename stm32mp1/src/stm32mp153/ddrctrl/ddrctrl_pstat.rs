#[doc = "Register `DDRCTRL_PSTAT` reader"]
pub struct R(crate::R<DDRCTRL_PSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_PORT_BUSY_0` reader - RD_PORT_BUSY_0"]
pub struct RD_PORT_BUSY_0_R(crate::FieldReader<bool, bool>);
impl RD_PORT_BUSY_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_PORT_BUSY_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_PORT_BUSY_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_PORT_BUSY_1` reader - RD_PORT_BUSY_1"]
pub struct RD_PORT_BUSY_1_R(crate::FieldReader<bool, bool>);
impl RD_PORT_BUSY_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_PORT_BUSY_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_PORT_BUSY_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_PORT_BUSY_0` reader - WR_PORT_BUSY_0"]
pub struct WR_PORT_BUSY_0_R(crate::FieldReader<bool, bool>);
impl WR_PORT_BUSY_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_PORT_BUSY_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_PORT_BUSY_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_PORT_BUSY_1` reader - WR_PORT_BUSY_1"]
pub struct WR_PORT_BUSY_1_R(crate::FieldReader<bool, bool>);
impl WR_PORT_BUSY_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_PORT_BUSY_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_PORT_BUSY_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RD_PORT_BUSY_0"]
    #[inline(always)]
    pub fn rd_port_busy_0(&self) -> RD_PORT_BUSY_0_R {
        RD_PORT_BUSY_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RD_PORT_BUSY_1"]
    #[inline(always)]
    pub fn rd_port_busy_1(&self) -> RD_PORT_BUSY_1_R {
        RD_PORT_BUSY_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WR_PORT_BUSY_0"]
    #[inline(always)]
    pub fn wr_port_busy_0(&self) -> WR_PORT_BUSY_0_R {
        WR_PORT_BUSY_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WR_PORT_BUSY_1"]
    #[inline(always)]
    pub fn wr_port_busy_1(&self) -> WR_PORT_BUSY_1_R {
        WR_PORT_BUSY_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "DDRCTRL port status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pstat](index.html) module"]
pub struct DDRCTRL_PSTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_PSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pstat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_PSTAT to value 0"]
impl crate::Resettable for DDRCTRL_PSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
