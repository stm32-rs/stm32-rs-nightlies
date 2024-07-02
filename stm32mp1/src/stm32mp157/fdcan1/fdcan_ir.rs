///Register `FDCAN_IR` reader
pub type R = crate::R<FDCAN_IRrs>;
///Register `FDCAN_IR` writer
pub type W = crate::W<FDCAN_IRrs>;
///Field `RF0N` reader - RF0N
pub type RF0N_R = crate::BitReader;
///Field `RF0N` writer - RF0N
pub type RF0N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0W` reader - RF0W
pub type RF0W_R = crate::BitReader;
///Field `RF0W` writer - RF0W
pub type RF0W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0F` reader - RF0F
pub type RF0F_R = crate::BitReader;
///Field `RF0F` writer - RF0F
pub type RF0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0L` reader - RF0L
pub type RF0L_R = crate::BitReader;
///Field `RF0L` writer - RF0L
pub type RF0L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1N` reader - RF1N
pub type RF1N_R = crate::BitReader;
///Field `RF1N` writer - RF1N
pub type RF1N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1W` reader - RF1W
pub type RF1W_R = crate::BitReader;
///Field `RF1W` writer - RF1W
pub type RF1W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1F` reader - RF1F
pub type RF1F_R = crate::BitReader;
///Field `RF1F` writer - RF1F
pub type RF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1L` reader - RF1L
pub type RF1L_R = crate::BitReader;
///Field `RF1L` writer - RF1L
pub type RF1L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPM` reader - HPM
pub type HPM_R = crate::BitReader;
///Field `HPM` writer - HPM
pub type HPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC` reader - TC
pub type TC_R = crate::BitReader;
///Field `TC` writer - TC
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCF` reader - TCF
pub type TCF_R = crate::BitReader;
///Field `TCF` writer - TCF
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFE` reader - TFE
pub type TFE_R = crate::BitReader;
///Field `TFE` writer - TFE
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFN` reader - TEFN
pub type TEFN_R = crate::BitReader;
///Field `TEFN` writer - TEFN
pub type TEFN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFW` reader - TEFW
pub type TEFW_R = crate::BitReader;
///Field `TEFW` writer - TEFW
pub type TEFW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFF` reader - TEFF
pub type TEFF_R = crate::BitReader;
///Field `TEFF` writer - TEFF
pub type TEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFL` reader - TEFL
pub type TEFL_R = crate::BitReader;
///Field `TEFL` writer - TEFL
pub type TEFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSW` reader - TSW
pub type TSW_R = crate::BitReader;
///Field `TSW` writer - TSW
pub type TSW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRAF` reader - MRAF
pub type MRAF_R = crate::BitReader;
///Field `MRAF` writer - MRAF
pub type MRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOO` reader - TOO
pub type TOO_R = crate::BitReader;
///Field `TOO` writer - TOO
pub type TOO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRX` reader - DRX
pub type DRX_R = crate::BitReader;
///Field `DRX` writer - DRX
pub type DRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELO` reader - ELO
pub type ELO_R = crate::BitReader;
///Field `ELO` writer - ELO
pub type ELO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EP` reader - EP
pub type EP_R = crate::BitReader;
///Field `EP` writer - EP
pub type EP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EW` reader - EW
pub type EW_R = crate::BitReader;
///Field `EW` writer - EW
pub type EW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BO` reader - BO
pub type BO_R = crate::BitReader;
///Field `BO` writer - BO
pub type BO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDI` reader - WDI
pub type WDI_R = crate::BitReader;
///Field `WDI` writer - WDI
pub type WDI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEA` reader - PEA
pub type PEA_R = crate::BitReader;
///Field `PEA` writer - PEA
pub type PEA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PED` reader - PED
pub type PED_R = crate::BitReader;
///Field `PED` writer - PED
pub type PED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARA` reader - ARA
pub type ARA_R = crate::BitReader;
///Field `ARA` writer - ARA
pub type ARA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RF0N
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RF0W
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RF0F
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RF0L
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RF1N
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RF1W
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RF1F
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RF1L
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HPM
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TCF
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TFE
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TEFN
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TEFW
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TEFF
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TEFL
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TSW
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - MRAF
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TOO
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DRX
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - ELO
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - EP
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - EW
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - BO
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - WDI
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - PEA
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PED
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ARA
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_IR")
            .field("rf0n", &self.rf0n())
            .field("rf0w", &self.rf0w())
            .field("rf0f", &self.rf0f())
            .field("rf0l", &self.rf0l())
            .field("rf1n", &self.rf1n())
            .field("rf1w", &self.rf1w())
            .field("rf1f", &self.rf1f())
            .field("rf1l", &self.rf1l())
            .field("hpm", &self.hpm())
            .field("tc", &self.tc())
            .field("tcf", &self.tcf())
            .field("tfe", &self.tfe())
            .field("tefn", &self.tefn())
            .field("tefw", &self.tefw())
            .field("teff", &self.teff())
            .field("tefl", &self.tefl())
            .field("tsw", &self.tsw())
            .field("mraf", &self.mraf())
            .field("too", &self.too())
            .field("drx", &self.drx())
            .field("elo", &self.elo())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("wdi", &self.wdi())
            .field("pea", &self.pea())
            .field("ped", &self.ped())
            .field("ara", &self.ara())
            .finish()
    }
}
impl W {
    ///Bit 0 - RF0N
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> RF0N_W<FDCAN_IRrs> {
        RF0N_W::new(self, 0)
    }
    ///Bit 1 - RF0W
    #[inline(always)]
    #[must_use]
    pub fn rf0w(&mut self) -> RF0W_W<FDCAN_IRrs> {
        RF0W_W::new(self, 1)
    }
    ///Bit 2 - RF0F
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> RF0F_W<FDCAN_IRrs> {
        RF0F_W::new(self, 2)
    }
    ///Bit 3 - RF0L
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<FDCAN_IRrs> {
        RF0L_W::new(self, 3)
    }
    ///Bit 4 - RF1N
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> RF1N_W<FDCAN_IRrs> {
        RF1N_W::new(self, 4)
    }
    ///Bit 5 - RF1W
    #[inline(always)]
    #[must_use]
    pub fn rf1w(&mut self) -> RF1W_W<FDCAN_IRrs> {
        RF1W_W::new(self, 5)
    }
    ///Bit 6 - RF1F
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> RF1F_W<FDCAN_IRrs> {
        RF1F_W::new(self, 6)
    }
    ///Bit 7 - RF1L
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> RF1L_W<FDCAN_IRrs> {
        RF1L_W::new(self, 7)
    }
    ///Bit 8 - HPM
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HPM_W<FDCAN_IRrs> {
        HPM_W::new(self, 8)
    }
    ///Bit 9 - TC
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<FDCAN_IRrs> {
        TC_W::new(self, 9)
    }
    ///Bit 10 - TCF
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<FDCAN_IRrs> {
        TCF_W::new(self, 10)
    }
    ///Bit 11 - TFE
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<FDCAN_IRrs> {
        TFE_W::new(self, 11)
    }
    ///Bit 12 - TEFN
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TEFN_W<FDCAN_IRrs> {
        TEFN_W::new(self, 12)
    }
    ///Bit 13 - TEFW
    #[inline(always)]
    #[must_use]
    pub fn tefw(&mut self) -> TEFW_W<FDCAN_IRrs> {
        TEFW_W::new(self, 13)
    }
    ///Bit 14 - TEFF
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TEFF_W<FDCAN_IRrs> {
        TEFF_W::new(self, 14)
    }
    ///Bit 15 - TEFL
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<FDCAN_IRrs> {
        TEFL_W::new(self, 15)
    }
    ///Bit 16 - TSW
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TSW_W<FDCAN_IRrs> {
        TSW_W::new(self, 16)
    }
    ///Bit 17 - MRAF
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MRAF_W<FDCAN_IRrs> {
        MRAF_W::new(self, 17)
    }
    ///Bit 18 - TOO
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TOO_W<FDCAN_IRrs> {
        TOO_W::new(self, 18)
    }
    ///Bit 19 - DRX
    #[inline(always)]
    #[must_use]
    pub fn drx(&mut self) -> DRX_W<FDCAN_IRrs> {
        DRX_W::new(self, 19)
    }
    ///Bit 22 - ELO
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> ELO_W<FDCAN_IRrs> {
        ELO_W::new(self, 22)
    }
    ///Bit 23 - EP
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<FDCAN_IRrs> {
        EP_W::new(self, 23)
    }
    ///Bit 24 - EW
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<FDCAN_IRrs> {
        EW_W::new(self, 24)
    }
    ///Bit 25 - BO
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<FDCAN_IRrs> {
        BO_W::new(self, 25)
    }
    ///Bit 26 - WDI
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WDI_W<FDCAN_IRrs> {
        WDI_W::new(self, 26)
    }
    ///Bit 27 - PEA
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PEA_W<FDCAN_IRrs> {
        PEA_W::new(self, 27)
    }
    ///Bit 28 - PED
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<FDCAN_IRrs> {
        PED_W::new(self, 28)
    }
    ///Bit 29 - ARA
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> ARA_W<FDCAN_IRrs> {
        ARA_W::new(self, 29)
    }
}
/**The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:FDCAN_IR)*/
pub struct FDCAN_IRrs;
impl crate::RegisterSpec for FDCAN_IRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ir::R`](R) reader structure
impl crate::Readable for FDCAN_IRrs {}
///`write(|w| ..)` method takes [`fdcan_ir::W`](W) writer structure
impl crate::Writable for FDCAN_IRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_IR to value 0
impl crate::Resettable for FDCAN_IRrs {
    const RESET_VALUE: u32 = 0;
}
