#[doc = "Reader of register GPIOD_HWCFGR2"]
pub type R = crate::R<u32, super::GPIOD_HWCFGR2>;
#[doc = "Reader of field `AFRL_RES`"]
pub type AFRL_RES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AFRL_RES"]
    #[inline(always)]
    pub fn afrl_res(&self) -> AFRL_RES_R {
        AFRL_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
