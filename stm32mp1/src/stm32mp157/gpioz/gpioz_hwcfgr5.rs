#[doc = "Reader of register GPIOZ_HWCFGR5"]
pub type R = crate::R<u32, super::GPIOZ_HWCFGR5>;
#[doc = "Reader of field `PUPDR_RES`"]
pub type PUPDR_RES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PUPDR_RES"]
    #[inline(always)]
    pub fn pupdr_res(&self) -> PUPDR_RES_R {
        PUPDR_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
