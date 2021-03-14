#[doc = "Reader of register STGENR_CIDR0"]
pub type R = crate::R<u32, super::STGENR_CIDR0>;
#[doc = "Reader of field `PRMBL_0`"]
pub type PRMBL_0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PRMBL_0"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
