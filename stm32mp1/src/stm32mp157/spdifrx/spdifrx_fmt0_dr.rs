#[doc = "Reader of register SPDIFRX_FMT0_DR"]
pub type R = crate::R<u32, super::SPDIFRX_FMT0_DR>;
#[doc = "Reader of field `DR`"]
pub type DR_R = crate::R<u32, u32>;
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, bool>;
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, bool>;
#[doc = "Reader of field `U`"]
pub type U_R = crate::R<bool, bool>;
#[doc = "Reader of field `C`"]
pub type C_R = crate::R<bool, bool>;
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:23 - DR"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - V"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - U"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - C"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
