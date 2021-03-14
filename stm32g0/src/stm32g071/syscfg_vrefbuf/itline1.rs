#[doc = "Reader of register ITLINE1"]
pub type R = crate::R<u32, super::ITLINE1>;
#[doc = "Reader of field `PVDOUT`"]
pub type PVDOUT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PVD supply monitoring interrupt request pending (EXTI line 16)."]
    #[inline(always)]
    pub fn pvdout(&self) -> PVDOUT_R {
        PVDOUT_R::new((self.bits & 0x01) != 0)
    }
}
