#[doc = "Reader of register GPIOB_HWCFGR1"]
pub type R = crate::R<u32, super::GPIOB_HWCFGR1>;
#[doc = "Reader of field `AFRH_RES`"]
pub type AFRH_RES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AFRH_RES"]
    #[inline(always)]
    pub fn afrh_res(&self) -> AFRH_RES_R {
        AFRH_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
