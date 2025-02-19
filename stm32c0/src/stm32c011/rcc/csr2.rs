///Register `CSR2` reader
pub type R = crate::R<CSR2rs>;
///Register `CSR2` writer
pub type W = crate::W<CSR2rs>;
///Field `LSION` reader - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
pub type LSION_R = crate::BitReader;
///Field `LSION` writer - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.
pub type LSIRDY_R = crate::BitReader;
///Field `RMVF` reader - Remove reset flags Set by software to clear the reset flags.
pub type RMVF_R = crate::BitReader;
///Field `RMVF` writer - Remove reset flags Set by software to clear the reset flags.
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBLRSTF` reader - Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit.
pub type OBLRSTF_R = crate::BitReader;
///Field `PINRSTF` reader - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit.
pub type PINRSTF_R = crate::BitReader;
///Field `PWRRSTF` reader - BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit.
pub type PWRRSTF_R = crate::BitReader;
///Field `SFTRSTF` reader - Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit.
pub type SFTRSTF_R = crate::BitReader;
///Field `IWDGRSTF` reader - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit.
pub type IWDGRSTF_R = crate::BitReader;
///Field `WWDGRSTF` reader - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit.
pub type WWDGRSTF_R = crate::BitReader;
///Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, or Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, or nRST_STDBY or nRST_SHDW option bits are cleared.
pub type LPWRRSTF_R = crate::BitReader;
impl R {
    ///Bit 0 - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 23 - Remove reset flags Set by software to clear the reset flags.
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn pwrrstf(&self) -> PWRRSTF_R {
        PWRRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, or Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, or nRST_STDBY or nRST_SHDW option bits are cleared.
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR2")
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .field("rmvf", &self.rmvf())
            .field("oblrstf", &self.oblrstf())
            .field("pinrstf", &self.pinrstf())
            .field("pwrrstf", &self.pwrrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<CSR2rs> {
        LSION_W::new(self, 0)
    }
    ///Bit 23 - Remove reset flags Set by software to clear the reset flags.
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<CSR2rs> {
        RMVF_W::new(self, 23)
    }
}
/**RCC control/status register 2

You can [`read`](crate::Reg::read) this register and get [`csr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CSR2)*/
pub struct CSR2rs;
impl crate::RegisterSpec for CSR2rs {
    type Ux = u32;
}
///`read()` method returns [`csr2::R`](R) reader structure
impl crate::Readable for CSR2rs {}
///`write(|w| ..)` method takes [`csr2::W`](W) writer structure
impl crate::Writable for CSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSR2 to value 0
impl crate::Resettable for CSR2rs {
    const RESET_VALUE: u32 = 0;
}
