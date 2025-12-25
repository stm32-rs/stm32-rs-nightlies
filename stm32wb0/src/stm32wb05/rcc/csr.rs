///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `RMVF` writer - Remove reset flag Set by software to clear the value of the reset flags. It auto clears by HW after clearing reason flags
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PADRSTF` reader - SYSTEM reset flag Reset by software by writing the RMVF bit. Set by hardware when a reset from pad occurs.
pub type PADRSTF_R = crate::BitReader;
///Field `PORRSTF` reader - POWER reset flag Reset by software by writing the RMVF bit. Set by hardware when a power reset occurs from LPMURESET block.
pub type PORRSTF_R = crate::BitReader;
///Field `SFTRSTF` reader - Software reset flag Reset by software by writing the RMVF bit. Set by hardware when a software reset occurs.
pub type SFTRSTF_R = crate::BitReader;
///Field `WDGRSTF` reader - Watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a watchdog reset from V33 domain occurs.
pub type WDGRSTF_R = crate::BitReader;
///Field `LOCKUPRSTF` reader - LOCK UP reset flag from CM0 Reset by software by writing the RMVF bit. Set by hardware from unrecoverable exception CPU. It reset V12i domain, FLASH controller and peripherals.
pub type LOCKUPRSTF_R = crate::BitReader;
impl R {
    ///Bit 26 - SYSTEM reset flag Reset by software by writing the RMVF bit. Set by hardware when a reset from pad occurs.
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - POWER reset flag Reset by software by writing the RMVF bit. Set by hardware when a power reset occurs from LPMURESET block.
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag Reset by software by writing the RMVF bit. Set by hardware when a software reset occurs.
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a watchdog reset from V33 domain occurs.
    #[inline(always)]
    pub fn wdgrstf(&self) -> WDGRSTF_R {
        WDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - LOCK UP reset flag from CM0 Reset by software by writing the RMVF bit. Set by hardware from unrecoverable exception CPU. It reset V12i domain, FLASH controller and peripherals.
    #[inline(always)]
    pub fn lockuprstf(&self) -> LOCKUPRSTF_R {
        LOCKUPRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("padrstf", &self.padrstf())
            .field("porrstf", &self.porrstf())
            .field("sftrstf", &self.sftrstf())
            .field("wdgrstf", &self.wdgrstf())
            .field("lockuprstf", &self.lockuprstf())
            .finish()
    }
}
impl W {
    ///Bit 23 - Remove reset flag Set by software to clear the value of the reset flags. It auto clears by HW after clearing reason flags
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, CSRrs> {
        RMVF_W::new(self, 23)
    }
}
/**CSR register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RCC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0x0c00_0000
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
