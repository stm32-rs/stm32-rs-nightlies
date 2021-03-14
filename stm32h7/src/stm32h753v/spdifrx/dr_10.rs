#[doc = "Reader of register DR_10"]
pub type R = crate::R<u32, super::DR_10>;
#[doc = "Reader of field `DRNL1`"]
pub type DRNL1_R = crate::R<u16, u16>;
#[doc = "Reader of field `DRNL2`"]
pub type DRNL2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data value"]
    #[inline(always)]
    pub fn drnl1(&self) -> DRNL1_R {
        DRNL1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data value"]
    #[inline(always)]
    pub fn drnl2(&self) -> DRNL2_R {
        DRNL2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
