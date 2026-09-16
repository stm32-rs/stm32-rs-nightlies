///Register `IORETRH` reader
pub type R = crate::R<IORETRHrs>;
///Register `IORETRH` writer
pub type W = crate::W<IORETRHrs>;
///Field `RET3` reader - Port H Standby GPIO retention active Access can be secured by GPIOH SEC3. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type RET3_R = crate::BitReader;
///Field `RET3` writer - Port H Standby GPIO retention active Access can be secured by GPIOH SEC3. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type RET3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Port H Standby GPIO retention active Access can be secured by GPIOH SEC3. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn ret3(&self) -> RET3_R {
        RET3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IORETRH")
            .field("ret3", &self.ret3())
            .finish()
    }
}
impl W {
    ///Bit 3 - Port H Standby GPIO retention active Access can be secured by GPIOH SEC3. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn ret3(&mut self) -> RET3_W<'_, IORETRHrs> {
        RET3_W::new(self, 3)
    }
}
/**PWR port H Standby IO retention status register

You can [`read`](crate::Reg::read) this register and get [`ioretrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#PWR:IORETRH)*/
pub struct IORETRHrs;
impl crate::RegisterSpec for IORETRHrs {
    type Ux = u32;
}
///`read()` method returns [`ioretrh::R`](R) reader structure
impl crate::Readable for IORETRHrs {}
///`write(|w| ..)` method takes [`ioretrh::W`](W) writer structure
impl crate::Writable for IORETRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IORETRH to value 0
impl crate::Resettable for IORETRHrs {}
