#[doc = "Reader of register HSEM_HWCFGR2"]
pub type R = crate::R<u32, super::HSEM_HWCFGR2>;
#[doc = "Reader of field `MASTERID1`"]
pub type MASTERID1_R = crate::R<u8, u8>;
#[doc = "Reader of field `MASTERID2`"]
pub type MASTERID2_R = crate::R<u8, u8>;
#[doc = "Reader of field `MASTERID3`"]
pub type MASTERID3_R = crate::R<u8, u8>;
#[doc = "Reader of field `MASTERID4`"]
pub type MASTERID4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - MASTERID1"]
    #[inline(always)]
    pub fn masterid1(&self) -> MASTERID1_R {
        MASTERID1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MASTERID2"]
    #[inline(always)]
    pub fn masterid2(&self) -> MASTERID2_R {
        MASTERID2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - MASTERID3"]
    #[inline(always)]
    pub fn masterid3(&self) -> MASTERID3_R {
        MASTERID3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - MASTERID4"]
    #[inline(always)]
    pub fn masterid4(&self) -> MASTERID4_R {
        MASTERID4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
