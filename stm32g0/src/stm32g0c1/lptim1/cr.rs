#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `ENABLE` reader - LPTIM enable The ENABLE bit is set and cleared by software."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - LPTIM enable The ENABLE bit is set and cleared by software."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNGSTRT` reader - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= '00â\u{80}\u{99}), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than '00â\u{80}\u{99}), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware."]
pub type SNGSTRT_R = crate::BitReader;
#[doc = "Field `SNGSTRT` writer - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= '00â\u{80}\u{99}), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than '00â\u{80}\u{99}), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware."]
pub type SNGSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTRT` reader - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= '00â\u{80}\u{99}), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than '00â\u{80}\u{99}), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware."]
pub type CNTSTRT_R = crate::BitReader;
#[doc = "Field `CNTSTRT` writer - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= '00â\u{80}\u{99}), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than '00â\u{80}\u{99}), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware."]
pub type CNTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTRST` reader - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
pub type COUNTRST_R = crate::BitReader;
#[doc = "Field `COUNTRST` writer - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
pub type COUNTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTARE` reader - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content."]
pub type RSTARE_R = crate::BitReader;
#[doc = "Field `RSTARE` writer - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content."]
pub type RSTARE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= '00â\u{80}\u{99}), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than '00â\u{80}\u{99}), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware."]
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= '00â\u{80}\u{99}), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than '00â\u{80}\u{99}), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware."]
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content."]
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CRrs> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= '00â\u{80}\u{99}), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than '00â\u{80}\u{99}), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<CRrs> {
        SNGSTRT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= '00â\u{80}\u{99}), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than '00â\u{80}\u{99}), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<CRrs> {
        CNTSTRT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn countrst(&mut self) -> COUNTRST_W<CRrs> {
        COUNTRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content."]
    #[inline(always)]
    #[must_use]
    pub fn rstare(&mut self) -> RSTARE_W<CRrs> {
        RSTARE_W::new(self, 4)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
