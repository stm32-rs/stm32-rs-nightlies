#[doc = "Reader of register AXIMC_PERIPH_ID_6"]
pub type R = crate::R<u32, super::AXIMC_PERIPH_ID_6>;
#[doc = "Reader of field `PERIPH_ID_6`"]
pub type PERIPH_ID_6_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PERIPH_ID_6"]
    #[inline(always)]
    pub fn periph_id_6(&self) -> PERIPH_ID_6_R {
        PERIPH_ID_6_R::new((self.bits & 0xff) as u8)
    }
}
