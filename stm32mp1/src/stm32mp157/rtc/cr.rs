#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `WUCKSEL` reader - WUCKSEL"]
pub type WUCKSEL_R = crate::FieldReader;
#[doc = "Field `WUCKSEL` writer - WUCKSEL"]
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSEDGE` reader - TSEDGE"]
pub type TSEDGE_R = crate::BitReader;
#[doc = "Field `TSEDGE` writer - TSEDGE"]
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFCKON` reader - REFCKON"]
pub type REFCKON_R = crate::BitReader;
#[doc = "Field `REFCKON` writer - REFCKON"]
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSHAD` reader - BYPSHAD"]
pub type BYPSHAD_R = crate::BitReader;
#[doc = "Field `BYPSHAD` writer - BYPSHAD"]
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMT` reader - FMT"]
pub type FMT_R = crate::BitReader;
#[doc = "Field `FMT` writer - FMT"]
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAE` reader - ALRAE"]
pub type ALRAE_R = crate::BitReader;
#[doc = "Field `ALRAE` writer - ALRAE"]
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBE` reader - ALRBE"]
pub type ALRBE_R = crate::BitReader;
#[doc = "Field `ALRBE` writer - ALRBE"]
pub type ALRBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTE` reader - WUTE"]
pub type WUTE_R = crate::BitReader;
#[doc = "Field `WUTE` writer - WUTE"]
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - TSE"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - TSE"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAIE` reader - ALRAIE"]
pub type ALRAIE_R = crate::BitReader;
#[doc = "Field `ALRAIE` writer - ALRAIE"]
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBIE` reader - ALRBIE"]
pub type ALRBIE_R = crate::BitReader;
#[doc = "Field `ALRBIE` writer - ALRBIE"]
pub type ALRBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTIE` reader - WUTIE"]
pub type WUTIE_R = crate::BitReader;
#[doc = "Field `WUTIE` writer - WUTIE"]
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - TSIE"]
pub type TSIE_R = crate::BitReader;
#[doc = "Field `TSIE` writer - TSIE"]
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD1H` writer - ADD1H"]
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB1H` writer - SUB1H"]
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::BitReader;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COSEL` reader - COSEL"]
pub type COSEL_R = crate::BitReader;
#[doc = "Field `COSEL` writer - COSEL"]
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - POL"]
pub type POL_R = crate::BitReader;
#[doc = "Field `POL` writer - POL"]
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSEL` reader - OSEL"]
pub type OSEL_R = crate::FieldReader;
#[doc = "Field `OSEL` writer - OSEL"]
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COE` reader - COE"]
pub type COE_R = crate::BitReader;
#[doc = "Field `COE` writer - COE"]
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITSE` reader - ITSE"]
pub type ITSE_R = crate::BitReader;
#[doc = "Field `ITSE` writer - ITSE"]
pub type ITSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPTS` reader - TAMPTS"]
pub type TAMPTS_R = crate::BitReader;
#[doc = "Field `TAMPTS` writer - TAMPTS"]
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPOE` reader - TAMPOE"]
pub type TAMPOE_R = crate::BitReader;
#[doc = "Field `TAMPOE` writer - TAMPOE"]
pub type TAMPOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPALRM_PU` reader - TAMPALRM_PU"]
pub type TAMPALRM_PU_R = crate::BitReader;
#[doc = "Field `TAMPALRM_PU` writer - TAMPALRM_PU"]
pub type TAMPALRM_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPALRM_TYPE` reader - TAMPALRM_TYPE"]
pub type TAMPALRM_TYPE_R = crate::BitReader;
#[doc = "Field `TAMPALRM_TYPE` writer - TAMPALRM_TYPE"]
pub type TAMPALRM_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT2EN` reader - OUT2EN"]
pub type OUT2EN_R = crate::BitReader;
#[doc = "Field `OUT2EN` writer - OUT2EN"]
pub type OUT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - WUCKSEL"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - TSEDGE"]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REFCKON"]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BYPSHAD"]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FMT"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - ALRAE"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ALRBE"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WUTE"]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ALRAIE"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ALRBIE"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - WUTIE"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - COSEL"]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - OSEL"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - COE"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ITSE"]
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TAMPTS"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TAMPOE"]
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - TAMPALRM_PU"]
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TAMPALRM_TYPE"]
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OUT2EN"]
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WUCKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<CRrs> {
        WUCKSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - TSEDGE"]
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<CRrs> {
        TSEDGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - REFCKON"]
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<CRrs> {
        REFCKON_W::new(self, 4)
    }
    #[doc = "Bit 5 - BYPSHAD"]
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BYPSHAD_W<CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    #[doc = "Bit 6 - FMT"]
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<CRrs> {
        FMT_W::new(self, 6)
    }
    #[doc = "Bit 8 - ALRAE"]
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> ALRAE_W<CRrs> {
        ALRAE_W::new(self, 8)
    }
    #[doc = "Bit 9 - ALRBE"]
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> ALRBE_W<CRrs> {
        ALRBE_W::new(self, 9)
    }
    #[doc = "Bit 10 - WUTE"]
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<CRrs> {
        WUTE_W::new(self, 10)
    }
    #[doc = "Bit 11 - TSE"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<CRrs> {
        TSE_W::new(self, 11)
    }
    #[doc = "Bit 12 - ALRAIE"]
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> ALRAIE_W<CRrs> {
        ALRAIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - ALRBIE"]
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> ALRBIE_W<CRrs> {
        ALRBIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - WUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WUTIE_W<CRrs> {
        WUTIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - TSIE"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<CRrs> {
        TSIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - ADD1H"]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<CRrs> {
        ADD1H_W::new(self, 16)
    }
    #[doc = "Bit 17 - SUB1H"]
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<CRrs> {
        SUB1H_W::new(self, 17)
    }
    #[doc = "Bit 18 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<CRrs> {
        BKP_W::new(self, 18)
    }
    #[doc = "Bit 19 - COSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<CRrs> {
        COSEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - POL"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CRrs> {
        POL_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - OSEL"]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<CRrs> {
        OSEL_W::new(self, 21)
    }
    #[doc = "Bit 23 - COE"]
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<CRrs> {
        COE_W::new(self, 23)
    }
    #[doc = "Bit 24 - ITSE"]
    #[inline(always)]
    #[must_use]
    pub fn itse(&mut self) -> ITSE_W<CRrs> {
        ITSE_W::new(self, 24)
    }
    #[doc = "Bit 25 - TAMPTS"]
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TAMPTS_W<CRrs> {
        TAMPTS_W::new(self, 25)
    }
    #[doc = "Bit 26 - TAMPOE"]
    #[inline(always)]
    #[must_use]
    pub fn tampoe(&mut self) -> TAMPOE_W<CRrs> {
        TAMPOE_W::new(self, 26)
    }
    #[doc = "Bit 29 - TAMPALRM_PU"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<CRrs> {
        TAMPALRM_PU_W::new(self, 29)
    }
    #[doc = "Bit 30 - TAMPALRM_TYPE"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<CRrs> {
        TAMPALRM_TYPE_W::new(self, 30)
    }
    #[doc = "Bit 31 - OUT2EN"]
    #[inline(always)]
    #[must_use]
    pub fn out2en(&mut self) -> OUT2EN_W<CRrs> {
        OUT2EN_W::new(self, 31)
    }
}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
