#[doc = "Reader of register AXIMC_PERIPH_ID_4"]
pub type R = crate::R<u32, super::AXIMC_PERIPH_ID_4>;
#[doc = "Reader of field `JEP106CON`"]
pub type JEP106CON_R = crate::R<u8, u8>;
#[doc = "Reader of field `K4COUNT`"]
pub type K4COUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP106CON"]
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - K4COUNT"]
    #[inline(always)]
    pub fn k4count(&self) -> K4COUNT_R {
        K4COUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
