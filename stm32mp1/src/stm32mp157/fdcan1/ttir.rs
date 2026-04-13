///Register `TTIR` reader
pub type R = crate::R<TTIRrs>;
///Register `TTIR` writer
pub type W = crate::W<TTIRrs>;
///Field `SBC` reader - SBC
pub type SBC_R = crate::BitReader;
///Field `SBC` writer - SBC
pub type SBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMC` reader - SMC
pub type SMC_R = crate::BitReader;
///Field `SMC` writer - SMC
pub type SMC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSM` reader - CSM
pub type CSM_R = crate::BitReader;
///Field `CSM` writer - CSM
pub type CSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOG` reader - SOG
pub type SOG_R = crate::BitReader;
///Field `SOG` writer - SOG
pub type SOG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTMI` reader - RTMI
pub type RTMI_R = crate::BitReader;
///Field `RTMI` writer - RTMI
pub type RTMI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTMI` reader - TTMI
pub type TTMI_R = crate::BitReader;
///Field `TTMI` writer - TTMI
pub type TTMI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWE` reader - SWE
pub type SWE_R = crate::BitReader;
///Field `SWE` writer - SWE
pub type SWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTW` reader - GTW
pub type GTW_R = crate::BitReader;
///Field `GTW` writer - GTW
pub type GTW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTD` reader - GTD
pub type GTD_R = crate::BitReader;
///Field `GTD` writer - GTD
pub type GTD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTE` reader - GTE
pub type GTE_R = crate::BitReader;
///Field `GTE` writer - GTE
pub type GTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXU` reader - TXU
pub type TXU_R = crate::BitReader;
///Field `TXU` writer - TXU
pub type TXU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXO` reader - TXO
pub type TXO_R = crate::BitReader;
///Field `TXO` writer - TXO
pub type TXO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE1` reader - SE1
pub type SE1_R = crate::BitReader;
///Field `SE1` writer - SE1
pub type SE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE2` reader - SE2
pub type SE2_R = crate::BitReader;
///Field `SE2` writer - SE2
pub type SE2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELC` reader - ELC
pub type ELC_R = crate::BitReader;
///Field `ELC` writer - ELC
pub type ELC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWTG` reader - IWTG
pub type IWTG_R = crate::BitReader;
///Field `IWTG` writer - IWTG
pub type IWTG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WT` reader - WT
pub type WT_R = crate::BitReader;
///Field `WT` writer - WT
pub type WT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AW` reader - AW
pub type AW_R = crate::BitReader;
///Field `AW` writer - AW
pub type AW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CER` reader - CER
pub type CER_R = crate::BitReader;
///Field `CER` writer - CER
pub type CER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SBC
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SMC
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CSM
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SOG
    #[inline(always)]
    pub fn sog(&self) -> SOG_R {
        SOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RTMI
    #[inline(always)]
    pub fn rtmi(&self) -> RTMI_R {
        RTMI_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TTMI
    #[inline(always)]
    pub fn ttmi(&self) -> TTMI_R {
        TTMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SWE
    #[inline(always)]
    pub fn swe(&self) -> SWE_R {
        SWE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTW
    #[inline(always)]
    pub fn gtw(&self) -> GTW_R {
        GTW_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTD
    #[inline(always)]
    pub fn gtd(&self) -> GTD_R {
        GTD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTE
    #[inline(always)]
    pub fn gte(&self) -> GTE_R {
        GTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TXU
    #[inline(always)]
    pub fn txu(&self) -> TXU_R {
        TXU_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TXO
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SE1
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SE2
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ELC
    #[inline(always)]
    pub fn elc(&self) -> ELC_R {
        ELC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - IWTG
    #[inline(always)]
    pub fn iwtg(&self) -> IWTG_R {
        IWTG_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - WT
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AW
    #[inline(always)]
    pub fn aw(&self) -> AW_R {
        AW_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CER
    #[inline(always)]
    pub fn cer(&self) -> CER_R {
        CER_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTIR")
            .field("sbc", &self.sbc())
            .field("smc", &self.smc())
            .field("csm", &self.csm())
            .field("sog", &self.sog())
            .field("rtmi", &self.rtmi())
            .field("ttmi", &self.ttmi())
            .field("swe", &self.swe())
            .field("gtw", &self.gtw())
            .field("gtd", &self.gtd())
            .field("gte", &self.gte())
            .field("txu", &self.txu())
            .field("txo", &self.txo())
            .field("se1", &self.se1())
            .field("se2", &self.se2())
            .field("elc", &self.elc())
            .field("iwtg", &self.iwtg())
            .field("wt", &self.wt())
            .field("aw", &self.aw())
            .field("cer", &self.cer())
            .finish()
    }
}
impl W {
    ///Bit 0 - SBC
    #[inline(always)]
    pub fn sbc(&mut self) -> SBC_W<'_, TTIRrs> {
        SBC_W::new(self, 0)
    }
    ///Bit 1 - SMC
    #[inline(always)]
    pub fn smc(&mut self) -> SMC_W<'_, TTIRrs> {
        SMC_W::new(self, 1)
    }
    ///Bit 2 - CSM
    #[inline(always)]
    pub fn csm(&mut self) -> CSM_W<'_, TTIRrs> {
        CSM_W::new(self, 2)
    }
    ///Bit 3 - SOG
    #[inline(always)]
    pub fn sog(&mut self) -> SOG_W<'_, TTIRrs> {
        SOG_W::new(self, 3)
    }
    ///Bit 4 - RTMI
    #[inline(always)]
    pub fn rtmi(&mut self) -> RTMI_W<'_, TTIRrs> {
        RTMI_W::new(self, 4)
    }
    ///Bit 5 - TTMI
    #[inline(always)]
    pub fn ttmi(&mut self) -> TTMI_W<'_, TTIRrs> {
        TTMI_W::new(self, 5)
    }
    ///Bit 6 - SWE
    #[inline(always)]
    pub fn swe(&mut self) -> SWE_W<'_, TTIRrs> {
        SWE_W::new(self, 6)
    }
    ///Bit 7 - GTW
    #[inline(always)]
    pub fn gtw(&mut self) -> GTW_W<'_, TTIRrs> {
        GTW_W::new(self, 7)
    }
    ///Bit 8 - GTD
    #[inline(always)]
    pub fn gtd(&mut self) -> GTD_W<'_, TTIRrs> {
        GTD_W::new(self, 8)
    }
    ///Bit 9 - GTE
    #[inline(always)]
    pub fn gte(&mut self) -> GTE_W<'_, TTIRrs> {
        GTE_W::new(self, 9)
    }
    ///Bit 10 - TXU
    #[inline(always)]
    pub fn txu(&mut self) -> TXU_W<'_, TTIRrs> {
        TXU_W::new(self, 10)
    }
    ///Bit 11 - TXO
    #[inline(always)]
    pub fn txo(&mut self) -> TXO_W<'_, TTIRrs> {
        TXO_W::new(self, 11)
    }
    ///Bit 12 - SE1
    #[inline(always)]
    pub fn se1(&mut self) -> SE1_W<'_, TTIRrs> {
        SE1_W::new(self, 12)
    }
    ///Bit 13 - SE2
    #[inline(always)]
    pub fn se2(&mut self) -> SE2_W<'_, TTIRrs> {
        SE2_W::new(self, 13)
    }
    ///Bit 14 - ELC
    #[inline(always)]
    pub fn elc(&mut self) -> ELC_W<'_, TTIRrs> {
        ELC_W::new(self, 14)
    }
    ///Bit 15 - IWTG
    #[inline(always)]
    pub fn iwtg(&mut self) -> IWTG_W<'_, TTIRrs> {
        IWTG_W::new(self, 15)
    }
    ///Bit 16 - WT
    #[inline(always)]
    pub fn wt(&mut self) -> WT_W<'_, TTIRrs> {
        WT_W::new(self, 16)
    }
    ///Bit 17 - AW
    #[inline(always)]
    pub fn aw(&mut self) -> AW_W<'_, TTIRrs> {
        AW_W::new(self, 17)
    }
    ///Bit 18 - CER
    #[inline(always)]
    pub fn cer(&mut self) -> CER_W<'_, TTIRrs> {
        CER_W::new(self, 18)
    }
}
/**The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register.

You can [`read`](crate::Reg::read) this register and get [`ttir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTIR)*/
pub struct TTIRrs;
impl crate::RegisterSpec for TTIRrs {
    type Ux = u32;
}
///`read()` method returns [`ttir::R`](R) reader structure
impl crate::Readable for TTIRrs {}
///`write(|w| ..)` method takes [`ttir::W`](W) writer structure
impl crate::Writable for TTIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTIR to value 0
impl crate::Resettable for TTIRrs {}
