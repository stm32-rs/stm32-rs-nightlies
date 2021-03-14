#[doc = "Reader of register ITLINE17"]
pub type R = crate::R<u32, super::ITLINE17>;
#[doc = "Reader of field `LPTIM1`"]
pub type LPTIM1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - LPTIM1"]
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
