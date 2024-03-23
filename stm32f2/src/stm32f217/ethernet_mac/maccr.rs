#[doc = "Register `MACCR` reader"]
pub type R = crate::R<MACCRrs>;
#[doc = "Register `MACCR` writer"]
pub type W = crate::W<MACCRrs>;
#[doc = "Field `RE` reader - RE"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - RE"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - TE"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - TE"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - DC"]
pub type DC_R = crate::BitReader;
#[doc = "Field `DC` writer - DC"]
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - BL"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - BL"]
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APCS` reader - APCS"]
pub type APCS_R = crate::BitReader;
#[doc = "Field `APCS` writer - APCS"]
pub type APCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - RD"]
pub type RD_R = crate::BitReader;
#[doc = "Field `RD` writer - RD"]
pub type RD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCO` reader - IPCO"]
pub type IPCO_R = crate::BitReader;
#[doc = "Field `IPCO` writer - IPCO"]
pub type IPCO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - DM"]
pub type DM_R = crate::BitReader;
#[doc = "Field `DM` writer - DM"]
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - LM"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - LM"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROD` reader - ROD"]
pub type ROD_R = crate::BitReader;
#[doc = "Field `ROD` writer - ROD"]
pub type ROD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - FES"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - FES"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSD` reader - CSD"]
pub type CSD_R = crate::BitReader;
#[doc = "Field `CSD` writer - CSD"]
pub type CSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - IFG"]
pub type IFG_R = crate::FieldReader;
#[doc = "Field `IFG` writer - IFG"]
pub type IFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JD` reader - JD"]
pub type JD_R = crate::BitReader;
#[doc = "Field `JD` writer - JD"]
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - WD"]
pub type WD_R = crate::BitReader;
#[doc = "Field `WD` writer - WD"]
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTF` reader - CSTF"]
pub type CSTF_R = crate::BitReader;
#[doc = "Field `CSTF` writer - CSTF"]
pub type CSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - RE"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TE"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DC"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - BL"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - APCS"]
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - RD"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPCO"]
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DM"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LM"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ROD"]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FES"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - CSD"]
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - IFG"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - JD"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - WD"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CSTF"]
    #[inline(always)]
    pub fn cstf(&self) -> CSTF_R {
        CSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RE"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<MACCRrs> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - TE"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<MACCRrs> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - DC"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<MACCRrs> {
        DC_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - BL"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<MACCRrs> {
        BL_W::new(self, 5)
    }
    #[doc = "Bit 7 - APCS"]
    #[inline(always)]
    #[must_use]
    pub fn apcs(&mut self) -> APCS_W<MACCRrs> {
        APCS_W::new(self, 7)
    }
    #[doc = "Bit 9 - RD"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<MACCRrs> {
        RD_W::new(self, 9)
    }
    #[doc = "Bit 10 - IPCO"]
    #[inline(always)]
    #[must_use]
    pub fn ipco(&mut self) -> IPCO_W<MACCRrs> {
        IPCO_W::new(self, 10)
    }
    #[doc = "Bit 11 - DM"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<MACCRrs> {
        DM_W::new(self, 11)
    }
    #[doc = "Bit 12 - LM"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<MACCRrs> {
        LM_W::new(self, 12)
    }
    #[doc = "Bit 13 - ROD"]
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<MACCRrs> {
        ROD_W::new(self, 13)
    }
    #[doc = "Bit 14 - FES"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<MACCRrs> {
        FES_W::new(self, 14)
    }
    #[doc = "Bit 16 - CSD"]
    #[inline(always)]
    #[must_use]
    pub fn csd(&mut self) -> CSD_W<MACCRrs> {
        CSD_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - IFG"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<MACCRrs> {
        IFG_W::new(self, 17)
    }
    #[doc = "Bit 22 - JD"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<MACCRrs> {
        JD_W::new(self, 22)
    }
    #[doc = "Bit 23 - WD"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<MACCRrs> {
        WD_W::new(self, 23)
    }
    #[doc = "Bit 25 - CSTF"]
    #[inline(always)]
    #[must_use]
    pub fn cstf(&mut self) -> CSTF_W<MACCRrs> {
        CSTF_W::new(self, 25)
    }
}
#[doc = "Ethernet MAC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACCRrs;
impl crate::RegisterSpec for MACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maccr::R`](R) reader structure"]
impl crate::Readable for MACCRrs {}
#[doc = "`write(|w| ..)` method takes [`maccr::W`](W) writer structure"]
impl crate::Writable for MACCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACCR to value 0x8000"]
impl crate::Resettable for MACCRrs {
    const RESET_VALUE: u32 = 0x8000;
}
