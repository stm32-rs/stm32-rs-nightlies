#[doc = "Reader of register OTG_DEACHINT"]
pub type R = crate::R<u32, super::OTG_DEACHINT>;
#[doc = "Reader of field `IEP1INT`"]
pub type IEP1INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OEP1INT`"]
pub type OEP1INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - IEP1INT"]
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OEP1INT"]
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
