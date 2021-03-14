#[doc = "Reader of register ETH_MACVR"]
pub type R = crate::R<u32, super::ETH_MACVR>;
#[doc = "Reader of field `SNPSVER`"]
pub type SNPSVER_R = crate::R<u8, u8>;
#[doc = "Reader of field `USERVER`"]
pub type USERVER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SNPSVER"]
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USERVER"]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
