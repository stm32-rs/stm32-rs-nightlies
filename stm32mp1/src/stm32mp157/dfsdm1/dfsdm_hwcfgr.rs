#[doc = "Register `DFSDM_HWCFGR` reader"]
pub type R = crate::R<DFSDM_HWCFGRrs>;
#[doc = "Field `NBT` reader - NBT"]
pub type NBT_R = crate::FieldReader;
#[doc = "Field `NBF` reader - NBF"]
pub type NBF_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - NBT"]
    #[inline(always)]
    pub fn nbt(&self) -> NBT_R {
        NBT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NBF"]
    #[inline(always)]
    pub fn nbf(&self) -> NBF_R {
        NBF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "This register specifies the hardware configuration of DFSDM peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_HWCFGRrs;
impl crate::RegisterSpec for DFSDM_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_hwcfgr::R`](R) reader structure"]
impl crate::Readable for DFSDM_HWCFGRrs {}
#[doc = "`reset()` method sets DFSDM_HWCFGR to value 0x0608"]
impl crate::Resettable for DFSDM_HWCFGRrs {
    const RESET_VALUE: u32 = 0x0608;
}
