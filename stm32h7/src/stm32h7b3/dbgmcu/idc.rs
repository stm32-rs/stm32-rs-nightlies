#[doc = "Register `IDC` reader"]
pub type R = crate::R<IDCrs>;
#[doc = "Field `DEV_ID` reader - Device ID"]
pub type DEV_ID_R = crate::FieldReader<u16>;
#[doc = "Field `REV_ID` reader - Revision"]
pub type REV_ID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Device ID"]
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - Revision"]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DBGMCU Identity Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDCrs;
impl crate::RegisterSpec for IDCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idc::R`](R) reader structure"]
impl crate::Readable for IDCrs {}
#[doc = "`reset()` method sets IDC to value 0x1000_6480"]
impl crate::Resettable for IDCrs {
    const RESET_VALUE: u32 = 0x1000_6480;
}
