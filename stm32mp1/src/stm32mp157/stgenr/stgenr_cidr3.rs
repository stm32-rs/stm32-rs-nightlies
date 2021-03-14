#[doc = "Reader of register STGENR_CIDR3"]
pub type R = crate::R<u32, super::STGENR_CIDR3>;
#[doc = "Reader of field `PRMBL_3`"]
pub type PRMBL_3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PRMBL_3"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
