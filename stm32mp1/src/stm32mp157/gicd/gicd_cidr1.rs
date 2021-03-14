#[doc = "Reader of register GICD_CIDR1"]
pub type R = crate::R<u32, super::GICD_CIDR1>;
#[doc = "Reader of field `CIDR1`"]
pub type CIDR1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CIDR1"]
    #[inline(always)]
    pub fn cidr1(&self) -> CIDR1_R {
        CIDR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
