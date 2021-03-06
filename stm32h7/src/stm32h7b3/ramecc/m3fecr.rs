#[doc = "Reader of register M3FECR"]
pub type R = crate::R<u32, super::M3FECR>;
#[doc = "Reader of field `FDATAL`"]
pub type FDATAL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
