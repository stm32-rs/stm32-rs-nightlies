#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `CMPMCF` writer - Compare match clear flag Writing 1 to this bit clears the CMP flag in the LPTIM_ISR register"]
pub type CMPMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` writer - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOKCF` writer - Compare register update OK clear flag Writing 1 to this bit clears the CMPOK flag in the LPTIM_ISR register"]
pub type CMPOKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKCF` writer - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register"]
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCF` writer - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNCF` writer - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Compare match clear flag Writing 1 to this bit clears the CMP flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<ICRrs> {
        CMPMCF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ARRMCF_W<ICRrs> {
        ARRMCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<ICRrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register update OK clear flag Writing 1 to this bit clears the CMPOK flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<ICRrs> {
        CMPOKCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ARROKCF_W<ICRrs> {
        ARROKCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UPCF_W<ICRrs> {
        UPCF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DOWNCF_W<ICRrs> {
        DOWNCF_W::new(self, 6)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
