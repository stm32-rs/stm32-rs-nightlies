#[doc = "Reader of register PERIPH_ID_1"]
pub type R = crate::R<u32, super::PERIPH_ID_1>;
#[doc = "Reader of field `PARTNUM`"]
pub type PARTNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `JEP106I`"]
pub type JEP106I_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Peripheral part number bits 8 to 11"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity bits 0 to 3"]
    #[inline(always)]
    pub fn jep106i(&self) -> JEP106I_R {
        JEP106I_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
