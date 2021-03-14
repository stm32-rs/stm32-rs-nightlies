#[doc = "Reader of register DDRPERFM_CNT0"]
pub type R = crate::R<u32, super::DDRPERFM_CNT0>;
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
