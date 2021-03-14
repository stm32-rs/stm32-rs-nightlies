#[doc = "Reader of register STGENC_PIDR0"]
pub type R = crate::R<u32, super::STGENC_PIDR0>;
#[doc = "Reader of field `PART_0`"]
pub type PART_0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PART_0"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
