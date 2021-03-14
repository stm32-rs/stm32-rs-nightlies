#[doc = "Reader of register DR"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Reader of field `RDATA`"]
pub type RDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group regular conversion data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
