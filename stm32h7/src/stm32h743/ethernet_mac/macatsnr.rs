#[doc = "Reader of register MACATSNR"]
pub type R = crate::R<u32, super::MACATSNR>;
#[doc = "Reader of field `AUXTSLO`"]
pub type AUXTSLO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:30 - Auxiliary Timestamp"]
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
