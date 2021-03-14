#[doc = "Reader of register PERIPH_ID_2"]
pub type R = crate::R<u32, super::PERIPH_ID_2>;
#[doc = "Reader of field `JEP106ID`"]
pub type JEP106ID_R = crate::R<u8, u8>;
#[doc = "Reader of field `JEDEC`"]
pub type JEDEC_R = crate::R<bool, bool>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - JEP106 Identity bits 4 to 6"]
    #[inline(always)]
    pub fn jep106id(&self) -> JEP106ID_R {
        JEP106ID_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - JEP106 code flag"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Peripheral revision number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
