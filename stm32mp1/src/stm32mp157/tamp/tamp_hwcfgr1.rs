#[doc = "Register `TAMP_HWCFGR1` reader"]
pub struct R(crate::R<TAMP_HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BACKUP_REGS` reader - BACKUP_REGS"]
pub struct BACKUP_REGS_R(crate::FieldReader<u8, u8>);
impl BACKUP_REGS_R {
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_REGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_REGS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPER` reader - TAMPER"]
pub struct TAMPER_R(crate::FieldReader<u8, u8>);
impl TAMPER_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE_TAMPER` reader - ACTIVE_TAMPER"]
pub struct ACTIVE_TAMPER_R(crate::FieldReader<u8, u8>);
impl ACTIVE_TAMPER_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACTIVE_TAMPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE_TAMPER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_TAMPER` reader - INT_TAMPER"]
pub struct INT_TAMPER_R(crate::FieldReader<u16, u16>);
impl INT_TAMPER_R {
    pub(crate) fn new(bits: u16) -> Self {
        INT_TAMPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_TAMPER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - BACKUP_REGS"]
    #[inline(always)]
    pub fn backup_regs(&self) -> BACKUP_REGS_R {
        BACKUP_REGS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - TAMPER"]
    #[inline(always)]
    pub fn tamper(&self) -> TAMPER_R {
        TAMPER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ACTIVE_TAMPER"]
    #[inline(always)]
    pub fn active_tamper(&self) -> ACTIVE_TAMPER_R {
        ACTIVE_TAMPER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - INT_TAMPER"]
    #[inline(always)]
    pub fn int_tamper(&self) -> INT_TAMPER_R {
        INT_TAMPER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "TAMP hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_hwcfgr1](index.html) module"]
pub struct TAMP_HWCFGR1_SPEC;
impl crate::RegisterSpec for TAMP_HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_hwcfgr1::R](R) reader structure"]
impl crate::Readable for TAMP_HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAMP_HWCFGR1 to value 0x009d_1320"]
impl crate::Resettable for TAMP_HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x009d_1320
    }
}
