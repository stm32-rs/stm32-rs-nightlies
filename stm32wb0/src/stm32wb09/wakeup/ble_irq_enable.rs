///Register `BLE_IRQ_ENABLE` reader
pub type R = crate::R<BLE_IRQ_ENABLErs>;
///Register `BLE_IRQ_ENABLE` writer
pub type W = crate::W<BLE_IRQ_ENABLErs>;
///Field `WAKEUP_IT` reader - IP_BLE wakeup interrupt enable:
pub type WAKEUP_IT_R = crate::BitReader;
///Field `WAKEUP_IT` writer - IP_BLE wakeup interrupt enable:
pub type WAKEUP_IT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IP_BLE wakeup interrupt enable:
    #[inline(always)]
    pub fn wakeup_it(&self) -> WAKEUP_IT_R {
        WAKEUP_IT_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLE_IRQ_ENABLE")
            .field("wakeup_it", &self.wakeup_it())
            .finish()
    }
}
impl W {
    ///Bit 0 - IP_BLE wakeup interrupt enable:
    #[inline(always)]
    pub fn wakeup_it(&mut self) -> WAKEUP_IT_W<'_, BLE_IRQ_ENABLErs> {
        WAKEUP_IT_W::new(self, 0)
    }
}
/**WAKEUP_BLE_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`ble_irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#WAKEUP:BLE_IRQ_ENABLE)*/
pub struct BLE_IRQ_ENABLErs;
impl crate::RegisterSpec for BLE_IRQ_ENABLErs {
    type Ux = u32;
}
///`read()` method returns [`ble_irq_enable::R`](R) reader structure
impl crate::Readable for BLE_IRQ_ENABLErs {}
///`write(|w| ..)` method takes [`ble_irq_enable::W`](W) writer structure
impl crate::Writable for BLE_IRQ_ENABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BLE_IRQ_ENABLE to value 0
impl crate::Resettable for BLE_IRQ_ENABLErs {}
