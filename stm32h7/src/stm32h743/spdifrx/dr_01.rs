#[doc = "Reader of register DR_01"]
pub type R = crate::R<u32, super::DR_01>;
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
#[doc = "Reader of field `DR`"]
pub type DR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bit 0 - Parity Error bit"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Validity bit"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - User bit"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel Status bit"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Preamble Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:31 - Data value"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
