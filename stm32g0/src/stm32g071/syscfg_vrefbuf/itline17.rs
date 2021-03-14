#[doc = "Reader of register ITLINE17"]
pub type R = crate::R<u32, super::ITLINE17>;
#[doc = "Reader of field `TIM6`"]
pub type TIM6_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAC`"]
pub type DAC_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPTIM1`"]
pub type LPTIM1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM6"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPTIM1"]
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
