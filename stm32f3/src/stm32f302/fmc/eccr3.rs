#[doc = "Reader of register ECCR3"]
pub type R = crate::R<u32, super::ECCR3>;
#[doc = "Reader of field `ECCx`"]
pub type ECCX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECCx"]
    #[inline(always)]
    pub fn eccx(&self) -> ECCX_R {
        ECCX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
