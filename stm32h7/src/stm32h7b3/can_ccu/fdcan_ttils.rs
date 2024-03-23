#[doc = "Register `FDCAN_TTILS` reader"]
pub type R = crate::R<FDCAN_TTILSrs>;
#[doc = "Register `FDCAN_TTILS` writer"]
pub type W = crate::W<FDCAN_TTILSrs>;
#[doc = "Field `SBCL` reader - Start of Basic Cycle Interrupt Line"]
pub type SBCL_R = crate::BitReader;
#[doc = "Field `SBCL` writer - Start of Basic Cycle Interrupt Line"]
pub type SBCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMCL` reader - Start of Matrix Cycle Interrupt Line"]
pub type SMCL_R = crate::BitReader;
#[doc = "Field `SMCL` writer - Start of Matrix Cycle Interrupt Line"]
pub type SMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSML` reader - Change of Synchronization Mode Interrupt Line"]
pub type CSML_R = crate::BitReader;
#[doc = "Field `CSML` writer - Change of Synchronization Mode Interrupt Line"]
pub type CSML_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOGL` reader - Start of Gap Interrupt Line"]
pub type SOGL_R = crate::BitReader;
#[doc = "Field `SOGL` writer - Start of Gap Interrupt Line"]
pub type SOGL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTMIL` reader - Register Time Mark Interrupt Line"]
pub type RTMIL_R = crate::BitReader;
#[doc = "Field `RTMIL` writer - Register Time Mark Interrupt Line"]
pub type RTMIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTMIL` reader - Trigger Time Mark Event Internal Interrupt Line"]
pub type TTMIL_R = crate::BitReader;
#[doc = "Field `TTMIL` writer - Trigger Time Mark Event Internal Interrupt Line"]
pub type TTMIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEL` reader - Stop Watch Event Interrupt Line"]
pub type SWEL_R = crate::BitReader;
#[doc = "Field `SWEL` writer - Stop Watch Event Interrupt Line"]
pub type SWEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTWL` reader - Global Time Wrap Interrupt Line"]
pub type GTWL_R = crate::BitReader;
#[doc = "Field `GTWL` writer - Global Time Wrap Interrupt Line"]
pub type GTWL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTDL` reader - Global Time Discontinuity Interrupt Line"]
pub type GTDL_R = crate::BitReader;
#[doc = "Field `GTDL` writer - Global Time Discontinuity Interrupt Line"]
pub type GTDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTEL` reader - Global Time Error Interrupt Line"]
pub type GTEL_R = crate::BitReader;
#[doc = "Field `GTEL` writer - Global Time Error Interrupt Line"]
pub type GTEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUL` reader - Tx Count Underflow Interrupt Line"]
pub type TXUL_R = crate::BitReader;
#[doc = "Field `TXUL` writer - Tx Count Underflow Interrupt Line"]
pub type TXUL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOL` reader - Tx Count Overflow Interrupt Line"]
pub type TXOL_R = crate::BitReader;
#[doc = "Field `TXOL` writer - Tx Count Overflow Interrupt Line"]
pub type TXOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1L` reader - Scheduling Error 1 Interrupt Line"]
pub type SE1L_R = crate::BitReader;
#[doc = "Field `SE1L` writer - Scheduling Error 1 Interrupt Line"]
pub type SE1L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2L` reader - Scheduling Error 2 Interrupt Line"]
pub type SE2L_R = crate::BitReader;
#[doc = "Field `SE2L` writer - Scheduling Error 2 Interrupt Line"]
pub type SE2L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELCL` reader - Change Error Level Interrupt Line"]
pub type ELCL_R = crate::BitReader;
#[doc = "Field `ELCL` writer - Change Error Level Interrupt Line"]
pub type ELCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWTGL` reader - Initialization Watch Trigger Interrupt Line"]
pub type IWTGL_R = crate::BitReader;
#[doc = "Field `IWTGL` writer - Initialization Watch Trigger Interrupt Line"]
pub type IWTGL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTL` reader - Watch Trigger Interrupt Line"]
pub type WTL_R = crate::BitReader;
#[doc = "Field `WTL` writer - Watch Trigger Interrupt Line"]
pub type WTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWL` reader - Application Watchdog Interrupt Line"]
pub type AWL_R = crate::BitReader;
#[doc = "Field `AWL` writer - Application Watchdog Interrupt Line"]
pub type AWL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERL` reader - Configuration Error Interrupt Line"]
pub type CERL_R = crate::BitReader;
#[doc = "Field `CERL` writer - Configuration Error Interrupt Line"]
pub type CERL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Line"]
    #[inline(always)]
    pub fn sbcl(&self) -> SBCL_R {
        SBCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Line"]
    #[inline(always)]
    pub fn smcl(&self) -> SMCL_R {
        SMCL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Line"]
    #[inline(always)]
    pub fn csml(&self) -> CSML_R {
        CSML_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Line"]
    #[inline(always)]
    pub fn sogl(&self) -> SOGL_R {
        SOGL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Line"]
    #[inline(always)]
    pub fn rtmil(&self) -> RTMIL_R {
        RTMIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Line"]
    #[inline(always)]
    pub fn ttmil(&self) -> TTMIL_R {
        TTMIL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Line"]
    #[inline(always)]
    pub fn swel(&self) -> SWEL_R {
        SWEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Line"]
    #[inline(always)]
    pub fn gtwl(&self) -> GTWL_R {
        GTWL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Line"]
    #[inline(always)]
    pub fn gtdl(&self) -> GTDL_R {
        GTDL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Line"]
    #[inline(always)]
    pub fn gtel(&self) -> GTEL_R {
        GTEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Line"]
    #[inline(always)]
    pub fn txul(&self) -> TXUL_R {
        TXUL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Line"]
    #[inline(always)]
    pub fn txol(&self) -> TXOL_R {
        TXOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Line"]
    #[inline(always)]
    pub fn se1l(&self) -> SE1L_R {
        SE1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Line"]
    #[inline(always)]
    pub fn se2l(&self) -> SE2L_R {
        SE2L_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Line"]
    #[inline(always)]
    pub fn elcl(&self) -> ELCL_R {
        ELCL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn iwtgl(&self) -> IWTGL_R {
        IWTGL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn wtl(&self) -> WTL_R {
        WTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Line"]
    #[inline(always)]
    pub fn cerl(&self) -> CERL_R {
        CERL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn sbcl(&mut self) -> SBCL_W<FDCAN_TTILSrs> {
        SBCL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn smcl(&mut self) -> SMCL_W<FDCAN_TTILSrs> {
        SMCL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn csml(&mut self) -> CSML_W<FDCAN_TTILSrs> {
        CSML_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn sogl(&mut self) -> SOGL_W<FDCAN_TTILSrs> {
        SOGL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rtmil(&mut self) -> RTMIL_W<FDCAN_TTILSrs> {
        RTMIL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn ttmil(&mut self) -> TTMIL_W<FDCAN_TTILSrs> {
        TTMIL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn swel(&mut self) -> SWEL_W<FDCAN_TTILSrs> {
        SWEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn gtwl(&mut self) -> GTWL_W<FDCAN_TTILSrs> {
        GTWL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn gtdl(&mut self) -> GTDL_W<FDCAN_TTILSrs> {
        GTDL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn gtel(&mut self) -> GTEL_W<FDCAN_TTILSrs> {
        GTEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn txul(&mut self) -> TXUL_W<FDCAN_TTILSrs> {
        TXUL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn txol(&mut self) -> TXOL_W<FDCAN_TTILSrs> {
        TXOL_W::new(self, 11)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn se1l(&mut self) -> SE1L_W<FDCAN_TTILSrs> {
        SE1L_W::new(self, 12)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn se2l(&mut self) -> SE2L_W<FDCAN_TTILSrs> {
        SE2L_W::new(self, 13)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn elcl(&mut self) -> ELCL_W<FDCAN_TTILSrs> {
        ELCL_W::new(self, 14)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn iwtgl(&mut self) -> IWTGL_W<FDCAN_TTILSrs> {
        IWTGL_W::new(self, 15)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn wtl(&mut self) -> WTL_W<FDCAN_TTILSrs> {
        WTL_W::new(self, 16)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn awl(&mut self) -> AWL_W<FDCAN_TTILSrs> {
        AWL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn cerl(&mut self) -> CERL_W<FDCAN_TTILSrs> {
        CERL_W::new(self, 18)
    }
}
#[doc = "FDCAN TT Interrupt Line Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttils::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttils::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTILSrs;
impl crate::RegisterSpec for FDCAN_TTILSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttils::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTILSrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttils::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTILSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTILS to value 0"]
impl crate::Resettable for FDCAN_TTILSrs {
    const RESET_VALUE: u32 = 0;
}
