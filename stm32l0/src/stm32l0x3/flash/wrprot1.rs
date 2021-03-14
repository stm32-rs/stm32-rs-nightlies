#[doc = "Reader of register WRPROT1"]
pub type R = crate::R<u32, super::WRPROT1>;
#[doc = "Reader of field `WRPROT1`"]
pub type WRPROT1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write Protection"]
    #[inline(always)]
    pub fn wrprot1(&self) -> WRPROT1_R {
        WRPROT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
