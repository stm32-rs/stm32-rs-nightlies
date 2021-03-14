#[doc = "Reader of register GPIOA_HWCFGR6"]
pub type R = crate::R<u32, super::GPIOA_HWCFGR6>;
#[doc = "Reader of field `MODER_RES`"]
pub type MODER_RES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - MODER_RES"]
    #[inline(always)]
    pub fn moder_res(&self) -> MODER_RES_R {
        MODER_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
