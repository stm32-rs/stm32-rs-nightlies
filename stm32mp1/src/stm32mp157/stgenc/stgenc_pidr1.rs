#[doc = "Reader of register STGENC_PIDR1"]
pub type R = crate::R<u32, super::STGENC_PIDR1>;
#[doc = "Reader of field `PART_1`"]
pub type PART_1_R = crate::R<u8, u8>;
#[doc = "Reader of field `DES_0`"]
pub type DES_0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - PART_1"]
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DES_0"]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
