///Register `CPU1CR` reader
pub type R = crate::R<CPU1CRrs>;
///Register `CPU1CR` writer
pub type W = crate::W<CPU1CRrs>;
///Field `PDDS_D1` reader - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain.
pub type PDDS_D1_R = crate::BitReader;
///Field `PDDS_D1` writer - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain.
pub type PDDS_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDDS_D2` reader - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain.
pub type PDDS_D2_R = crate::BitReader;
///Field `PDDS_D2` writer - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain.
pub type PDDS_D2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDDS_D3` reader - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain.
pub type PDDS_D3_R = crate::BitReader;
///Field `PDDS_D3` writer - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain.
pub type PDDS_D3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLD2F` reader - CPU2 in hold wakeup flag. This flag also generates a CPU1 interrupt. CPU1 has been woken up from a CPU2 wakeup source with CPU2 on hold. This flag is set by hardware and cleared only by a system reset or by setting the CPU1 CSSF bit.
pub type HOLD2F_R = crate::BitReader;
///Field `HOLD2F` writer - CPU2 in hold wakeup flag. This flag also generates a CPU1 interrupt. CPU1 has been woken up from a CPU2 wakeup source with CPU2 on hold. This flag is set by hardware and cleared only by a system reset or by setting the CPU1 CSSF bit.
pub type HOLD2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPF` reader - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit.
pub type STOPF_R = crate::BitReader;
///Field `SBF` reader - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit
pub type SBF_R = crate::BitReader;
///Field `SBF_D1` reader - D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode.
pub type SBF_D1_R = crate::BitReader;
///Field `SBF_D2` reader - D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode.
pub type SBF_D2_R = crate::BitReader;
///Field `CSSF` reader - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_R = crate::BitReader;
///Field `CSSF` writer - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLD2` reader - Hold the CPU2 and allocated peripherals when exiting from Stop mode.
pub type HOLD2_R = crate::BitReader;
///Field `HOLD2` writer - Hold the CPU2 and allocated peripherals when exiting from Stop mode.
pub type HOLD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RUN_D3` reader - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes
pub type RUN_D3_R = crate::BitReader;
///Field `RUN_D3` writer - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes
pub type RUN_D3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain.
    #[inline(always)]
    pub fn pdds_d1(&self) -> PDDS_D1_R {
        PDDS_D1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain.
    #[inline(always)]
    pub fn pdds_d2(&self) -> PDDS_D2_R {
        PDDS_D2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain.
    #[inline(always)]
    pub fn pdds_d3(&self) -> PDDS_D3_R {
        PDDS_D3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CPU2 in hold wakeup flag. This flag also generates a CPU1 interrupt. CPU1 has been woken up from a CPU2 wakeup source with CPU2 on hold. This flag is set by hardware and cleared only by a system reset or by setting the CPU1 CSSF bit.
    #[inline(always)]
    pub fn hold2f(&self) -> HOLD2F_R {
        HOLD2F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit.
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode.
    #[inline(always)]
    pub fn sbf_d1(&self) -> SBF_D1_R {
        SBF_D1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode.
    #[inline(always)]
    pub fn sbf_d2(&self) -> SBF_D2_R {
        SBF_D2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Hold the CPU2 and allocated peripherals when exiting from Stop mode.
    #[inline(always)]
    pub fn hold2(&self) -> HOLD2_R {
        HOLD2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes
    #[inline(always)]
    pub fn run_d3(&self) -> RUN_D3_R {
        RUN_D3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU1CR")
            .field("pdds_d1", &self.pdds_d1())
            .field("pdds_d2", &self.pdds_d2())
            .field("pdds_d3", &self.pdds_d3())
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .field("sbf_d1", &self.sbf_d1())
            .field("sbf_d2", &self.sbf_d2())
            .field("cssf", &self.cssf())
            .field("run_d3", &self.run_d3())
            .field("hold2f", &self.hold2f())
            .field("hold2", &self.hold2())
            .finish()
    }
}
impl W {
    ///Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain.
    #[inline(always)]
    pub fn pdds_d1(&mut self) -> PDDS_D1_W<'_, CPU1CRrs> {
        PDDS_D1_W::new(self, 0)
    }
    ///Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain.
    #[inline(always)]
    pub fn pdds_d2(&mut self) -> PDDS_D2_W<'_, CPU1CRrs> {
        PDDS_D2_W::new(self, 1)
    }
    ///Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain.
    #[inline(always)]
    pub fn pdds_d3(&mut self) -> PDDS_D3_W<'_, CPU1CRrs> {
        PDDS_D3_W::new(self, 2)
    }
    ///Bit 4 - CPU2 in hold wakeup flag. This flag also generates a CPU1 interrupt. CPU1 has been woken up from a CPU2 wakeup source with CPU2 on hold. This flag is set by hardware and cleared only by a system reset or by setting the CPU1 CSSF bit.
    #[inline(always)]
    pub fn hold2f(&mut self) -> HOLD2F_W<'_, CPU1CRrs> {
        HOLD2F_W::new(self, 4)
    }
    ///Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<'_, CPU1CRrs> {
        CSSF_W::new(self, 9)
    }
    ///Bit 10 - Hold the CPU2 and allocated peripherals when exiting from Stop mode.
    #[inline(always)]
    pub fn hold2(&mut self) -> HOLD2_W<'_, CPU1CRrs> {
        HOLD2_W::new(self, 10)
    }
    ///Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes
    #[inline(always)]
    pub fn run_d3(&mut self) -> RUN_D3_W<'_, CPU1CRrs> {
        RUN_D3_W::new(self, 11)
    }
}
/**This register allows controlling CPU1 power.

You can [`read`](crate::Reg::read) this register and get [`cpu1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#PWR:CPU1CR)*/
pub struct CPU1CRrs;
impl crate::RegisterSpec for CPU1CRrs {
    type Ux = u32;
}
///`read()` method returns [`cpu1cr::R`](R) reader structure
impl crate::Readable for CPU1CRrs {}
///`write(|w| ..)` method takes [`cpu1cr::W`](W) writer structure
impl crate::Writable for CPU1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPU1CR to value 0
impl crate::Resettable for CPU1CRrs {}
