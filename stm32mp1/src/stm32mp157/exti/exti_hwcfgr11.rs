#[doc = "Reader of register EXTI_HWCFGR11"]
pub type R = crate::R<u32, super::EXTI_HWCFGR11>;
#[doc = "Reader of field `TZ`"]
pub type TZ_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TZ"]
    #[inline(always)]
    pub fn tz(&self) -> TZ_R {
        TZ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
