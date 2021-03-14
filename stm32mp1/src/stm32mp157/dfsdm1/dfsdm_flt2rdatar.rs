#[doc = "Reader of register DFSDM_FLT2RDATAR"]
pub type R = crate::R<u32, super::DFSDM_FLT2RDATAR>;
#[doc = "Reader of field `RDATACH`"]
pub type RDATACH_R = crate::R<u8, u8>;
#[doc = "Reader of field `RPEND`"]
pub type RPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDATA`"]
pub type RDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - RDATACH"]
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - RPEND"]
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - RDATA"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
