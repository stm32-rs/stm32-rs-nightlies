#[doc = "Reader of register PERIPH_ID_0"]
pub type R = crate::R<u32, super::PERIPH_ID_0>;
#[doc = "Reader of field `PARTNUM`"]
pub type PARTNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral part number bits 0 to 7"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
