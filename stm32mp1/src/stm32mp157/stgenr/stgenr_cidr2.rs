#[doc = "Reader of register STGENR_CIDR2"]
pub type R = crate::R<u32, super::STGENR_CIDR2>;
#[doc = "Reader of field `PRMBL_2`"]
pub type PRMBL_2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PRMBL_2"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
