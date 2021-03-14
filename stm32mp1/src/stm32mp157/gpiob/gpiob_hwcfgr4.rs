#[doc = "Reader of register GPIOB_HWCFGR4"]
pub type R = crate::R<u32, super::GPIOB_HWCFGR4>;
#[doc = "Reader of field `OSPEED_RES`"]
pub type OSPEED_RES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - OSPEED_RES"]
    #[inline(always)]
    pub fn ospeed_res(&self) -> OSPEED_RES_R {
        OSPEED_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
