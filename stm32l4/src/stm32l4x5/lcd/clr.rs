///Register `CLR` writer
pub type W = crate::W<CLRrs>;
///Field `SOFC` writer - Start of frame flag clear
pub type SOFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDDC` writer - Update display done clear
pub type UDDC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CLRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - Start of frame flag clear
    #[inline(always)]
    pub fn sofc(&mut self) -> SOFC_W<'_, CLRrs> {
        SOFC_W::new(self, 1)
    }
    ///Bit 3 - Update display done clear
    #[inline(always)]
    pub fn uddc(&mut self) -> UDDC_W<'_, CLRrs> {
        UDDC_W::new(self, 3)
    }
}
/**clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#LCD:CLR)*/
pub struct CLRrs;
impl crate::RegisterSpec for CLRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`clr::W`](W) writer structure
impl crate::Writable for CLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLR to value 0
impl crate::Resettable for CLRrs {}
