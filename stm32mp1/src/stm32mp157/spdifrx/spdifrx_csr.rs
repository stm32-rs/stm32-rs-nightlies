#[doc = "Reader of register SPDIFRX_CSR"]
pub type R = crate::R<u32, super::SPDIFRX_CSR>;
#[doc = "Reader of field `USR`"]
pub type USR_R = crate::R<u16, u16>;
#[doc = "Reader of field `CS`"]
pub type CS_R = crate::R<u8, u8>;
#[doc = "Reader of field `SOB`"]
pub type SOB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - USR"]
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - CS"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - SOB"]
    #[inline(always)]
    pub fn sob(&self) -> SOB_R {
        SOB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
