///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
///Field `LATENCY` reader - Latency These bits represent the ratio between the AHB hclk1 clock period and the Flash memory access time. Access to the bit can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. ... Note: Before entering Stop 1 mode software must set FLASH wait state latency to at least 1.
pub type LATENCY_R = crate::FieldReader;
///Field `LATENCY` writer - Latency These bits represent the ratio between the AHB hclk1 clock period and the Flash memory access time. Access to the bit can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. ... Note: Before entering Stop 1 mode software must set FLASH wait state latency to at least 1.
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PRFTEN` reader - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory. This bit can be protected against unprivileged access by FLASH NSPRIV.
pub type PRFTEN_R = crate::BitReader;
///Field `PRFTEN` writer - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory. This bit can be protected against unprivileged access by FLASH NSPRIV.
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPM` reader - Low-power read mode This bit puts the Flash memory in low-power read mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. This bit can't be written when a Flash program or erase operation is busy (BSY = 1) or when the write buffer is not empty (WDW = 1). Changing this bit while a Flash program or erase operation is busy (BSY = 1) is rejected.
pub type LPM_R = crate::BitReader;
///Field `LPM` writer - Low-power read mode This bit puts the Flash memory in low-power read mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. This bit can't be written when a Flash program or erase operation is busy (BSY = 1) or when the write buffer is not empty (WDW = 1). Changing this bit while a Flash program or erase operation is busy (BSY = 1) is rejected.
pub type LPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDREQ` reader - Flash power-down mode request This bit requests Flash to enter power-down mode. When Flash enters power-down mode, this bit is cleared by hardware and the PDKEYR is locked. This bit is write-protected with FLASH_PDKEYR. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV.
pub type PDREQ_R = crate::BitReader;
///Field `PDREQ` writer - Flash power-down mode request This bit requests Flash to enter power-down mode. When Flash enters power-down mode, this bit is cleared by hardware and the PDKEYR is locked. This bit is write-protected with FLASH_PDKEYR. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV.
pub type PDREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP_PD` reader - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. The Flash must not be put in power-down while a program or an erase operation is ongoing.
pub type SLEEP_PD_R = crate::BitReader;
///Field `SLEEP_PD` writer - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. The Flash must not be put in power-down while a program or an erase operation is ongoing.
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Latency These bits represent the ratio between the AHB hclk1 clock period and the Flash memory access time. Access to the bit can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. ... Note: Before entering Stop 1 mode software must set FLASH wait state latency to at least 1.
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory. This bit can be protected against unprivileged access by FLASH NSPRIV.
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Low-power read mode This bit puts the Flash memory in low-power read mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. This bit can't be written when a Flash program or erase operation is busy (BSY = 1) or when the write buffer is not empty (WDW = 1). Changing this bit while a Flash program or erase operation is busy (BSY = 1) is rejected.
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Flash power-down mode request This bit requests Flash to enter power-down mode. When Flash enters power-down mode, this bit is cleared by hardware and the PDKEYR is locked. This bit is write-protected with FLASH_PDKEYR. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV.
    #[inline(always)]
    pub fn pdreq(&self) -> PDREQ_R {
        PDREQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. The Flash must not be put in power-down while a program or an erase operation is ongoing.
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("prften", &self.prften())
            .field("lpm", &self.lpm())
            .field("pdreq", &self.pdreq())
            .field("sleep_pd", &self.sleep_pd())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Latency These bits represent the ratio between the AHB hclk1 clock period and the Flash memory access time. Access to the bit can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. ... Note: Before entering Stop 1 mode software must set FLASH wait state latency to at least 1.
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 8 - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory. This bit can be protected against unprivileged access by FLASH NSPRIV.
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 11 - Low-power read mode This bit puts the Flash memory in low-power read mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. This bit can't be written when a Flash program or erase operation is busy (BSY = 1) or when the write buffer is not empty (WDW = 1). Changing this bit while a Flash program or erase operation is busy (BSY = 1) is rejected.
    #[inline(always)]
    pub fn lpm(&mut self) -> LPM_W<'_, ACRrs> {
        LPM_W::new(self, 11)
    }
    ///Bit 12 - Flash power-down mode request This bit requests Flash to enter power-down mode. When Flash enters power-down mode, this bit is cleared by hardware and the PDKEYR is locked. This bit is write-protected with FLASH_PDKEYR. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV.
    #[inline(always)]
    pub fn pdreq(&mut self) -> PDREQ_W<'_, ACRrs> {
        PDREQ_W::new(self, 12)
    }
    ///Bit 14 - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with FLASH SPRIV or when non-secure with FLASH NSPRIV. The Flash must not be put in power-down while a program or an erase operation is ongoing.
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<'_, ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
}
/**FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0x01
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x01;
}
