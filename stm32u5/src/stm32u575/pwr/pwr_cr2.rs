#[doc = "Register `PWR_CR2` reader"]
pub type R = crate::R<PWR_CR2rs>;
#[doc = "Register `PWR_CR2` writer"]
pub type W = crate::W<PWR_CR2rs>;
#[doc = "Field `SRAM1PDS1` reader - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM1PDS1_R = crate::BitReader;
#[doc = "Field `SRAM1PDS1` writer - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM1PDS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1PDS2` reader - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM1PDS2_R = crate::BitReader;
#[doc = "Field `SRAM1PDS2` writer - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM1PDS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1PDS3` reader - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM1PDS3_R = crate::BitReader;
#[doc = "Field `SRAM1PDS3` writer - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM1PDS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2PDS1` reader - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1."]
pub type SRAM2PDS1_R = crate::BitReader;
#[doc = "Field `SRAM2PDS1` writer - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1."]
pub type SRAM2PDS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2PDS2` reader - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1."]
pub type SRAM2PDS2_R = crate::BitReader;
#[doc = "Field `SRAM2PDS2` writer - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1."]
pub type SRAM2PDS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM4PDS` reader - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM4PDS_R = crate::BitReader;
#[doc = "Field `SRAM4PDS` writer - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM4PDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICRAMPDS` reader - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type ICRAMPDS_R = crate::BitReader;
#[doc = "Field `ICRAMPDS` writer - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type ICRAMPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC1RAMPDS` reader - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type DC1RAMPDS_R = crate::BitReader;
#[doc = "Field `DC1RAMPDS` writer - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type DC1RAMPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DRAMPDS` reader - DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type DMA2DRAMPDS_R = crate::BitReader;
#[doc = "Field `DMA2DRAMPDS` writer - DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type DMA2DRAMPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRAMPDS` reader - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type PRAMPDS_R = crate::BitReader;
#[doc = "Field `PRAMPDS` writer - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type PRAMPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKARAMPDS` reader - PKA SRAM power-down"]
pub type PKARAMPDS_R = crate::BitReader;
#[doc = "Field `PKARAMPDS` writer - PKA SRAM power-down"]
pub type PKARAMPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM4FWU` reader - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes."]
pub type SRAM4FWU_R = crate::BitReader;
#[doc = "Field `SRAM4FWU` writer - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes."]
pub type SRAM4FWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHFWU` reader - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption."]
pub type FLASHFWU_R = crate::BitReader;
#[doc = "Field `FLASHFWU` writer - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption."]
pub type FLASHFWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3PDS1` reader - SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS1_R = crate::BitReader;
#[doc = "Field `SRAM3PDS1` writer - SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3PDS2` reader - SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS2_R = crate::BitReader;
#[doc = "Field `SRAM3PDS2` writer - SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3PDS3` reader - SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS3_R = crate::BitReader;
#[doc = "Field `SRAM3PDS3` writer - SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3PDS4` reader - SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS4_R = crate::BitReader;
#[doc = "Field `SRAM3PDS4` writer - SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3PDS5` reader - SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS5_R = crate::BitReader;
#[doc = "Field `SRAM3PDS5` writer - SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3PDS6` reader - SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS6_R = crate::BitReader;
#[doc = "Field `SRAM3PDS6` writer - SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3PDS7` reader - SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS7_R = crate::BitReader;
#[doc = "Field `SRAM3PDS7` writer - SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3PDS8` reader - SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS8_R = crate::BitReader;
#[doc = "Field `SRAM3PDS8` writer - SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
pub type SRAM3PDS8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRDRUN` reader - SmartRun domain in Run mode"]
pub type SRDRUN_R = crate::BitReader;
#[doc = "Field `SRDRUN` writer - SmartRun domain in Run mode"]
pub type SRDRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram1pds1(&self) -> SRAM1PDS1_R {
        SRAM1PDS1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram1pds2(&self) -> SRAM1PDS2_R {
        SRAM1PDS2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram1pds3(&self) -> SRAM1PDS3_R {
        SRAM1PDS3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1."]
    #[inline(always)]
    pub fn sram2pds1(&self) -> SRAM2PDS1_R {
        SRAM2PDS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1."]
    #[inline(always)]
    pub fn sram2pds2(&self) -> SRAM2PDS2_R {
        SRAM2PDS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram4pds(&self) -> SRAM4PDS_R {
        SRAM4PDS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn icrampds(&self) -> ICRAMPDS_R {
        ICRAMPDS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn dc1rampds(&self) -> DC1RAMPDS_R {
        DC1RAMPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn dma2drampds(&self) -> DMA2DRAMPDS_R {
        DMA2DRAMPDS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn prampds(&self) -> PRAMPDS_R {
        PRAMPDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PKA SRAM power-down"]
    #[inline(always)]
    pub fn pkarampds(&self) -> PKARAMPDS_R {
        PKARAMPDS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes."]
    #[inline(always)]
    pub fn sram4fwu(&self) -> SRAM4FWU_R {
        SRAM4FWU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption."]
    #[inline(always)]
    pub fn flashfwu(&self) -> FLASHFWU_R {
        FLASHFWU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram3pds1(&self) -> SRAM3PDS1_R {
        SRAM3PDS1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram3pds2(&self) -> SRAM3PDS2_R {
        SRAM3PDS2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram3pds3(&self) -> SRAM3PDS3_R {
        SRAM3PDS3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram3pds4(&self) -> SRAM3PDS4_R {
        SRAM3PDS4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram3pds5(&self) -> SRAM3PDS5_R {
        SRAM3PDS5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram3pds6(&self) -> SRAM3PDS6_R {
        SRAM3PDS6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram3pds7(&self) -> SRAM3PDS7_R {
        SRAM3PDS7_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    pub fn sram3pds8(&self) -> SRAM3PDS8_R {
        SRAM3PDS8_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - SmartRun domain in Run mode"]
    #[inline(always)]
    pub fn srdrun(&self) -> SRDRUN_R {
        SRDRUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram1pds1(&mut self) -> SRAM1PDS1_W<PWR_CR2rs> {
        SRAM1PDS1_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram1pds2(&mut self) -> SRAM1PDS2_W<PWR_CR2rs> {
        SRAM1PDS2_W::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram1pds3(&mut self) -> SRAM1PDS3_W<PWR_CR2rs> {
        SRAM1PDS3_W::new(self, 2)
    }
    #[doc = "Bit 4 - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1."]
    #[inline(always)]
    #[must_use]
    pub fn sram2pds1(&mut self) -> SRAM2PDS1_W<PWR_CR2rs> {
        SRAM2PDS1_W::new(self, 4)
    }
    #[doc = "Bit 5 - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1."]
    #[inline(always)]
    #[must_use]
    pub fn sram2pds2(&mut self) -> SRAM2PDS2_W<PWR_CR2rs> {
        SRAM2PDS2_W::new(self, 5)
    }
    #[doc = "Bit 6 - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram4pds(&mut self) -> SRAM4PDS_W<PWR_CR2rs> {
        SRAM4PDS_W::new(self, 6)
    }
    #[doc = "Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn icrampds(&mut self) -> ICRAMPDS_W<PWR_CR2rs> {
        ICRAMPDS_W::new(self, 8)
    }
    #[doc = "Bit 9 - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn dc1rampds(&mut self) -> DC1RAMPDS_W<PWR_CR2rs> {
        DC1RAMPDS_W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn dma2drampds(&mut self) -> DMA2DRAMPDS_W<PWR_CR2rs> {
        DMA2DRAMPDS_W::new(self, 10)
    }
    #[doc = "Bit 11 - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn prampds(&mut self) -> PRAMPDS_W<PWR_CR2rs> {
        PRAMPDS_W::new(self, 11)
    }
    #[doc = "Bit 12 - PKA SRAM power-down"]
    #[inline(always)]
    #[must_use]
    pub fn pkarampds(&mut self) -> PKARAMPDS_W<PWR_CR2rs> {
        PKARAMPDS_W::new(self, 12)
    }
    #[doc = "Bit 13 - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes."]
    #[inline(always)]
    #[must_use]
    pub fn sram4fwu(&mut self) -> SRAM4FWU_W<PWR_CR2rs> {
        SRAM4FWU_W::new(self, 13)
    }
    #[doc = "Bit 14 - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption."]
    #[inline(always)]
    #[must_use]
    pub fn flashfwu(&mut self) -> FLASHFWU_W<PWR_CR2rs> {
        FLASHFWU_W::new(self, 14)
    }
    #[doc = "Bit 16 - SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram3pds1(&mut self) -> SRAM3PDS1_W<PWR_CR2rs> {
        SRAM3PDS1_W::new(self, 16)
    }
    #[doc = "Bit 17 - SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram3pds2(&mut self) -> SRAM3PDS2_W<PWR_CR2rs> {
        SRAM3PDS2_W::new(self, 17)
    }
    #[doc = "Bit 18 - SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram3pds3(&mut self) -> SRAM3PDS3_W<PWR_CR2rs> {
        SRAM3PDS3_W::new(self, 18)
    }
    #[doc = "Bit 19 - SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram3pds4(&mut self) -> SRAM3PDS4_W<PWR_CR2rs> {
        SRAM3PDS4_W::new(self, 19)
    }
    #[doc = "Bit 20 - SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram3pds5(&mut self) -> SRAM3PDS5_W<PWR_CR2rs> {
        SRAM3PDS5_W::new(self, 20)
    }
    #[doc = "Bit 21 - SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram3pds6(&mut self) -> SRAM3PDS6_W<PWR_CR2rs> {
        SRAM3PDS6_W::new(self, 21)
    }
    #[doc = "Bit 22 - SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram3pds7(&mut self) -> SRAM3PDS7_W<PWR_CR2rs> {
        SRAM3PDS7_W::new(self, 22)
    }
    #[doc = "Bit 23 - SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
    #[inline(always)]
    #[must_use]
    pub fn sram3pds8(&mut self) -> SRAM3PDS8_W<PWR_CR2rs> {
        SRAM3PDS8_W::new(self, 23)
    }
    #[doc = "Bit 31 - SmartRun domain in Run mode"]
    #[inline(always)]
    #[must_use]
    pub fn srdrun(&mut self) -> SRDRUN_W<PWR_CR2rs> {
        SRDRUN_W::new(self, 31)
    }
}
#[doc = "PWR control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_CR2rs;
impl crate::RegisterSpec for PWR_CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr2::R`](R) reader structure"]
impl crate::Readable for PWR_CR2rs {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr2::W`](W) writer structure"]
impl crate::Writable for PWR_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR2 to value 0"]
impl crate::Resettable for PWR_CR2rs {
    const RESET_VALUE: u32 = 0;
}
