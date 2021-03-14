#[doc = "Reader of register SMCR"]
pub type R = crate::R<u32, super::SMCR>;
#[doc = "Reader of field `Res`"]
pub type RES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Res."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
