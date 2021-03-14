#[doc = "Reader of register HASH_HWCFGR"]
pub type R = crate::R<u32, super::HASH_HWCFGR>;
#[doc = "Reader of field `CFG1`"]
pub type CFG1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
}
