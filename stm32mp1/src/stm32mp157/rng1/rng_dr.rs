#[doc = "Reader of register RNG_DR"]
pub type R = crate::R<u32, super::RNG_DR>;
#[doc = "Reader of field `RNDATA`"]
pub type RNDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RNDATA"]
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
