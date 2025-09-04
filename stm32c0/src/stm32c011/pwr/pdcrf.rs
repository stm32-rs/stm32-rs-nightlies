///Register `PDCRF` reader
pub type R = crate::R<PDCRFrs>;
///Register `PDCRF` writer
pub type W = crate::W<PDCRFrs>;
///Field `PD2` reader - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\[i\] I/O. On STM32C011xx, only PD2 is available.
pub type PD2_R = crate::BitReader;
///Field `PD2` writer - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\[i\] I/O. On STM32C011xx, only PD2 is available.
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\[i\] I/O. On STM32C011xx, only PD2 is available.
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRF").field("pd2", &self.pd2()).finish()
    }
}
impl W {
    ///Bit 2 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\[i\] I/O. On STM32C011xx, only PD2 is available.
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<PDCRFrs> {
        PD2_W::new(self, 2)
    }
}
/**PWR Port F pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#PWR:PDCRF)*/
pub struct PDCRFrs;
impl crate::RegisterSpec for PDCRFrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrf::R`](R) reader structure
impl crate::Readable for PDCRFrs {}
///`write(|w| ..)` method takes [`pdcrf::W`](W) writer structure
impl crate::Writable for PDCRFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRF to value 0
impl crate::Resettable for PDCRFrs {}
