#[doc = "Register `IDCODER` reader"]
pub type R = crate::R<IDCODERrs>;
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
#[doc = "DBGMCU Identity Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcoder::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDCODERrs;
impl crate::RegisterSpec for IDCODERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcoder::R`](R) reader structure"]
impl crate::Readable for IDCODERrs {}
#[doc = "`reset()` method sets IDCODER to value 0x1000_6497"]
impl crate::Resettable for IDCODERrs {
    const RESET_VALUE: u32 = 0x1000_6497;
}
