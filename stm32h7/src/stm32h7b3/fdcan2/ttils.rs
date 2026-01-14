///Register `TTILS` reader
pub type R = crate::R<TTILSrs>;
///Register `TTILS` writer
pub type W = crate::W<TTILSrs>;
///Field `SBCL` reader - Start of Basic Cycle Interrupt Line
pub type SBCL_R = crate::BitReader;
///Field `SBCL` writer - Start of Basic Cycle Interrupt Line
pub type SBCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMCL` reader - Start of Matrix Cycle Interrupt Line
pub type SMCL_R = crate::BitReader;
///Field `SMCL` writer - Start of Matrix Cycle Interrupt Line
pub type SMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSML` reader - Change of Synchronization Mode Interrupt Line
pub type CSML_R = crate::BitReader;
///Field `CSML` writer - Change of Synchronization Mode Interrupt Line
pub type CSML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOGL` reader - Start of Gap Interrupt Line
pub type SOGL_R = crate::BitReader;
///Field `SOGL` writer - Start of Gap Interrupt Line
pub type SOGL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTMIL` reader - Register Time Mark Interrupt Line
pub type RTMIL_R = crate::BitReader;
///Field `RTMIL` writer - Register Time Mark Interrupt Line
pub type RTMIL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTMIL` reader - Trigger Time Mark Event Internal Interrupt Line
pub type TTMIL_R = crate::BitReader;
///Field `TTMIL` writer - Trigger Time Mark Event Internal Interrupt Line
pub type TTMIL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWEL` reader - Stop Watch Event Interrupt Line
pub type SWEL_R = crate::BitReader;
///Field `SWEL` writer - Stop Watch Event Interrupt Line
pub type SWEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTWL` reader - Global Time Wrap Interrupt Line
pub type GTWL_R = crate::BitReader;
///Field `GTWL` writer - Global Time Wrap Interrupt Line
pub type GTWL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTDL` reader - Global Time Discontinuity Interrupt Line
pub type GTDL_R = crate::BitReader;
///Field `GTDL` writer - Global Time Discontinuity Interrupt Line
pub type GTDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTEL` reader - Global Time Error Interrupt Line
pub type GTEL_R = crate::BitReader;
///Field `GTEL` writer - Global Time Error Interrupt Line
pub type GTEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUL` reader - Tx Count Underflow Interrupt Line
pub type TXUL_R = crate::BitReader;
///Field `TXUL` writer - Tx Count Underflow Interrupt Line
pub type TXUL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOL` reader - Tx Count Overflow Interrupt Line
pub type TXOL_R = crate::BitReader;
///Field `TXOL` writer - Tx Count Overflow Interrupt Line
pub type TXOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE1L` reader - Scheduling Error 1 Interrupt Line
pub type SE1L_R = crate::BitReader;
///Field `SE1L` writer - Scheduling Error 1 Interrupt Line
pub type SE1L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE2L` reader - Scheduling Error 2 Interrupt Line
pub type SE2L_R = crate::BitReader;
///Field `SE2L` writer - Scheduling Error 2 Interrupt Line
pub type SE2L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELCL` reader - Change Error Level Interrupt Line
pub type ELCL_R = crate::BitReader;
///Field `ELCL` writer - Change Error Level Interrupt Line
pub type ELCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWTGL` reader - Initialization Watch Trigger Interrupt Line
pub type IWTGL_R = crate::BitReader;
///Field `IWTGL` writer - Initialization Watch Trigger Interrupt Line
pub type IWTGL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WTL` reader - Watch Trigger Interrupt Line
pub type WTL_R = crate::BitReader;
///Field `WTL` writer - Watch Trigger Interrupt Line
pub type WTL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWL` reader - Application Watchdog Interrupt Line
pub type AWL_R = crate::BitReader;
///Field `AWL` writer - Application Watchdog Interrupt Line
pub type AWL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CERL` reader - Configuration Error Interrupt Line
pub type CERL_R = crate::BitReader;
///Field `CERL` writer - Configuration Error Interrupt Line
pub type CERL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Start of Basic Cycle Interrupt Line
    #[inline(always)]
    pub fn sbcl(&self) -> SBCL_R {
        SBCL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start of Matrix Cycle Interrupt Line
    #[inline(always)]
    pub fn smcl(&self) -> SMCL_R {
        SMCL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Change of Synchronization Mode Interrupt Line
    #[inline(always)]
    pub fn csml(&self) -> CSML_R {
        CSML_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Start of Gap Interrupt Line
    #[inline(always)]
    pub fn sogl(&self) -> SOGL_R {
        SOGL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Register Time Mark Interrupt Line
    #[inline(always)]
    pub fn rtmil(&self) -> RTMIL_R {
        RTMIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Trigger Time Mark Event Internal Interrupt Line
    #[inline(always)]
    pub fn ttmil(&self) -> TTMIL_R {
        TTMIL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Stop Watch Event Interrupt Line
    #[inline(always)]
    pub fn swel(&self) -> SWEL_R {
        SWEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Global Time Wrap Interrupt Line
    #[inline(always)]
    pub fn gtwl(&self) -> GTWL_R {
        GTWL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Global Time Discontinuity Interrupt Line
    #[inline(always)]
    pub fn gtdl(&self) -> GTDL_R {
        GTDL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Global Time Error Interrupt Line
    #[inline(always)]
    pub fn gtel(&self) -> GTEL_R {
        GTEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx Count Underflow Interrupt Line
    #[inline(always)]
    pub fn txul(&self) -> TXUL_R {
        TXUL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx Count Overflow Interrupt Line
    #[inline(always)]
    pub fn txol(&self) -> TXOL_R {
        TXOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Scheduling Error 1 Interrupt Line
    #[inline(always)]
    pub fn se1l(&self) -> SE1L_R {
        SE1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Scheduling Error 2 Interrupt Line
    #[inline(always)]
    pub fn se2l(&self) -> SE2L_R {
        SE2L_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Change Error Level Interrupt Line
    #[inline(always)]
    pub fn elcl(&self) -> ELCL_R {
        ELCL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Initialization Watch Trigger Interrupt Line
    #[inline(always)]
    pub fn iwtgl(&self) -> IWTGL_R {
        IWTGL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Watch Trigger Interrupt Line
    #[inline(always)]
    pub fn wtl(&self) -> WTL_R {
        WTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Application Watchdog Interrupt Line
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Configuration Error Interrupt Line
    #[inline(always)]
    pub fn cerl(&self) -> CERL_R {
        CERL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTILS")
            .field("sbcl", &self.sbcl())
            .field("smcl", &self.smcl())
            .field("csml", &self.csml())
            .field("sogl", &self.sogl())
            .field("rtmil", &self.rtmil())
            .field("ttmil", &self.ttmil())
            .field("swel", &self.swel())
            .field("gtwl", &self.gtwl())
            .field("gtdl", &self.gtdl())
            .field("gtel", &self.gtel())
            .field("txul", &self.txul())
            .field("txol", &self.txol())
            .field("se1l", &self.se1l())
            .field("se2l", &self.se2l())
            .field("elcl", &self.elcl())
            .field("iwtgl", &self.iwtgl())
            .field("wtl", &self.wtl())
            .field("awl", &self.awl())
            .field("cerl", &self.cerl())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start of Basic Cycle Interrupt Line
    #[inline(always)]
    pub fn sbcl(&mut self) -> SBCL_W<'_, TTILSrs> {
        SBCL_W::new(self, 0)
    }
    ///Bit 1 - Start of Matrix Cycle Interrupt Line
    #[inline(always)]
    pub fn smcl(&mut self) -> SMCL_W<'_, TTILSrs> {
        SMCL_W::new(self, 1)
    }
    ///Bit 2 - Change of Synchronization Mode Interrupt Line
    #[inline(always)]
    pub fn csml(&mut self) -> CSML_W<'_, TTILSrs> {
        CSML_W::new(self, 2)
    }
    ///Bit 3 - Start of Gap Interrupt Line
    #[inline(always)]
    pub fn sogl(&mut self) -> SOGL_W<'_, TTILSrs> {
        SOGL_W::new(self, 3)
    }
    ///Bit 4 - Register Time Mark Interrupt Line
    #[inline(always)]
    pub fn rtmil(&mut self) -> RTMIL_W<'_, TTILSrs> {
        RTMIL_W::new(self, 4)
    }
    ///Bit 5 - Trigger Time Mark Event Internal Interrupt Line
    #[inline(always)]
    pub fn ttmil(&mut self) -> TTMIL_W<'_, TTILSrs> {
        TTMIL_W::new(self, 5)
    }
    ///Bit 6 - Stop Watch Event Interrupt Line
    #[inline(always)]
    pub fn swel(&mut self) -> SWEL_W<'_, TTILSrs> {
        SWEL_W::new(self, 6)
    }
    ///Bit 7 - Global Time Wrap Interrupt Line
    #[inline(always)]
    pub fn gtwl(&mut self) -> GTWL_W<'_, TTILSrs> {
        GTWL_W::new(self, 7)
    }
    ///Bit 8 - Global Time Discontinuity Interrupt Line
    #[inline(always)]
    pub fn gtdl(&mut self) -> GTDL_W<'_, TTILSrs> {
        GTDL_W::new(self, 8)
    }
    ///Bit 9 - Global Time Error Interrupt Line
    #[inline(always)]
    pub fn gtel(&mut self) -> GTEL_W<'_, TTILSrs> {
        GTEL_W::new(self, 9)
    }
    ///Bit 10 - Tx Count Underflow Interrupt Line
    #[inline(always)]
    pub fn txul(&mut self) -> TXUL_W<'_, TTILSrs> {
        TXUL_W::new(self, 10)
    }
    ///Bit 11 - Tx Count Overflow Interrupt Line
    #[inline(always)]
    pub fn txol(&mut self) -> TXOL_W<'_, TTILSrs> {
        TXOL_W::new(self, 11)
    }
    ///Bit 12 - Scheduling Error 1 Interrupt Line
    #[inline(always)]
    pub fn se1l(&mut self) -> SE1L_W<'_, TTILSrs> {
        SE1L_W::new(self, 12)
    }
    ///Bit 13 - Scheduling Error 2 Interrupt Line
    #[inline(always)]
    pub fn se2l(&mut self) -> SE2L_W<'_, TTILSrs> {
        SE2L_W::new(self, 13)
    }
    ///Bit 14 - Change Error Level Interrupt Line
    #[inline(always)]
    pub fn elcl(&mut self) -> ELCL_W<'_, TTILSrs> {
        ELCL_W::new(self, 14)
    }
    ///Bit 15 - Initialization Watch Trigger Interrupt Line
    #[inline(always)]
    pub fn iwtgl(&mut self) -> IWTGL_W<'_, TTILSrs> {
        IWTGL_W::new(self, 15)
    }
    ///Bit 16 - Watch Trigger Interrupt Line
    #[inline(always)]
    pub fn wtl(&mut self) -> WTL_W<'_, TTILSrs> {
        WTL_W::new(self, 16)
    }
    ///Bit 17 - Application Watchdog Interrupt Line
    #[inline(always)]
    pub fn awl(&mut self) -> AWL_W<'_, TTILSrs> {
        AWL_W::new(self, 17)
    }
    ///Bit 18 - Configuration Error Interrupt Line
    #[inline(always)]
    pub fn cerl(&mut self) -> CERL_W<'_, TTILSrs> {
        CERL_W::new(self, 18)
    }
}
/**FDCAN TT Interrupt Line Select Register

You can [`read`](crate::Reg::read) this register and get [`ttils::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttils::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#FDCAN2:TTILS)*/
pub struct TTILSrs;
impl crate::RegisterSpec for TTILSrs {
    type Ux = u32;
}
///`read()` method returns [`ttils::R`](R) reader structure
impl crate::Readable for TTILSrs {}
///`write(|w| ..)` method takes [`ttils::W`](W) writer structure
impl crate::Writable for TTILSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTILS to value 0
impl crate::Resettable for TTILSrs {}
