#[doc = "Reader of register FDCAN_RXESC"]
pub type R = crate::R<u32, super::FDCAN_RXESC>;
#[doc = "Reader of field `F0DS`"]
pub type F0DS_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1DS`"]
pub type F1DS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RBDS`"]
pub type RBDS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - F0DS"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - F1DS"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - RBDS"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
