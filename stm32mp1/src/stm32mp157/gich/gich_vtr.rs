#[doc = "Reader of register GICH_VTR"]
pub type R = crate::R<u32, super::GICH_VTR>;
#[doc = "Reader of field `LISTREGS`"]
pub type LISTREGS_R = crate::R<u8, u8>;
#[doc = "Reader of field `PREBITS`"]
pub type PREBITS_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRIBITS`"]
pub type PRIBITS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - LISTREGS"]
    #[inline(always)]
    pub fn listregs(&self) -> LISTREGS_R {
        LISTREGS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 26:28 - PREBITS"]
    #[inline(always)]
    pub fn prebits(&self) -> PREBITS_R {
        PREBITS_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - PRIBITS"]
    #[inline(always)]
    pub fn pribits(&self) -> PRIBITS_R {
        PRIBITS_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
