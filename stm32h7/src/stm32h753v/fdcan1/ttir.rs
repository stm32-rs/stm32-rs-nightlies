#[doc = "Register `TTIR` reader"]
pub type R = crate::R<TTIRrs>;
#[doc = "Register `TTIR` writer"]
pub type W = crate::W<TTIRrs>;
#[doc = "Field `SBC` reader - Start of Basic Cycle"]
pub type SBC_R = crate::BitReader;
#[doc = "Field `SBC` writer - Start of Basic Cycle"]
pub type SBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMC` reader - Start of Matrix Cycle"]
pub type SMC_R = crate::BitReader;
#[doc = "Field `SMC` writer - Start of Matrix Cycle"]
pub type SMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSM` reader - Change of Synchronization Mode"]
pub type CSM_R = crate::BitReader;
#[doc = "Field `CSM` writer - Change of Synchronization Mode"]
pub type CSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOG` reader - Start of Gap"]
pub type SOG_R = crate::BitReader;
#[doc = "Field `SOG` writer - Start of Gap"]
pub type SOG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTMI` reader - Register Time Mark Interrupt."]
pub type RTMI_R = crate::BitReader;
#[doc = "Field `RTMI` writer - Register Time Mark Interrupt."]
pub type RTMI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTMI` reader - Trigger Time Mark Event Internal"]
pub type TTMI_R = crate::BitReader;
#[doc = "Field `TTMI` writer - Trigger Time Mark Event Internal"]
pub type TTMI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWE` reader - Stop Watch Event"]
pub type SWE_R = crate::BitReader;
#[doc = "Field `SWE` writer - Stop Watch Event"]
pub type SWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTW` reader - Global Time Wrap"]
pub type GTW_R = crate::BitReader;
#[doc = "Field `GTW` writer - Global Time Wrap"]
pub type GTW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTD` reader - Global Time Discontinuity"]
pub type GTD_R = crate::BitReader;
#[doc = "Field `GTD` writer - Global Time Discontinuity"]
pub type GTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTE` reader - Global Time Error"]
pub type GTE_R = crate::BitReader;
#[doc = "Field `GTE` writer - Global Time Error"]
pub type GTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXU` reader - Tx Count Underflow"]
pub type TXU_R = crate::BitReader;
#[doc = "Field `TXU` writer - Tx Count Underflow"]
pub type TXU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXO` reader - Tx Count Overflow"]
pub type TXO_R = crate::BitReader;
#[doc = "Field `TXO` writer - Tx Count Overflow"]
pub type TXO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1` reader - Scheduling Error 1"]
pub type SE1_R = crate::BitReader;
#[doc = "Field `SE1` writer - Scheduling Error 1"]
pub type SE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2` reader - Scheduling Error 2"]
pub type SE2_R = crate::BitReader;
#[doc = "Field `SE2` writer - Scheduling Error 2"]
pub type SE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELC` reader - Error Level Changed."]
pub type ELC_R = crate::BitReader;
#[doc = "Field `ELC` writer - Error Level Changed."]
pub type ELC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWTG` reader - Initialization Watch Trigger"]
pub type IWTG_R = crate::BitReader;
#[doc = "Field `IWTG` writer - Initialization Watch Trigger"]
pub type IWTG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WT` reader - Watch Trigger"]
pub type WT_R = crate::BitReader;
#[doc = "Field `WT` writer - Watch Trigger"]
pub type WT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW` reader - Application Watchdog"]
pub type AW_R = crate::BitReader;
#[doc = "Field `AW` writer - Application Watchdog"]
pub type AW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CER` reader - Configuration Error"]
pub type CER_R = crate::BitReader;
#[doc = "Field `CER` writer - Configuration Error"]
pub type CER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle"]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Gap"]
    #[inline(always)]
    pub fn sog(&self) -> SOG_R {
        SOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt."]
    #[inline(always)]
    pub fn rtmi(&self) -> RTMI_R {
        RTMI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal"]
    #[inline(always)]
    pub fn ttmi(&self) -> TTMI_R {
        TTMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event"]
    #[inline(always)]
    pub fn swe(&self) -> SWE_R {
        SWE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap"]
    #[inline(always)]
    pub fn gtw(&self) -> GTW_R {
        GTW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity"]
    #[inline(always)]
    pub fn gtd(&self) -> GTD_R {
        GTD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Time Error"]
    #[inline(always)]
    pub fn gte(&self) -> GTE_R {
        GTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow"]
    #[inline(always)]
    pub fn txu(&self) -> TXU_R {
        TXU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1"]
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Level Changed."]
    #[inline(always)]
    pub fn elc(&self) -> ELC_R {
        ELC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger"]
    #[inline(always)]
    pub fn iwtg(&self) -> IWTG_R {
        IWTG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog"]
    #[inline(always)]
    pub fn aw(&self) -> AW_R {
        AW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration Error"]
    #[inline(always)]
    pub fn cer(&self) -> CER_R {
        CER_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SBC_W<TTIRrs> {
        SBC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn smc(&mut self) -> SMC_W<TTIRrs> {
        SMC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn csm(&mut self) -> CSM_W<TTIRrs> {
        CSM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Gap"]
    #[inline(always)]
    #[must_use]
    pub fn sog(&mut self) -> SOG_W<TTIRrs> {
        SOG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rtmi(&mut self) -> RTMI_W<TTIRrs> {
        RTMI_W::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal"]
    #[inline(always)]
    #[must_use]
    pub fn ttmi(&mut self) -> TTMI_W<TTIRrs> {
        TTMI_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stop Watch Event"]
    #[inline(always)]
    #[must_use]
    pub fn swe(&mut self) -> SWE_W<TTIRrs> {
        SWE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Global Time Wrap"]
    #[inline(always)]
    #[must_use]
    pub fn gtw(&mut self) -> GTW_W<TTIRrs> {
        GTW_W::new(self, 7)
    }
    #[doc = "Bit 8 - Global Time Discontinuity"]
    #[inline(always)]
    #[must_use]
    pub fn gtd(&mut self) -> GTD_W<TTIRrs> {
        GTD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Global Time Error"]
    #[inline(always)]
    #[must_use]
    pub fn gte(&mut self) -> GTE_W<TTIRrs> {
        GTE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx Count Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn txu(&mut self) -> TXU_W<TTIRrs> {
        TXU_W::new(self, 10)
    }
    #[doc = "Bit 11 - Tx Count Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn txo(&mut self) -> TXO_W<TTIRrs> {
        TXO_W::new(self, 11)
    }
    #[doc = "Bit 12 - Scheduling Error 1"]
    #[inline(always)]
    #[must_use]
    pub fn se1(&mut self) -> SE1_W<TTIRrs> {
        SE1_W::new(self, 12)
    }
    #[doc = "Bit 13 - Scheduling Error 2"]
    #[inline(always)]
    #[must_use]
    pub fn se2(&mut self) -> SE2_W<TTIRrs> {
        SE2_W::new(self, 13)
    }
    #[doc = "Bit 14 - Error Level Changed."]
    #[inline(always)]
    #[must_use]
    pub fn elc(&mut self) -> ELC_W<TTIRrs> {
        ELC_W::new(self, 14)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn iwtg(&mut self) -> IWTG_W<TTIRrs> {
        IWTG_W::new(self, 15)
    }
    #[doc = "Bit 16 - Watch Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WT_W<TTIRrs> {
        WT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Application Watchdog"]
    #[inline(always)]
    #[must_use]
    pub fn aw(&mut self) -> AW_W<TTIRrs> {
        AW_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configuration Error"]
    #[inline(always)]
    #[must_use]
    pub fn cer(&mut self) -> CER_W<TTIRrs> {
        CER_W::new(self, 18)
    }
}
#[doc = "FDCAN TT Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTIRrs;
impl crate::RegisterSpec for TTIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttir::R`](R) reader structure"]
impl crate::Readable for TTIRrs {}
#[doc = "`write(|w| ..)` method takes [`ttir::W`](W) writer structure"]
impl crate::Writable for TTIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TTIR to value 0"]
impl crate::Resettable for TTIRrs {
    const RESET_VALUE: u32 = 0;
}
