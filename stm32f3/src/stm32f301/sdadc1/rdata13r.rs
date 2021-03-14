#[doc = "Reader of register RDATA13R"]
pub type R = crate::R<u32, super::RDATA13R>;
#[doc = "Reader of field `RDATA3`"]
pub type RDATA3_R = crate::R<u16, u16>;
#[doc = "Reader of field `RDATA1`"]
pub type RDATA1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Regular conversion data for SDADC3"]
    #[inline(always)]
    pub fn rdata3(&self) -> RDATA3_R {
        RDATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Regular conversion data for SDADC1"]
    #[inline(always)]
    pub fn rdata1(&self) -> RDATA1_R {
        RDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
