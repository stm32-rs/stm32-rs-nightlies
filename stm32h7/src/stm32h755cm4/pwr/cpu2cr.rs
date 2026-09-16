///Register `CPU2CR` reader
pub type R = crate::R<CPU2CRrs>;
///Register `CPU2CR` writer
pub type W = crate::W<CPU2CRrs>;
///Field `PDDS_D1` reader - D1 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D1 domain
pub type PDDS_D1_R = crate::BitReader;
///Field `PDDS_D1` writer - D1 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D1 domain
pub type PDDS_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDDS_D2` reader - D2 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D2 domain
pub type PDDS_D2_R = crate::BitReader;
///Field `PDDS_D2` writer - D2 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D2 domain
pub type PDDS_D2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDDS_D3` reader - D3 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D3 domain
pub type PDDS_D3_R = crate::BitReader;
///Field `PDDS_D3` writer - D3 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D3 domain
pub type PDDS_D3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLD1F` reader - CPU1 in hold wakeup flag. This flag also generates a CPU2 interrupt. CPU2 has been woken up from a CPU1 wakeup source with CPU1 on hold. This flag is set by hardware and cleared only by a system reset or by setting the CPU2 CSSF bit.
pub type HOLD1F_R = crate::BitReader;
///Field `HOLD1F` writer - CPU1 in hold wakeup flag. This flag also generates a CPU2 interrupt. CPU2 has been woken up from a CPU1 wakeup source with CPU1 on hold. This flag is set by hardware and cleared only by a system reset or by setting the CPU2 CSSF bit.
pub type HOLD1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPF` reader - Stop Flag. This bit is set by hardware and cleared only by any reset or by setting the CPU2 CSSF bit.
pub type STOPF_R = crate::BitReader;
///Field `STOPF` writer - Stop Flag. This bit is set by hardware and cleared only by any reset or by setting the CPU2 CSSF bit.
pub type STOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBF` reader - System Standby flag. This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU2 CSSF bit
pub type SBF_R = crate::BitReader;
///Field `SBF` writer - System Standby flag. This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU2 CSSF bit
pub type SBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBF_D1` reader - D1 domain DStandby flag. This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU2 CSSF bit
pub type SBF_D1_R = crate::BitReader;
///Field `SBF_D1` writer - D1 domain DStandby flag. This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU2 CSSF bit
pub type SBF_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBF_D2` reader - D2 domain DStandby flag. This bit is set by hardware and cleared by any Reset or by setting the CPU2 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode.
pub type SBF_D2_R = crate::BitReader;
///Field `SBF_D2` writer - D2 domain DStandby flag. This bit is set by hardware and cleared by any Reset or by setting the CPU2 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode.
pub type SBF_D2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSF` reader - Clear D2 domain CPU2 Standby, Stop and HOLD flags (always read as 0)
pub type CSSF_R = crate::BitReader;
///Field `CSSF` writer - Clear D2 domain CPU2 Standby, Stop and HOLD flags (always read as 0)
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLD1` reader - Hold the CPU1 and allocated peripherals when exiting from Stop mode.
pub type HOLD1_R = crate::BitReader;
///Field `HOLD1` writer - Hold the CPU1 and allocated peripherals when exiting from Stop mode.
pub type HOLD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RUN_D3` reader - Keep D3 domain in Run mode regardless of the other CPU subsystems modes.
pub type RUN_D3_R = crate::BitReader;
///Field `RUN_D3` writer - Keep D3 domain in Run mode regardless of the other CPU subsystems modes.
pub type RUN_D3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D1 domain
    #[inline(always)]
    pub fn pdds_d1(&self) -> PDDS_D1_R {
        PDDS_D1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - D2 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D2 domain
    #[inline(always)]
    pub fn pdds_d2(&self) -> PDDS_D2_R {
        PDDS_D2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - D3 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D3 domain
    #[inline(always)]
    pub fn pdds_d3(&self) -> PDDS_D3_R {
        PDDS_D3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CPU1 in hold wakeup flag. This flag also generates a CPU2 interrupt. CPU2 has been woken up from a CPU1 wakeup source with CPU1 on hold. This flag is set by hardware and cleared only by a system reset or by setting the CPU2 CSSF bit.
    #[inline(always)]
    pub fn hold1f(&self) -> HOLD1F_R {
        HOLD1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Stop Flag. This bit is set by hardware and cleared only by any reset or by setting the CPU2 CSSF bit.
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - System Standby flag. This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU2 CSSF bit
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - D1 domain DStandby flag. This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU2 CSSF bit
    #[inline(always)]
    pub fn sbf_d1(&self) -> SBF_D1_R {
        SBF_D1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - D2 domain DStandby flag. This bit is set by hardware and cleared by any Reset or by setting the CPU2 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode.
    #[inline(always)]
    pub fn sbf_d2(&self) -> SBF_D2_R {
        SBF_D2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clear D2 domain CPU2 Standby, Stop and HOLD flags (always read as 0)
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Hold the CPU1 and allocated peripherals when exiting from Stop mode.
    #[inline(always)]
    pub fn hold1(&self) -> HOLD1_R {
        HOLD1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Keep D3 domain in Run mode regardless of the other CPU subsystems modes.
    #[inline(always)]
    pub fn run_d3(&self) -> RUN_D3_R {
        RUN_D3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU2CR")
            .field("pdds_d1", &self.pdds_d1())
            .field("pdds_d2", &self.pdds_d2())
            .field("pdds_d3", &self.pdds_d3())
            .field("hold1f", &self.hold1f())
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .field("sbf_d1", &self.sbf_d1())
            .field("sbf_d2", &self.sbf_d2())
            .field("cssf", &self.cssf())
            .field("hold1", &self.hold1())
            .field("run_d3", &self.run_d3())
            .finish()
    }
}
impl W {
    ///Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D1 domain
    #[inline(always)]
    pub fn pdds_d1(&mut self) -> PDDS_D1_W<'_, CPU2CRrs> {
        PDDS_D1_W::new(self, 0)
    }
    ///Bit 1 - D2 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D2 domain
    #[inline(always)]
    pub fn pdds_d2(&mut self) -> PDDS_D2_W<'_, CPU2CRrs> {
        PDDS_D2_W::new(self, 1)
    }
    ///Bit 2 - D3 domain Power Down Deepsleep selection. This bit allows CPU2 to define the Deepsleep mode for D3 domain
    #[inline(always)]
    pub fn pdds_d3(&mut self) -> PDDS_D3_W<'_, CPU2CRrs> {
        PDDS_D3_W::new(self, 2)
    }
    ///Bit 4 - CPU1 in hold wakeup flag. This flag also generates a CPU2 interrupt. CPU2 has been woken up from a CPU1 wakeup source with CPU1 on hold. This flag is set by hardware and cleared only by a system reset or by setting the CPU2 CSSF bit.
    #[inline(always)]
    pub fn hold1f(&mut self) -> HOLD1F_W<'_, CPU2CRrs> {
        HOLD1F_W::new(self, 4)
    }
    ///Bit 5 - Stop Flag. This bit is set by hardware and cleared only by any reset or by setting the CPU2 CSSF bit.
    #[inline(always)]
    pub fn stopf(&mut self) -> STOPF_W<'_, CPU2CRrs> {
        STOPF_W::new(self, 5)
    }
    ///Bit 6 - System Standby flag. This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU2 CSSF bit
    #[inline(always)]
    pub fn sbf(&mut self) -> SBF_W<'_, CPU2CRrs> {
        SBF_W::new(self, 6)
    }
    ///Bit 7 - D1 domain DStandby flag. This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU2 CSSF bit
    #[inline(always)]
    pub fn sbf_d1(&mut self) -> SBF_D1_W<'_, CPU2CRrs> {
        SBF_D1_W::new(self, 7)
    }
    ///Bit 8 - D2 domain DStandby flag. This bit is set by hardware and cleared by any Reset or by setting the CPU2 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode.
    #[inline(always)]
    pub fn sbf_d2(&mut self) -> SBF_D2_W<'_, CPU2CRrs> {
        SBF_D2_W::new(self, 8)
    }
    ///Bit 9 - Clear D2 domain CPU2 Standby, Stop and HOLD flags (always read as 0)
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<'_, CPU2CRrs> {
        CSSF_W::new(self, 9)
    }
    ///Bit 10 - Hold the CPU1 and allocated peripherals when exiting from Stop mode.
    #[inline(always)]
    pub fn hold1(&mut self) -> HOLD1_W<'_, CPU2CRrs> {
        HOLD1_W::new(self, 10)
    }
    ///Bit 11 - Keep D3 domain in Run mode regardless of the other CPU subsystems modes.
    #[inline(always)]
    pub fn run_d3(&mut self) -> RUN_D3_W<'_, CPU2CRrs> {
        RUN_D3_W::new(self, 11)
    }
}
/**This register allows controlling CPU2 power

You can [`read`](crate::Reg::read) this register and get [`cpu2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#PWR:CPU2CR)*/
pub struct CPU2CRrs;
impl crate::RegisterSpec for CPU2CRrs {
    type Ux = u32;
}
///`read()` method returns [`cpu2cr::R`](R) reader structure
impl crate::Readable for CPU2CRrs {}
///`write(|w| ..)` method takes [`cpu2cr::W`](W) writer structure
impl crate::Writable for CPU2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPU2CR to value 0
impl crate::Resettable for CPU2CRrs {}
