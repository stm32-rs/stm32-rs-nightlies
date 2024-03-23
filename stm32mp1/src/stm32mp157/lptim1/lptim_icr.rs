#[doc = "Register `LPTIM_ICR` writer"]
pub type W = crate::W<LPTIM_ICRrs>;
#[doc = "Field `CMPMCF` writer - CMPMCF"]
pub type CMPMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` writer - ARRMCF"]
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - EXTTRIGCF"]
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOKCF` writer - CMPOKCF"]
pub type CMPOKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKCF` writer - ARROKCF"]
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCF` writer - UPCF"]
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNCF` writer - DOWNCF"]
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CMPMCF"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<LPTIM_ICRrs> {
        CMPMCF_W::new(self, 0)
    }
    #[doc = "Bit 1 - ARRMCF"]
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ARRMCF_W<LPTIM_ICRrs> {
        ARRMCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - EXTTRIGCF"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<LPTIM_ICRrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - CMPOKCF"]
    #[inline(always)]
    #[must_use]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<LPTIM_ICRrs> {
        CMPOKCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - ARROKCF"]
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ARROKCF_W<LPTIM_ICRrs> {
        ARROKCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - UPCF"]
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UPCF_W<LPTIM_ICRrs> {
        UPCF_W::new(self, 5)
    }
    #[doc = "Bit 6 - DOWNCF"]
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DOWNCF_W<LPTIM_ICRrs> {
        DOWNCF_W::new(self, 6)
    }
}
#[doc = "LPTIM interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_ICRrs;
impl crate::RegisterSpec for LPTIM_ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lptim_icr::W`](W) writer structure"]
impl crate::Writable for LPTIM_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM_ICR to value 0"]
impl crate::Resettable for LPTIM_ICRrs {
    const RESET_VALUE: u32 = 0;
}
