///Register `IORETENRH` reader
pub type R = crate::R<IORETENRHrs>;
///Register `IORETENRH` writer
pub type W = crate::W<IORETENRHrs>;
///Field `EN3` reader - Port H Standby GPIO retention enable Access can be secured by GPIOH SEC3. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN3_R = crate::BitReader;
///Field `EN3` writer - Port H Standby GPIO retention enable Access can be secured by GPIOH SEC3. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Port H Standby GPIO retention enable Access can be secured by GPIOH SEC3. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IORETENRH")
            .field("en3", &self.en3())
            .finish()
    }
}
impl W {
    ///Bit 3 - Port H Standby GPIO retention enable Access can be secured by GPIOH SEC3. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W<'_, IORETENRHrs> {
        EN3_W::new(self, 3)
    }
}
/**PWR port H Standby IO retention enable register

You can [`read`](crate::Reg::read) this register and get [`ioretenrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretenrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETENRH)*/
pub struct IORETENRHrs;
impl crate::RegisterSpec for IORETENRHrs {
    type Ux = u32;
}
///`read()` method returns [`ioretenrh::R`](R) reader structure
impl crate::Readable for IORETENRHrs {}
///`write(|w| ..)` method takes [`ioretenrh::W`](W) writer structure
impl crate::Writable for IORETENRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IORETENRH to value 0
impl crate::Resettable for IORETENRHrs {}
