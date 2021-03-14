#[doc = "Reader of register CDR"]
pub type R = crate::R<u32, super::CDR>;
#[doc = "Reader of field `RDATA_SLV`"]
pub type RDATA_SLV_R = crate::R<u16, u16>;
#[doc = "Reader of field `RDATA_MST`"]
pub type RDATA_MST_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Regular data of the slave ADC"]
    #[inline(always)]
    pub fn rdata_slv(&self) -> RDATA_SLV_R {
        RDATA_SLV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Regular data of the master ADC"]
    #[inline(always)]
    pub fn rdata_mst(&self) -> RDATA_MST_R {
        RDATA_MST_R::new((self.bits & 0xffff) as u16)
    }
}
