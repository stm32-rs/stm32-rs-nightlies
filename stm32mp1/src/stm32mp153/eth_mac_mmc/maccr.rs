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
///Field `PRELEN` reader - PRELEN
pub type PRELEN_R = crate::FieldReader;
///Field `PRELEN` writer - PRELEN
pub type PRELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DC` reader - DC
pub type DC_R = crate::BitReader;
///Field `DC` writer - DC
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BL` reader - BL
pub type BL_R = crate::FieldReader;
///Field `BL` writer - BL
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DR` reader - DR
pub type DR_R = crate::BitReader;
///Field `DR` writer - DR
pub type DR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCRS` reader - DCRS
pub type DCRS_R = crate::BitReader;
///Field `DCRS` writer - DCRS
pub type DCRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DO` reader - DO
pub type DO_R = crate::BitReader;
///Field `DO` writer - DO
pub type DO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECRSFD` reader - ECRSFD
pub type ECRSFD_R = crate::BitReader;
///Field `ECRSFD` writer - ECRSFD
pub type ECRSFD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LM` reader - LM
pub type LM_R = crate::BitReader;
///Field `LM` writer - LM
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DM` reader - DM
pub type DM_R = crate::BitReader;
///Field `DM` writer - DM
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FES` reader - FES
pub type FES_R = crate::BitReader;
///Field `FES` writer - FES
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS` reader - PS
pub type PS_R = crate::BitReader;
///Field `PS` writer - PS
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JE` reader - JE
pub type JE_R = crate::BitReader;
///Field `JE` writer - JE
pub type JE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JD` reader - JD
pub type JD_R = crate::BitReader;
///Field `JD` writer - JD
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BE` reader - BE
pub type BE_R = crate::BitReader;
///Field `BE` writer - BE
pub type BE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WD` reader - WD
pub type WD_R = crate::BitReader;
///Field `WD` writer - WD
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACS` reader - ACS
pub type ACS_R = crate::BitReader;
///Field `ACS` writer - ACS
pub type ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CST` reader - CST
pub type CST_R = crate::BitReader;
///Field `CST` writer - CST
pub type CST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S2KP` reader - S2KP
pub type S2KP_R = crate::BitReader;
///Field `S2KP` writer - S2KP
pub type S2KP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPSLCE` reader - GPSLCE
pub type GPSLCE_R = crate::BitReader;
///Field `GPSLCE` writer - GPSLCE
pub type GPSLCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPG` reader - IPG
pub type IPG_R = crate::FieldReader;
///Field `IPG` writer - IPG
pub type IPG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IPC` reader - IPC
pub type IPC_R = crate::BitReader;
///Field `IPC` writer - IPC
pub type IPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SARC` reader - SARC
pub type SARC_R = crate::FieldReader;
///Field `SARC` writer - SARC
pub type SARC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ARPEN` reader - ARPEN
pub type ARPEN_R = crate::BitReader;
///Field `ARPEN` writer - ARPEN
pub type ARPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RE
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TE
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - PRELEN
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new(((self.bits >> 2) & 3) as u8)
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
    ///Bit 8 - DR
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DCRS
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DO
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ECRSFD
    #[inline(always)]
    pub fn ecrsfd(&self) -> ECRSFD_R {
        ECRSFD_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LM
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DM
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - FES
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PS
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - JE
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - JD
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - BE
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - WD
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ACS
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CST
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - S2KP
    #[inline(always)]
    pub fn s2kp(&self) -> S2KP_R {
        S2KP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - GPSLCE
    #[inline(always)]
    pub fn gpslce(&self) -> GPSLCE_R {
        GPSLCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - IPG
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - IPC
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:30 - SARC
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - ARPEN
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACCR")
            .field("re", &self.re())
            .field("te", &self.te())
            .field("prelen", &self.prelen())
            .field("dc", &self.dc())
            .field("bl", &self.bl())
            .field("dr", &self.dr())
            .field("dcrs", &self.dcrs())
            .field("do_", &self.do_())
            .field("ecrsfd", &self.ecrsfd())
            .field("lm", &self.lm())
            .field("dm", &self.dm())
            .field("fes", &self.fes())
            .field("ps", &self.ps())
            .field("je", &self.je())
            .field("jd", &self.jd())
            .field("be", &self.be())
            .field("wd", &self.wd())
            .field("acs", &self.acs())
            .field("cst", &self.cst())
            .field("s2kp", &self.s2kp())
            .field("gpslce", &self.gpslce())
            .field("ipg", &self.ipg())
            .field("ipc", &self.ipc())
            .field("sarc", &self.sarc())
            .field("arpen", &self.arpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - RE
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, MACCRrs> {
        RE_W::new(self, 0)
    }
    ///Bit 1 - TE
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, MACCRrs> {
        TE_W::new(self, 1)
    }
    ///Bits 2:3 - PRELEN
    #[inline(always)]
    pub fn prelen(&mut self) -> PRELEN_W<'_, MACCRrs> {
        PRELEN_W::new(self, 2)
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
    ///Bit 8 - DR
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, MACCRrs> {
        DR_W::new(self, 8)
    }
    ///Bit 9 - DCRS
    #[inline(always)]
    pub fn dcrs(&mut self) -> DCRS_W<'_, MACCRrs> {
        DCRS_W::new(self, 9)
    }
    ///Bit 10 - DO
    #[inline(always)]
    pub fn do_(&mut self) -> DO_W<'_, MACCRrs> {
        DO_W::new(self, 10)
    }
    ///Bit 11 - ECRSFD
    #[inline(always)]
    pub fn ecrsfd(&mut self) -> ECRSFD_W<'_, MACCRrs> {
        ECRSFD_W::new(self, 11)
    }
    ///Bit 12 - LM
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<'_, MACCRrs> {
        LM_W::new(self, 12)
    }
    ///Bit 13 - DM
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<'_, MACCRrs> {
        DM_W::new(self, 13)
    }
    ///Bit 14 - FES
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W<'_, MACCRrs> {
        FES_W::new(self, 14)
    }
    ///Bit 15 - PS
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, MACCRrs> {
        PS_W::new(self, 15)
    }
    ///Bit 16 - JE
    #[inline(always)]
    pub fn je(&mut self) -> JE_W<'_, MACCRrs> {
        JE_W::new(self, 16)
    }
    ///Bit 17 - JD
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W<'_, MACCRrs> {
        JD_W::new(self, 17)
    }
    ///Bit 18 - BE
    #[inline(always)]
    pub fn be(&mut self) -> BE_W<'_, MACCRrs> {
        BE_W::new(self, 18)
    }
    ///Bit 19 - WD
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W<'_, MACCRrs> {
        WD_W::new(self, 19)
    }
    ///Bit 20 - ACS
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W<'_, MACCRrs> {
        ACS_W::new(self, 20)
    }
    ///Bit 21 - CST
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W<'_, MACCRrs> {
        CST_W::new(self, 21)
    }
    ///Bit 22 - S2KP
    #[inline(always)]
    pub fn s2kp(&mut self) -> S2KP_W<'_, MACCRrs> {
        S2KP_W::new(self, 22)
    }
    ///Bit 23 - GPSLCE
    #[inline(always)]
    pub fn gpslce(&mut self) -> GPSLCE_W<'_, MACCRrs> {
        GPSLCE_W::new(self, 23)
    }
    ///Bits 24:26 - IPG
    #[inline(always)]
    pub fn ipg(&mut self) -> IPG_W<'_, MACCRrs> {
        IPG_W::new(self, 24)
    }
    ///Bit 27 - IPC
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W<'_, MACCRrs> {
        IPC_W::new(self, 27)
    }
    ///Bits 28:30 - SARC
    #[inline(always)]
    pub fn sarc(&mut self) -> SARC_W<'_, MACCRrs> {
        SARC_W::new(self, 28)
    }
    ///Bit 31 - ARPEN
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W<'_, MACCRrs> {
        ARPEN_W::new(self, 31)
    }
}
/**The MAC Configuration Register establishes the operating mode of the MAC.

You can [`read`](crate::Reg::read) this register and get [`maccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACCR)*/
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
