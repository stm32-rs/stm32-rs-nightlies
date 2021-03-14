#[doc = "Reader of register DFSDM_HWCFGR"]
pub type R = crate::R<u32, super::DFSDM_HWCFGR>;
#[doc = "Reader of field `NBT`"]
pub type NBT_R = crate::R<u8, u8>;
#[doc = "Reader of field `NBF`"]
pub type NBF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - NBT"]
    #[inline(always)]
    pub fn nbt(&self) -> NBT_R {
        NBT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NBF"]
    #[inline(always)]
    pub fn nbf(&self) -> NBF_R {
        NBF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
