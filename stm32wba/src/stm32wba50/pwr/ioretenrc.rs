///Register `IORETENRC` reader
pub type R = crate::R<IORETENRCrs>;
///Register `IORETENRC` writer
pub type W = crate::W<IORETENRCrs>;
///Field `EN13` reader - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN13_R = crate::BitReader;
///Field `EN13` writer - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN14` reader - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN14_R = crate::BitReader;
///Field `EN14` writer - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN15` reader - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN15_R = crate::BitReader;
///Field `EN15` writer - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en13(&self) -> EN13_R {
        EN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en14(&self) -> EN14_R {
        EN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en15(&self) -> EN15_R {
        EN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IORETENRC")
            .field("en13", &self.en13())
            .field("en14", &self.en14())
            .field("en15", &self.en15())
            .finish()
    }
}
impl W {
    ///Bit 13 - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en13(&mut self) -> EN13_W<'_, IORETENRCrs> {
        EN13_W::new(self, 13)
    }
    ///Bit 14 - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en14(&mut self) -> EN14_W<'_, IORETENRCrs> {
        EN14_W::new(self, 14)
    }
    ///Bit 15 - Port C Standby GPIO retention enable Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en15(&mut self) -> EN15_W<'_, IORETENRCrs> {
        EN15_W::new(self, 15)
    }
}
/**PWR port C Standby IO retention enable register

You can [`read`](crate::Reg::read) this register and get [`ioretenrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretenrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETENRC)*/
pub struct IORETENRCrs;
impl crate::RegisterSpec for IORETENRCrs {
    type Ux = u32;
}
///`read()` method returns [`ioretenrc::R`](R) reader structure
impl crate::Readable for IORETENRCrs {}
///`write(|w| ..)` method takes [`ioretenrc::W`](W) writer structure
impl crate::Writable for IORETENRCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IORETENRC to value 0
impl crate::Resettable for IORETENRCrs {}
