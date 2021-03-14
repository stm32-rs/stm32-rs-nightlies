#[doc = "Reader of register GPIOI_HWCFGR0"]
pub type R = crate::R<u32, super::GPIOI_HWCFGR0>;
#[doc = "Reader of field `OR_RES`"]
pub type OR_RES_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - OR_RES"]
    #[inline(always)]
    pub fn or_res(&self) -> OR_RES_R {
        OR_RES_R::new((self.bits & 0xffff) as u16)
    }
}
