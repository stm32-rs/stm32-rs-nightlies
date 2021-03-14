#[doc = "Reader of register AHB3SECSR"]
pub type R = crate::R<u32, super::AHB3SECSR>;
#[doc = "Reader of field `OSPI1SECF`"]
pub type OSPI1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FSMCSECF`"]
pub type FSMCSECF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8 - OSPI1SECF"]
    #[inline(always)]
    pub fn ospi1secf(&self) -> OSPI1SECF_R {
        OSPI1SECF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FSMCSECF"]
    #[inline(always)]
    pub fn fsmcsecf(&self) -> FSMCSECF_R {
        FSMCSECF_R::new((self.bits & 0x01) != 0)
    }
}
