#[doc = "Register `HFNUM` reader"]
pub type R = crate::R<HFNUMrs>;
#[doc = "Field `FRNUM` reader - Frame number"]
pub type FRNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FTREM` reader - Frame time remaining"]
pub type FTREM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame time remaining"]
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_HS host frame number/frame time remaining register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFNUMrs;
impl crate::RegisterSpec for HFNUMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfnum::R`](R) reader structure"]
impl crate::Readable for HFNUMrs {}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HFNUMrs {
    const RESET_VALUE: u32 = 0x3fff;
}
