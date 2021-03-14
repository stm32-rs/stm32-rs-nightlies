#[doc = "Reader of register GICD_CIDR3"]
pub type R = crate::R<u32, super::GICD_CIDR3>;
#[doc = "Reader of field `CIDR3`"]
pub type CIDR3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CIDR3"]
    #[inline(always)]
    pub fn cidr3(&self) -> CIDR3_R {
        CIDR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
