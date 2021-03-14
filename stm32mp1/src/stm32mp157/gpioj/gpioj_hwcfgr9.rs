#[doc = "Reader of register GPIOJ_HWCFGR9"]
pub type R = crate::R<u32, super::GPIOJ_HWCFGR9>;
#[doc = "Reader of field `EN_IO`"]
pub type EN_IO_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - EN_IO"]
    #[inline(always)]
    pub fn en_io(&self) -> EN_IO_R {
        EN_IO_R::new((self.bits & 0xffff) as u16)
    }
}
