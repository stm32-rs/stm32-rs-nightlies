#[doc = "Register `OTG_DEACHINT` reader"]
pub type R = crate::R<OTG_DEACHINTrs>;
#[doc = "Field `IEP1INT` reader - IEP1INT"]
pub type IEP1INT_R = crate::BitReader;
#[doc = "Field `OEP1INT` reader - OEP1INT"]
pub type OEP1INT_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - IEP1INT"]
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OEP1INT"]
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "OTG device each endpoint interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_deachint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DEACHINTrs;
impl crate::RegisterSpec for OTG_DEACHINTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_deachint::R`](R) reader structure"]
impl crate::Readable for OTG_DEACHINTrs {}
#[doc = "`reset()` method sets OTG_DEACHINT to value 0"]
impl crate::Resettable for OTG_DEACHINTrs {
    const RESET_VALUE: u32 = 0;
}
