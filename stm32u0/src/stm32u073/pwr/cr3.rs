///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `EWUP1` reader - Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register.
pub type EWUP1_R = crate::BitReader;
///Field `EWUP1` writer - Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register.
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP2` reader - Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register.
pub type EWUP2_R = crate::BitReader;
///Field `EWUP2` writer - Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register.
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP3` reader - Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register.
pub type EWUP3_R = crate::BitReader;
///Field `EWUP3` writer - Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register.
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP4` reader - Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
pub type EWUP4_R = crate::BitReader;
///Field `EWUP4` writer - Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP5` reader - Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register.
pub type EWUP5_R = crate::BitReader;
///Field `EWUP5` writer - Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register.
pub type EWUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP7` reader - Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register.
pub type EWUP7_R = crate::BitReader;
///Field `EWUP7` writer - Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register.
pub type EWUP7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRS` reader - SRAM2 retention in Standby mode
pub type RRS_R = crate::BitReader;
///Field `RRS` writer - SRAM2 retention in Standby mode
pub type RRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENULP` reader - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes.
pub type ENULP_R = crate::BitReader;
///Field `ENULP` writer - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes.
pub type ENULP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode.
pub type APC_R = crate::BitReader;
///Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode.
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIWUL` reader - Enable internal wake-up line
pub type EIWUL_R = crate::BitReader;
///Field `EIWUL` writer - Enable internal wake-up line
pub type EIWUL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup7(&self) -> EWUP7_R {
        EWUP7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - SRAM2 retention in Standby mode
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes.
    #[inline(always)]
    pub fn enulp(&self) -> ENULP_R {
        ENULP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode.
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Enable internal wake-up line
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("ewup1", &self.ewup1())
            .field("ewup2", &self.ewup2())
            .field("ewup3", &self.ewup3())
            .field("ewup4", &self.ewup4())
            .field("ewup5", &self.ewup5())
            .field("ewup7", &self.ewup7())
            .field("rrs", &self.rrs())
            .field("enulp", &self.enulp())
            .field("apc", &self.apc())
            .field("eiwul", &self.eiwul())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<'_, CR3rs> {
        EWUP1_W::new(self, 0)
    }
    ///Bit 1 - Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<'_, CR3rs> {
        EWUP2_W::new(self, 1)
    }
    ///Bit 2 - Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<'_, CR3rs> {
        EWUP3_W::new(self, 2)
    }
    ///Bit 3 - Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W<'_, CR3rs> {
        EWUP4_W::new(self, 3)
    }
    ///Bit 4 - Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W<'_, CR3rs> {
        EWUP5_W::new(self, 4)
    }
    ///Bit 6 - Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup7(&mut self) -> EWUP7_W<'_, CR3rs> {
        EWUP7_W::new(self, 6)
    }
    ///Bit 8 - SRAM2 retention in Standby mode
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W<'_, CR3rs> {
        RRS_W::new(self, 8)
    }
    ///Bit 9 - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes.
    #[inline(always)]
    pub fn enulp(&mut self) -> ENULP_W<'_, CR3rs> {
        ENULP_W::new(self, 9)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode.
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<'_, CR3rs> {
        APC_W::new(self, 10)
    }
    ///Bit 15 - Enable internal wake-up line
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W<'_, CR3rs> {
        EIWUL_W::new(self, 15)
    }
}
/**Power control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#PWR:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0x8000
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x8000;
}
