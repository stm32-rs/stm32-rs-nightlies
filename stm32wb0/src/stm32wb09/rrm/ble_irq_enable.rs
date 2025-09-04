///Register `BLE_IRQ_ENABLE` reader
pub type R = crate::R<BLE_IRQ_ENABLErs>;
///Register `BLE_IRQ_ENABLE` writer
pub type W = crate::W<BLE_IRQ_ENABLErs>;
///Field `PORT_GRANT` reader - IP_BLE Port grant interrupt enable
pub type PORT_GRANT_R = crate::BitReader;
///Field `PORT_GRANT` writer - IP_BLE Port grant interrupt enable
pub type PORT_GRANT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_RELEASE` reader - IP_BLE Port release interrupt enable
pub type PORT_RELEASE_R = crate::BitReader;
///Field `PORT_RELEASE` writer - IP_BLE Port release interrupt enable
pub type PORT_RELEASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_CMD_START` reader - IP_BLE Port command start interrup enable
pub type PORT_CMD_START_R = crate::BitReader;
///Field `PORT_CMD_START` writer - IP_BLE Port command start interrup enable
pub type PORT_CMD_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_CMD_END` reader - IP_BLE Port command end interrup enable
pub type PORT_CMD_END_R = crate::BitReader;
///Field `PORT_CMD_END` writer - IP_BLE Port command end interrup enable
pub type PORT_CMD_END_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IP_BLE Port grant interrupt enable
    #[inline(always)]
    pub fn port_grant(&self) -> PORT_GRANT_R {
        PORT_GRANT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IP_BLE Port release interrupt enable
    #[inline(always)]
    pub fn port_release(&self) -> PORT_RELEASE_R {
        PORT_RELEASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - IP_BLE Port command start interrup enable
    #[inline(always)]
    pub fn port_cmd_start(&self) -> PORT_CMD_START_R {
        PORT_CMD_START_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IP_BLE Port command end interrup enable
    #[inline(always)]
    pub fn port_cmd_end(&self) -> PORT_CMD_END_R {
        PORT_CMD_END_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLE_IRQ_ENABLE")
            .field("port_grant", &self.port_grant())
            .field("port_release", &self.port_release())
            .field("port_cmd_start", &self.port_cmd_start())
            .field("port_cmd_end", &self.port_cmd_end())
            .finish()
    }
}
impl W {
    ///Bit 0 - IP_BLE Port grant interrupt enable
    #[inline(always)]
    pub fn port_grant(&mut self) -> PORT_GRANT_W<BLE_IRQ_ENABLErs> {
        PORT_GRANT_W::new(self, 0)
    }
    ///Bit 1 - IP_BLE Port release interrupt enable
    #[inline(always)]
    pub fn port_release(&mut self) -> PORT_RELEASE_W<BLE_IRQ_ENABLErs> {
        PORT_RELEASE_W::new(self, 1)
    }
    ///Bit 3 - IP_BLE Port command start interrup enable
    #[inline(always)]
    pub fn port_cmd_start(&mut self) -> PORT_CMD_START_W<BLE_IRQ_ENABLErs> {
        PORT_CMD_START_W::new(self, 3)
    }
    ///Bit 4 - IP_BLE Port command end interrup enable
    #[inline(always)]
    pub fn port_cmd_end(&mut self) -> PORT_CMD_END_W<BLE_IRQ_ENABLErs> {
        PORT_CMD_END_W::new(self, 4)
    }
}
/**BLE_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`ble_irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:BLE_IRQ_ENABLE)*/
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
