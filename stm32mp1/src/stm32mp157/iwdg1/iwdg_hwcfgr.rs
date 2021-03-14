#[doc = "Reader of register IWDG_HWCFGR"]
pub type R = crate::R<u32, super::IWDG_HWCFGR>;
#[doc = "Reader of field `WINDOW`"]
pub type WINDOW_R = crate::R<u8, u8>;
#[doc = "Reader of field `PR_DEFAULT`"]
pub type PR_DEFAULT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - WINDOW"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PR_DEFAULT"]
    #[inline(always)]
    pub fn pr_default(&self) -> PR_DEFAULT_R {
        PR_DEFAULT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
