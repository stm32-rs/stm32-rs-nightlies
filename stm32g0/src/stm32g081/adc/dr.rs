#[doc = "Reader of register DR"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Reader of field `regularDATA`"]
pub type REGULARDATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC group regular conversion data"]
    #[inline(always)]
    pub fn regular_data(&self) -> REGULARDATA_R {
        REGULARDATA_R::new((self.bits & 0xffff) as u16)
    }
}
