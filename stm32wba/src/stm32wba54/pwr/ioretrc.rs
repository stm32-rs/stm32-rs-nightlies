///Register `IORETRC` reader
pub type R = crate::R<IORETRCrs>;
///Register `IORETRC` writer
pub type W = crate::W<IORETRCrs>;
///Field `RET13` reader - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type RET13_R = crate::BitReader;
///Field `RET13` writer - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type RET13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RET14` reader - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type RET14_R = crate::BitReader;
///Field `RET14` writer - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type RET14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RET15` reader - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type RET15_R = crate::BitReader;
///Field `RET15` writer - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type RET15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn ret13(&self) -> RET13_R {
        RET13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn ret14(&self) -> RET14_R {
        RET14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn ret15(&self) -> RET15_R {
        RET15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IORETRC")
            .field("ret13", &self.ret13())
            .field("ret14", &self.ret14())
            .field("ret15", &self.ret15())
            .finish()
    }
}
impl W {
    ///Bit 13 - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn ret13(&mut self) -> RET13_W<'_, IORETRCrs> {
        RET13_W::new(self, 13)
    }
    ///Bit 14 - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn ret14(&mut self) -> RET14_W<'_, IORETRCrs> {
        RET14_W::new(self, 14)
    }
    ///Bit 15 - Port C Standby GPIO retention active Access can be secured by GPIOC SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn ret15(&mut self) -> RET15_W<'_, IORETRCrs> {
        RET15_W::new(self, 15)
    }
}
/**PWR port C Standby IO retention status register

You can [`read`](crate::Reg::read) this register and get [`ioretrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#PWR:IORETRC)*/
pub struct IORETRCrs;
impl crate::RegisterSpec for IORETRCrs {
    type Ux = u32;
}
///`read()` method returns [`ioretrc::R`](R) reader structure
impl crate::Readable for IORETRCrs {}
///`write(|w| ..)` method takes [`ioretrc::W`](W) writer structure
impl crate::Writable for IORETRCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IORETRC to value 0
impl crate::Resettable for IORETRCrs {}
