///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `CSSF` writer - Clear Stop and Standby flags Access can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the STOPF and SBF flags.
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPF` reader - Stop flag This bit is set by hardware when the device enters a Stop or Standby mode at the same time as the sysclk has been set by hardware to select HSI16. It's cleared by software by writing 1 to the CSSF bit and by hardware when SBF is set.
pub type STOPF_R = crate::BitReader;
///Field `SBF` reader - Standby flag This bit is set by hardware when the device enters the Standby mode and the CPU restart from its reset vector. It's cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.
pub type SBF_R = crate::BitReader;
impl R {
    ///Bit 1 - Stop flag This bit is set by hardware when the device enters a Stop or Standby mode at the same time as the sysclk has been set by hardware to select HSI16. It's cleared by software by writing 1 to the CSSF bit and by hardware when SBF is set.
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Standby flag This bit is set by hardware when the device enters the Standby mode and the CPU restart from its reset vector. It's cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Stop and Standby flags Access can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the STOPF and SBF flags.
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<'_, SRrs> {
        CSSF_W::new(self, 0)
    }
}
/**PWR status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PWR:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
