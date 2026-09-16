///Register `MACCR` reader
pub type R = crate::R<MACCRrs>;
///Register `MACCR` writer
pub type W = crate::W<MACCRrs>;
///Field `RE` reader - RE
pub type RE_R = crate::BitReader;
///Field `RE` writer - RE
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE` reader - TE
pub type TE_R = crate::BitReader;
///Field `TE` writer - TE
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC` reader - DC
pub type DC_R = crate::BitReader;
///Field `DC` writer - DC
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BL` reader - BL
pub type BL_R = crate::FieldReader;
///Field `BL` writer - BL
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `APCS` reader - APCS
pub type APCS_R = crate::BitReader;
///Field `APCS` writer - APCS
pub type APCS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD` reader - RD
pub type RD_R = crate::BitReader;
///Field `RD` writer - RD
pub type RD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPCO` reader - IPCO
pub type IPCO_R = crate::BitReader;
///Field `IPCO` writer - IPCO
pub type IPCO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DM` reader - DM
pub type DM_R = crate::BitReader;
///Field `DM` writer - DM
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LM` reader - LM
pub type LM_R = crate::BitReader;
///Field `LM` writer - LM
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROD` reader - ROD
pub type ROD_R = crate::BitReader;
///Field `ROD` writer - ROD
pub type ROD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FES` reader - FES
pub type FES_R = crate::BitReader;
///Field `FES` writer - FES
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSD` reader - CSD
pub type CSD_R = crate::BitReader;
///Field `CSD` writer - CSD
pub type CSD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFG` reader - IFG
pub type IFG_R = crate::FieldReader;
///Field `IFG` writer - IFG
pub type IFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `JD` reader - JD
pub type JD_R = crate::BitReader;
///Field `JD` writer - JD
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WD` reader - WD
pub type WD_R = crate::BitReader;
///Field `WD` writer - WD
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSTF` reader - CSTF
pub type CSTF_R = crate::BitReader;
///Field `CSTF` writer - CSTF
pub type CSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - RE
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TE
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DC
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - BL
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - APCS
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - RD
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IPCO
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DM
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LM
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ROD
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - FES
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - CSD
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - IFG
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 22 - JD
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - WD
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CSTF
    #[inline(always)]
    pub fn cstf(&self) -> CSTF_R {
        CSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACCR")
            .field("re", &self.re())
            .field("te", &self.te())
            .field("dc", &self.dc())
            .field("bl", &self.bl())
            .field("apcs", &self.apcs())
            .field("rd", &self.rd())
            .field("ipco", &self.ipco())
            .field("dm", &self.dm())
            .field("lm", &self.lm())
            .field("rod", &self.rod())
            .field("fes", &self.fes())
            .field("csd", &self.csd())
            .field("ifg", &self.ifg())
            .field("jd", &self.jd())
            .field("wd", &self.wd())
            .field("cstf", &self.cstf())
            .finish()
    }
}
impl W {
    ///Bit 2 - RE
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, MACCRrs> {
        RE_W::new(self, 2)
    }
    ///Bit 3 - TE
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, MACCRrs> {
        TE_W::new(self, 3)
    }
    ///Bit 4 - DC
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<'_, MACCRrs> {
        DC_W::new(self, 4)
    }
    ///Bits 5:6 - BL
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<'_, MACCRrs> {
        BL_W::new(self, 5)
    }
    ///Bit 7 - APCS
    #[inline(always)]
    pub fn apcs(&mut self) -> APCS_W<'_, MACCRrs> {
        APCS_W::new(self, 7)
    }
    ///Bit 9 - RD
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W<'_, MACCRrs> {
        RD_W::new(self, 9)
    }
    ///Bit 10 - IPCO
    #[inline(always)]
    pub fn ipco(&mut self) -> IPCO_W<'_, MACCRrs> {
        IPCO_W::new(self, 10)
    }
    ///Bit 11 - DM
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<'_, MACCRrs> {
        DM_W::new(self, 11)
    }
    ///Bit 12 - LM
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<'_, MACCRrs> {
        LM_W::new(self, 12)
    }
    ///Bit 13 - ROD
    #[inline(always)]
    pub fn rod(&mut self) -> ROD_W<'_, MACCRrs> {
        ROD_W::new(self, 13)
    }
    ///Bit 14 - FES
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W<'_, MACCRrs> {
        FES_W::new(self, 14)
    }
    ///Bit 16 - CSD
    #[inline(always)]
    pub fn csd(&mut self) -> CSD_W<'_, MACCRrs> {
        CSD_W::new(self, 16)
    }
    ///Bits 17:19 - IFG
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W<'_, MACCRrs> {
        IFG_W::new(self, 17)
    }
    ///Bit 22 - JD
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W<'_, MACCRrs> {
        JD_W::new(self, 22)
    }
    ///Bit 23 - WD
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W<'_, MACCRrs> {
        WD_W::new(self, 23)
    }
    ///Bit 25 - CSTF
    #[inline(always)]
    pub fn cstf(&mut self) -> CSTF_W<'_, MACCRrs> {
        CSTF_W::new(self, 25)
    }
}
/**Ethernet MAC configuration register

You can [`read`](crate::Reg::read) this register and get [`maccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_MAC:MACCR)*/
pub struct MACCRrs;
impl crate::RegisterSpec for MACCRrs {
    type Ux = u32;
}
///`read()` method returns [`maccr::R`](R) reader structure
impl crate::Readable for MACCRrs {}
///`write(|w| ..)` method takes [`maccr::W`](W) writer structure
impl crate::Writable for MACCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACCR to value 0x8000
impl crate::Resettable for MACCRrs {
    const RESET_VALUE: u32 = 0x8000;
}
