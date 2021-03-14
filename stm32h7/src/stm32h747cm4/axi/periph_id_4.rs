#[doc = "Reader of register PERIPH_ID_4"]
pub type R = crate::R<u32, super::PERIPH_ID_4>;
#[doc = "Reader of field `JEP106CON`"]
pub type JEP106CON_R = crate::R<u8, u8>;
#[doc = "Reader of field `KCOUNT4`"]
pub type KCOUNT4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Register file size"]
    #[inline(always)]
    pub fn kcount4(&self) -> KCOUNT4_R {
        KCOUNT4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
