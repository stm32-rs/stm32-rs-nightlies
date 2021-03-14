#[doc = "Reader of register TTLGT"]
pub type R = crate::R<u32, super::TTLGT>;
#[doc = "Reader of field `LT`"]
pub type LT_R = crate::R<u16, u16>;
#[doc = "Reader of field `GT`"]
pub type GT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Local Time"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Global Time"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
