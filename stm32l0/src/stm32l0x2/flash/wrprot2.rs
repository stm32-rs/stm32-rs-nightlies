#[doc = "Reader of register WRPROT2"]
pub type R = crate::R<u32, super::WRPROT2>;
#[doc = "Reader of field `WRPROT2`"]
pub type WRPROT2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write Protection"]
    #[inline(always)]
    pub fn wrprot2(&self) -> WRPROT2_R {
        WRPROT2_R::new((self.bits & 0xffff) as u16)
    }
}
