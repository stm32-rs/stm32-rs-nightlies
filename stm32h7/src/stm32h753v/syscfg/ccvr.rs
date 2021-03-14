#[doc = "Reader of register CCVR"]
pub type R = crate::R<u32, super::CCVR>;
#[doc = "Reader of field `NCV`"]
pub type NCV_R = crate::R<u8, u8>;
#[doc = "Reader of field `PCV`"]
pub type PCV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - NMOS compensation value"]
    #[inline(always)]
    pub fn ncv(&self) -> NCV_R {
        NCV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PMOS compensation value"]
    #[inline(always)]
    pub fn pcv(&self) -> PCV_R {
        PCV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
