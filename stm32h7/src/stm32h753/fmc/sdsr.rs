#[doc = "Reader of register SDSR"]
pub type R = crate::R<u32, super::SDSR>;
#[doc = "Reader of field `RE`"]
pub type RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MODES1`"]
pub type MODES1_R = crate::R<u8, u8>;
#[doc = "Reader of field `MODES2`"]
pub type MODES2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1."]
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Status Mode for Bank 2 These bits define the Status Mode of SDRAM Bank 2."]
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
