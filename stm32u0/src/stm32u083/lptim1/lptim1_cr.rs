///Register `LPTIM1_CR` reader
pub type R = crate::R<LPTIM1_CRrs>;
///Register `LPTIM1_CR` writer
pub type W = crate::W<LPTIM1_CRrs>;
///Field `ENABLE` reader - LPTIM enable The ENABLE bit is set and cleared by software.
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - LPTIM enable The ENABLE bit is set and cleared by software.
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `SNGSTRT` reader - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
= 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\]
different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM1_ARR and LPTIM1_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware.*/
pub type SNGSTRT_R = crate::BitReader;
/**Field `SNGSTRT` writer - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
= 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\]
different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM1_ARR and LPTIM1_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware.*/
pub type SNGSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `CNTSTRT` reader - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
= 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\]
different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM1_ARR and LPTIM1_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware.*/
pub type CNTSTRT_R = crate::BitReader;
/**Field `CNTSTRT` writer - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
= 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\]
different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM1_ARR and LPTIM1_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware.*/
pub type CNTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COUNTRST` reader - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM1_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software must consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.
pub type COUNTRST_R = crate::BitReader;
///Field `COUNTRST` writer - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM1_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software must consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.
pub type COUNTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTARE` reader - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM1_CNT register asynchronously resets LPTIM1_CNT register content. This bit can be set only when the LPTIM is enabled.
pub type RSTARE_R = crate::BitReader;
///Field `RSTARE` writer - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM1_CNT register asynchronously resets LPTIM1_CNT register content. This bit can be set only when the LPTIM is enabled.
pub type RSTARE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    /**Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
    = 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\]
    different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM1_ARR and LPTIM1_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware.*/
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    /**Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
    = 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\]
    different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM1_ARR and LPTIM1_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware.*/
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM1_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software must consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM1_CNT register asynchronously resets LPTIM1_CNT register content. This bit can be set only when the LPTIM is enabled.
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM1_CR")
            .field("enable", &self.enable())
            .field("sngstrt", &self.sngstrt())
            .field("cntstrt", &self.cntstrt())
            .field("countrst", &self.countrst())
            .field("rstare", &self.rstare())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software.
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<LPTIM1_CRrs> {
        ENABLE_W::new(self, 0)
    }
    /**Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
    = 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\]
    different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM1_ARR and LPTIM1_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware.*/
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<LPTIM1_CRrs> {
        SNGSTRT_W::new(self, 1)
    }
    /**Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
    = 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\]
    different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM1_ARR and LPTIM1_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware.*/
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<LPTIM1_CRrs> {
        CNTSTRT_W::new(self, 2)
    }
    ///Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM1_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software must consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.
    #[inline(always)]
    pub fn countrst(&mut self) -> COUNTRST_W<LPTIM1_CRrs> {
        COUNTRST_W::new(self, 3)
    }
    ///Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM1_CNT register asynchronously resets LPTIM1_CNT register content. This bit can be set only when the LPTIM is enabled.
    #[inline(always)]
    pub fn rstare(&mut self) -> RSTARE_W<LPTIM1_CRrs> {
        RSTARE_W::new(self, 4)
    }
}
/**LPTIM control register

You can [`read`](crate::Reg::read) this register and get [`lptim1_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM1:LPTIM1_CR)*/
pub struct LPTIM1_CRrs;
impl crate::RegisterSpec for LPTIM1_CRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim1_cr::R`](R) reader structure
impl crate::Readable for LPTIM1_CRrs {}
///`write(|w| ..)` method takes [`lptim1_cr::W`](W) writer structure
impl crate::Writable for LPTIM1_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM1_CR to value 0
impl crate::Resettable for LPTIM1_CRrs {
    const RESET_VALUE: u32 = 0;
}