///Register `BLE_IRQ_STATUS` reader
pub type R = crate::R<BLE_IRQ_STATUSrs>;
///Register `BLE_IRQ_STATUS` writer
pub type W = crate::W<BLE_IRQ_STATUSrs>;
///Field `PORT_GRANT` reader - IP_BLE hardware port granted interrupt status:
pub type PORT_GRANT_R = crate::BitReader;
///Field `PORT_GRANT` writer - IP_BLE hardware port granted interrupt status:
pub type PORT_GRANT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_RELEASE` reader - IP_BLE hardware port released interrupt status.
pub type PORT_RELEASE_R = crate::BitReader;
///Field `PORT_RELEASE` writer - IP_BLE hardware port released interrupt status.
pub type PORT_RELEASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_START` reader - IP_BLE hardware port command start interrupt status.
pub type CMD_START_R = crate::BitReader;
///Field `CMD_START` writer - IP_BLE hardware port command start interrupt status.
pub type CMD_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_END` reader - IP_BLE hardware port command end interrupt status.
pub type CMD_END_R = crate::BitReader;
///Field `CMD_END` writer - IP_BLE hardware port command end interrupt status.
pub type CMD_END_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IP_BLE hardware port granted interrupt status:
    #[inline(always)]
    pub fn port_grant(&self) -> PORT_GRANT_R {
        PORT_GRANT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IP_BLE hardware port released interrupt status.
    #[inline(always)]
    pub fn port_release(&self) -> PORT_RELEASE_R {
        PORT_RELEASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - IP_BLE hardware port command start interrupt status.
    #[inline(always)]
    pub fn cmd_start(&self) -> CMD_START_R {
        CMD_START_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IP_BLE hardware port command end interrupt status.
    #[inline(always)]
    pub fn cmd_end(&self) -> CMD_END_R {
        CMD_END_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLE_IRQ_STATUS")
            .field("port_grant", &self.port_grant())
            .field("port_release", &self.port_release())
            .field("cmd_start", &self.cmd_start())
            .field("cmd_end", &self.cmd_end())
            .finish()
    }
}
impl W {
    ///Bit 0 - IP_BLE hardware port granted interrupt status:
    #[inline(always)]
    pub fn port_grant(&mut self) -> PORT_GRANT_W<'_, BLE_IRQ_STATUSrs> {
        PORT_GRANT_W::new(self, 0)
    }
    ///Bit 1 - IP_BLE hardware port released interrupt status.
    #[inline(always)]
    pub fn port_release(&mut self) -> PORT_RELEASE_W<'_, BLE_IRQ_STATUSrs> {
        PORT_RELEASE_W::new(self, 1)
    }
    ///Bit 3 - IP_BLE hardware port command start interrupt status.
    #[inline(always)]
    pub fn cmd_start(&mut self) -> CMD_START_W<'_, BLE_IRQ_STATUSrs> {
        CMD_START_W::new(self, 3)
    }
    ///Bit 4 - IP_BLE hardware port command end interrupt status.
    #[inline(always)]
    pub fn cmd_end(&mut self) -> CMD_END_W<'_, BLE_IRQ_STATUSrs> {
        CMD_END_W::new(self, 4)
    }
}
/**BLE_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`ble_irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RRM:BLE_IRQ_STATUS)*/
pub struct BLE_IRQ_STATUSrs;
impl crate::RegisterSpec for BLE_IRQ_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`ble_irq_status::R`](R) reader structure
impl crate::Readable for BLE_IRQ_STATUSrs {}
///`write(|w| ..)` method takes [`ble_irq_status::W`](W) writer structure
impl crate::Writable for BLE_IRQ_STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BLE_IRQ_STATUS to value 0
impl crate::Resettable for BLE_IRQ_STATUSrs {}
