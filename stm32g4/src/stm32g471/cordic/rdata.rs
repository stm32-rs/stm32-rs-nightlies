#[doc = "Reader of register RDATA"]
pub type R = crate::R<u32, super::RDATA>;
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RES"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
