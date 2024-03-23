#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IDCODErs>;
#[doc = "Field `DEV_ID` reader - device identification"]
pub type DEV_ID_R = crate::FieldReader<u16>;
#[doc = "Field `REV_ID` reader - revision This field indicates the revision of the device."]
pub type REV_ID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - device identification"]
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - revision This field indicates the revision of the device."]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DBGMCU identity code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDCODErs;
impl crate::RegisterSpec for IDCODErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IDCODErs {}
#[doc = "`reset()` method sets IDCODE to value 0x6000"]
impl crate::Resettable for IDCODErs {
    const RESET_VALUE: u32 = 0x6000;
}
