#[doc = "Register `DBG_IDCODE` reader"]
pub type R = crate::R<DBG_IDCODErs>;
#[doc = "Field `DEV_ID` reader - Device identifier This bitfield indicates the device ID."]
pub type DEV_ID_R = crate::FieldReader<u16>;
#[doc = "Field `REV_ID` reader - Revision identifier This bitfield indicates the revision of the device."]
pub type REV_ID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Device identifier This bitfield indicates the device ID."]
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - Revision identifier This bitfield indicates the revision of the device."]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DBG device ID code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_idcode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_IDCODErs;
impl crate::RegisterSpec for DBG_IDCODErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_idcode::R`](R) reader structure"]
impl crate::Readable for DBG_IDCODErs {}
#[doc = "`reset()` method sets DBG_IDCODE to value 0x1000_0453"]
impl crate::Resettable for DBG_IDCODErs {
    const RESET_VALUE: u32 = 0x1000_0453;
}
