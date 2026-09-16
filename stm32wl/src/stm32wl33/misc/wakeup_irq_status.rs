///Register `WAKEUP_IRQ_STATUS` reader
pub type R = crate::R<WAKEUP_IRQ_STATUSrs>;
///Register `WAKEUP_IRQ_STATUS` writer
pub type W = crate::W<WAKEUP_IRQ_STATUSrs>;
///Field `CPU_WAKEUP_F` reader - Set when the interpolated absolute time matches the CPU_WAKEUPTIME while WAKEUP_CTRL.
pub type CPU_WAKEUP_F_R = crate::BitReader;
///Field `CPU_WAKEUP_F` writer - Set when the interpolated absolute time matches the CPU_WAKEUPTIME while WAKEUP_CTRL.
pub type CPU_WAKEUP_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFIP_WAKEUP_F` reader - Set when the interpolated absolute time matches the RFIP_WAKEUPTIME while WAKEUP_CTRL.
pub type RFIP_WAKEUP_F_R = crate::BitReader;
///Field `RFIP_WAKEUP_F` writer - Set when the interpolated absolute time matches the RFIP_WAKEUPTIME while WAKEUP_CTRL.
pub type RFIP_WAKEUP_F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set when the interpolated absolute time matches the CPU_WAKEUPTIME while WAKEUP_CTRL.
    #[inline(always)]
    pub fn cpu_wakeup_f(&self) -> CPU_WAKEUP_F_R {
        CPU_WAKEUP_F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set when the interpolated absolute time matches the RFIP_WAKEUPTIME while WAKEUP_CTRL.
    #[inline(always)]
    pub fn rfip_wakeup_f(&self) -> RFIP_WAKEUP_F_R {
        RFIP_WAKEUP_F_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEUP_IRQ_STATUS")
            .field("cpu_wakeup_f", &self.cpu_wakeup_f())
            .field("rfip_wakeup_f", &self.rfip_wakeup_f())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set when the interpolated absolute time matches the CPU_WAKEUPTIME while WAKEUP_CTRL.
    #[inline(always)]
    pub fn cpu_wakeup_f(&mut self) -> CPU_WAKEUP_F_W<'_, WAKEUP_IRQ_STATUSrs> {
        CPU_WAKEUP_F_W::new(self, 0)
    }
    ///Bit 1 - Set when the interpolated absolute time matches the RFIP_WAKEUPTIME while WAKEUP_CTRL.
    #[inline(always)]
    pub fn rfip_wakeup_f(&mut self) -> RFIP_WAKEUP_F_W<'_, WAKEUP_IRQ_STATUSrs> {
        RFIP_WAKEUP_F_W::new(self, 1)
    }
}
/**WAKEUP_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`wakeup_irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:WAKEUP_IRQ_STATUS)*/
pub struct WAKEUP_IRQ_STATUSrs;
impl crate::RegisterSpec for WAKEUP_IRQ_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`wakeup_irq_status::R`](R) reader structure
impl crate::Readable for WAKEUP_IRQ_STATUSrs {}
///`write(|w| ..)` method takes [`wakeup_irq_status::W`](W) writer structure
impl crate::Writable for WAKEUP_IRQ_STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WAKEUP_IRQ_STATUS to value 0
impl crate::Resettable for WAKEUP_IRQ_STATUSrs {}
