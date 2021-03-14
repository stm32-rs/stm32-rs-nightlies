#[doc = "Reader of register FDCAN_TTLGT"]
pub type R = crate::R<u32, super::FDCAN_TTLGT>;
#[doc = "Reader of field `LT`"]
pub type LT_R = crate::R<u16, u16>;
#[doc = "Reader of field `GT`"]
pub type GT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - LT"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - GT"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
