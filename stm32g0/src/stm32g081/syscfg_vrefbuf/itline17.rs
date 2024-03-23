#[doc = "Register `ITLINE17` reader"]
pub type R = crate::R<ITLINE17rs>;
#[doc = "Field `TIM6` reader - TIM6"]
pub type TIM6_R = crate::BitReader;
#[doc = "Field `DAC` reader - DAC"]
pub type DAC_R = crate::BitReader;
#[doc = "Field `LPTIM1` reader - LPTIM1"]
pub type LPTIM1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM6"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPTIM1"]
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "interrupt line 17 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline17::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE17rs;
impl crate::RegisterSpec for ITLINE17rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline17::R`](R) reader structure"]
impl crate::Readable for ITLINE17rs {}
#[doc = "`reset()` method sets ITLINE17 to value 0"]
impl crate::Resettable for ITLINE17rs {
    const RESET_VALUE: u32 = 0;
}
