#[doc = "Reader of register ITLINE0"]
pub type R = crate::R<u32, super::ITLINE0>;
#[doc = "Reader of field `WWDG`"]
pub type WWDG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Window watchdog interrupt pending flag"]
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new((self.bits & 0x01) != 0)
    }
}
