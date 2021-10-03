#[doc = "Register `GPIOE_HWCFGR10` reader"]
pub struct R(crate::R<GPIOE_HWCFGR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_HWCFGR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_HWCFGR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_HWCFGR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AHB_IOP` reader - AHB_IOP"]
pub struct AHB_IOP_R(crate::FieldReader<u8, u8>);
impl AHB_IOP_R {
    pub(crate) fn new(bits: u8) -> Self {
        AHB_IOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_IOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_SIZE` reader - AF_SIZE"]
pub struct AF_SIZE_R(crate::FieldReader<u8, u8>);
impl AF_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEED_CFG` reader - SPEED_CFG"]
pub struct SPEED_CFG_R(crate::FieldReader<u8, u8>);
impl SPEED_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPEED_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPEED_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_CFG` reader - LOCK_CFG"]
pub struct LOCK_CFG_R(crate::FieldReader<u8, u8>);
impl LOCK_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOCK_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_CFG` reader - SEC_CFG"]
pub struct SEC_CFG_R(crate::FieldReader<u8, u8>);
impl SEC_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OR_CFG` reader - OR_CFG"]
pub struct OR_CFG_R(crate::FieldReader<u8, u8>);
impl OR_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        OR_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OR_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - AHB_IOP"]
    #[inline(always)]
    pub fn ahb_iop(&self) -> AHB_IOP_R {
        AHB_IOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AF_SIZE"]
    #[inline(always)]
    pub fn af_size(&self) -> AF_SIZE_R {
        AF_SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SPEED_CFG"]
    #[inline(always)]
    pub fn speed_cfg(&self) -> SPEED_CFG_R {
        SPEED_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - LOCK_CFG"]
    #[inline(always)]
    pub fn lock_cfg(&self) -> LOCK_CFG_R {
        LOCK_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SEC_CFG"]
    #[inline(always)]
    pub fn sec_cfg(&self) -> SEC_CFG_R {
        SEC_CFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - OR_CFG"]
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr10](index.html) module"]
pub struct GPIOE_HWCFGR10_SPEC;
impl crate::RegisterSpec for GPIOE_HWCFGR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_hwcfgr10::R](R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOE_HWCFGR10 to value 0x0001_1240"]
impl crate::Resettable for GPIOE_HWCFGR10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_1240
    }
}
