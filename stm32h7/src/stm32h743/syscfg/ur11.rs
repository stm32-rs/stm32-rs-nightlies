#[doc = "Reader of register UR11"]
pub type R = crate::R<u32, super::UR11>;
#[doc = "Reader of field `SA_END_2`"]
pub type SA_END_2_R = crate::R<u16, u16>;
#[doc = "Reader of field `IWDG1M`"]
pub type IWDG1M_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:11 - Secured area end address for bank 2"]
    #[inline(always)]
    pub fn sa_end_2(&self) -> SA_END_2_R {
        SA_END_2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Independent Watchdog 1 mode"]
    #[inline(always)]
    pub fn iwdg1m(&self) -> IWDG1M_R {
        IWDG1M_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
