#[doc = "Reader of register GPIOC_HWCFGR3"]
pub type R = crate::R<u32, super::GPIOC_HWCFGR3>;
#[doc = "Reader of field `ODR_RES`"]
pub type ODR_RES_R = crate::R<u16, u16>;
#[doc = "Reader of field `OTYPER_RES`"]
pub type OTYPER_RES_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ODR_RES"]
    #[inline(always)]
    pub fn odr_res(&self) -> ODR_RES_R {
        ODR_RES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OTYPER_RES"]
    #[inline(always)]
    pub fn otyper_res(&self) -> OTYPER_RES_R {
        OTYPER_RES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
