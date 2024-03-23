#[doc = "Register `IE` reader"]
pub type R = crate::R<IErs>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IErs>;
#[doc = "Field `CWEE` reader - Calibration Watchdog Event Enable"]
pub type CWEE_R = crate::BitReader;
#[doc = "Field `CWEE` writer - Calibration Watchdog Event Enable"]
pub type CWEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSCE` reader - Calibration State Changed Enable"]
pub type CSCE_R = crate::BitReader;
#[doc = "Field `CSCE` writer - Calibration State Changed Enable"]
pub type CSCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Calibration Watchdog Event Enable"]
    #[inline(always)]
    pub fn cwee(&self) -> CWEE_R {
        CWEE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Calibration State Changed Enable"]
    #[inline(always)]
    pub fn csce(&self) -> CSCE_R {
        CSCE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Watchdog Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cwee(&mut self) -> CWEE_W<IErs> {
        CWEE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Calibration State Changed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csce(&mut self) -> CSCE_W<IErs> {
        CSCE_W::new(self, 1)
    }
}
#[doc = "Clock Calibration Unit Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IErs;
impl crate::RegisterSpec for IErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IErs {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IErs {
    const RESET_VALUE: u32 = 0;
}
