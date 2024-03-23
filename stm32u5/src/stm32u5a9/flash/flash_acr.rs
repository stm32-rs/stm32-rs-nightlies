#[doc = "Register `FLASH_ACR` reader"]
pub type R = crate::R<FLASH_ACRrs>;
#[doc = "Register `FLASH_ACR` writer"]
pub type W = crate::W<FLASH_ACRrs>;
#[doc = "Field `LATENCY` reader - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ..."]
pub type LATENCY_R = crate::FieldReader;
#[doc = "Field `LATENCY` writer - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ..."]
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRFTEN` reader - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory."]
pub type PRFTEN_R = crate::BitReader;
#[doc = "Field `PRFTEN` writer - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory."]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM` reader - Low-power read mode This bit puts the Flash memory in low-power read mode."]
pub type LPM_R = crate::BitReader;
#[doc = "Field `LPM` writer - Low-power read mode This bit puts the Flash memory in low-power read mode."]
pub type LPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDREQ1` reader - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked."]
pub type PDREQ1_R = crate::BitReader;
#[doc = "Field `PDREQ1` writer - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked."]
pub type PDREQ1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDREQ2` reader - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked."]
pub type PDREQ2_R = crate::BitReader;
#[doc = "Field `PDREQ2` writer - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked."]
pub type PDREQ2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_PD` reader - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going."]
pub type SLEEP_PD_R = crate::BitReader;
#[doc = "Field `SLEEP_PD` writer - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going."]
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ..."]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory."]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Low-power read mode This bit puts the Flash memory in low-power read mode."]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked."]
    #[inline(always)]
    pub fn pdreq1(&self) -> PDREQ1_R {
        PDREQ1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked."]
    #[inline(always)]
    pub fn pdreq2(&self) -> PDREQ2_R {
        PDREQ2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going."]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ..."]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<FLASH_ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bit 8 - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory."]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<FLASH_ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    #[doc = "Bit 11 - Low-power read mode This bit puts the Flash memory in low-power read mode."]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LPM_W<FLASH_ACRrs> {
        LPM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked."]
    #[inline(always)]
    #[must_use]
    pub fn pdreq1(&mut self) -> PDREQ1_W<FLASH_ACRrs> {
        PDREQ1_W::new(self, 12)
    }
    #[doc = "Bit 13 - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked."]
    #[inline(always)]
    #[must_use]
    pub fn pdreq2(&mut self) -> PDREQ2_W<FLASH_ACRrs> {
        PDREQ2_W::new(self, 13)
    }
    #[doc = "Bit 14 - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going."]
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<FLASH_ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
}
#[doc = "FLASH access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ACRrs;
impl crate::RegisterSpec for FLASH_ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_acr::R`](R) reader structure"]
impl crate::Readable for FLASH_ACRrs {}
#[doc = "`write(|w| ..)` method takes [`flash_acr::W`](W) writer structure"]
impl crate::Writable for FLASH_ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_ACR to value 0"]
impl crate::Resettable for FLASH_ACRrs {
    const RESET_VALUE: u32 = 0;
}
