#[doc = "Reader of register DFSDM_CH4WDATR"]
pub type R = crate::R<u32, super::DFSDM_CH4WDATR>;
#[doc = "Reader of field `WDATA`"]
pub type WDATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
