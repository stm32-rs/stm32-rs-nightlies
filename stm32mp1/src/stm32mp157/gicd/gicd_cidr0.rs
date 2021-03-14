#[doc = "Reader of register GICD_CIDR0"]
pub type R = crate::R<u32, super::GICD_CIDR0>;
#[doc = "Reader of field `CIDR0`"]
pub type CIDR0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CIDR0"]
    #[inline(always)]
    pub fn cidr0(&self) -> CIDR0_R {
        CIDR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
