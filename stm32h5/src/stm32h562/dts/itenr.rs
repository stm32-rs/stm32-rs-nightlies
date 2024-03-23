#[doc = "Register `ITENR` reader"]
pub type R = crate::R<ITENRrs>;
#[doc = "Register `ITENR` writer"]
pub type W = crate::W<ITENRrs>;
#[doc = "Field `TS1_ITEEN` reader - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
pub type TS1_ITEEN_R = crate::BitReader;
#[doc = "Field `TS1_ITEEN` writer - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
pub type TS1_ITEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_ITLEN` reader - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
pub type TS1_ITLEN_R = crate::BitReader;
#[doc = "Field `TS1_ITLEN` writer - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
pub type TS1_ITLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_ITHEN` reader - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
pub type TS1_ITHEN_R = crate::BitReader;
#[doc = "Field `TS1_ITHEN` writer - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
pub type TS1_ITHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_AITEEN` reader - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
pub type TS1_AITEEN_R = crate::BitReader;
#[doc = "Field `TS1_AITEEN` writer - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
pub type TS1_AITEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_AITLEN` reader - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)"]
pub type TS1_AITLEN_R = crate::BitReader;
#[doc = "Field `TS1_AITLEN` writer - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)"]
pub type TS1_AITLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_AITHEN` reader - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)"]
pub type TS1_AITHEN_R = crate::BitReader;
#[doc = "Field `TS1_AITHEN` writer - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)"]
pub type TS1_AITHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
    #[inline(always)]
    pub fn ts1_iteen(&self) -> TS1_ITEEN_R {
        TS1_ITEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
    #[inline(always)]
    pub fn ts1_itlen(&self) -> TS1_ITLEN_R {
        TS1_ITLEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
    #[inline(always)]
    pub fn ts1_ithen(&self) -> TS1_ITHEN_R {
        TS1_ITHEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
    #[inline(always)]
    pub fn ts1_aiteen(&self) -> TS1_AITEEN_R {
        TS1_AITEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)"]
    #[inline(always)]
    pub fn ts1_aitlen(&self) -> TS1_AITLEN_R {
        TS1_AITLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)"]
    #[inline(always)]
    pub fn ts1_aithen(&self) -> TS1_AITHEN_R {
        TS1_AITHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_iteen(&mut self) -> TS1_ITEEN_W<ITENRrs> {
        TS1_ITEEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_itlen(&mut self) -> TS1_ITLEN_W<ITENRrs> {
        TS1_ITLEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_ithen(&mut self) -> TS1_ITHEN_W<ITENRrs> {
        TS1_ITHEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_aiteen(&mut self) -> TS1_AITEEN_W<ITENRrs> {
        TS1_AITEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_aitlen(&mut self) -> TS1_AITLEN_W<ITENRrs> {
        TS1_AITLEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_aithen(&mut self) -> TS1_AITHEN_W<ITENRrs> {
        TS1_AITHEN_W::new(self, 6)
    }
}
#[doc = "Temperature sensor interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITENRrs;
impl crate::RegisterSpec for ITENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itenr::R`](R) reader structure"]
impl crate::Readable for ITENRrs {}
#[doc = "`write(|w| ..)` method takes [`itenr::W`](W) writer structure"]
impl crate::Writable for ITENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITENR to value 0"]
impl crate::Resettable for ITENRrs {
    const RESET_VALUE: u32 = 0;
}
