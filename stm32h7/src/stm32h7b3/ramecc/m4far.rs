#[doc = "Reader of register M4FAR"]
pub type R = crate::R<u32, super::M4FAR>;
#[doc = "Reader of field `FDATAH`"]
pub type FDATAH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
