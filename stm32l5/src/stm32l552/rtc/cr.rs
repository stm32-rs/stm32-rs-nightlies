///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `WUCKSEL` reader - WUCKSEL
pub type WUCKSEL_R = crate::FieldReader;
///Field `WUCKSEL` writer - WUCKSEL
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TSEDGE` reader - TSEDGE
pub type TSEDGE_R = crate::BitReader;
///Field `TSEDGE` writer - TSEDGE
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REFCKON` reader - REFCKON
pub type REFCKON_R = crate::BitReader;
///Field `REFCKON` writer - REFCKON
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYPSHAD` reader - BYPSHAD
pub type BYPSHAD_R = crate::BitReader;
///Field `BYPSHAD` writer - BYPSHAD
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMT` reader - FMT
pub type FMT_R = crate::BitReader;
///Field `FMT` writer - FMT
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAE` reader - ALRAE
pub type ALRAE_R = crate::BitReader;
///Field `ALRAE` writer - ALRAE
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBE` reader - ALRBE
pub type ALRBE_R = crate::BitReader;
///Field `ALRBE` writer - ALRBE
pub type ALRBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTE` reader - WUTE
pub type WUTE_R = crate::BitReader;
///Field `WUTE` writer - WUTE
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSE` reader - TSE
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - TSE
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAIE` reader - ALRAIE
pub type ALRAIE_R = crate::BitReader;
///Field `ALRAIE` writer - ALRAIE
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBIE` reader - ALRBIE
pub type ALRBIE_R = crate::BitReader;
///Field `ALRBIE` writer - ALRBIE
pub type ALRBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTIE` reader - WUTIE
pub type WUTIE_R = crate::BitReader;
///Field `WUTIE` writer - WUTIE
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSIE` reader - TSIE
pub type TSIE_R = crate::BitReader;
///Field `TSIE` writer - TSIE
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD1H` reader - ADD1H
pub type ADD1H_R = crate::BitReader;
///Field `ADD1H` writer - ADD1H
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUB1H` reader - SUB1H
pub type SUB1H_R = crate::BitReader;
///Field `SUB1H` writer - SUB1H
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKP` reader - BKP
pub type BKP_R = crate::BitReader;
///Field `BKP` writer - BKP
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COSEL` reader - COSEL
pub type COSEL_R = crate::BitReader;
///Field `COSEL` writer - COSEL
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POL` reader - POL
pub type POL_R = crate::BitReader;
///Field `POL` writer - POL
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSEL` reader - OSEL
pub type OSEL_R = crate::FieldReader;
///Field `OSEL` writer - OSEL
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COE` reader - COE
pub type COE_R = crate::BitReader;
///Field `COE` writer - COE
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITSE` reader - ITSE
pub type ITSE_R = crate::BitReader;
///Field `ITSE` writer - ITSE
pub type ITSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPTS` reader - TAMPTS
pub type TAMPTS_R = crate::BitReader;
///Field `TAMPTS` writer - TAMPTS
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPOE` reader - TAMPOE
pub type TAMPOE_R = crate::BitReader;
///Field `TAMPOE` writer - TAMPOE
pub type TAMPOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPALRM_PU` reader - TAMPALRM_PU
pub type TAMPALRM_PU_R = crate::BitReader;
///Field `TAMPALRM_PU` writer - TAMPALRM_PU
pub type TAMPALRM_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPALRM_TYPE` reader - TAMPALRM_TYPE
pub type TAMPALRM_TYPE_R = crate::BitReader;
///Field `TAMPALRM_TYPE` writer - TAMPALRM_TYPE
pub type TAMPALRM_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT2EN` reader - OUT2EN
pub type OUT2EN_R = crate::BitReader;
///Field `OUT2EN` writer - OUT2EN
pub type OUT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - WUCKSEL
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - TSEDGE
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - REFCKON
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BYPSHAD
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FMT
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - ALRAE
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ALRBE
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - WUTE
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TSE
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ALRAIE
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ALRBIE
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - WUTIE
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TSIE
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ADD1H
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SUB1H
    #[inline(always)]
    pub fn sub1h(&self) -> SUB1H_R {
        SUB1H_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - BKP
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - COSEL
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - POL
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - OSEL
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - COE
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ITSE
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TAMPTS
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - TAMPOE
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 29 - TAMPALRM_PU
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TAMPALRM_TYPE
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - OUT2EN
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("wucksel", &self.wucksel())
            .field("tsedge", &self.tsedge())
            .field("refckon", &self.refckon())
            .field("bypshad", &self.bypshad())
            .field("fmt", &self.fmt())
            .field("alrae", &self.alrae())
            .field("alrbe", &self.alrbe())
            .field("wute", &self.wute())
            .field("tse", &self.tse())
            .field("alraie", &self.alraie())
            .field("alrbie", &self.alrbie())
            .field("wutie", &self.wutie())
            .field("tsie", &self.tsie())
            .field("add1h", &self.add1h())
            .field("sub1h", &self.sub1h())
            .field("bkp", &self.bkp())
            .field("cosel", &self.cosel())
            .field("pol", &self.pol())
            .field("osel", &self.osel())
            .field("coe", &self.coe())
            .field("itse", &self.itse())
            .field("tampts", &self.tampts())
            .field("tampoe", &self.tampoe())
            .field("tampalrm_pu", &self.tampalrm_pu())
            .field("tampalrm_type", &self.tampalrm_type())
            .field("out2en", &self.out2en())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - WUCKSEL
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<CRrs> {
        WUCKSEL_W::new(self, 0)
    }
    ///Bit 3 - TSEDGE
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<CRrs> {
        TSEDGE_W::new(self, 3)
    }
    ///Bit 4 - REFCKON
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<CRrs> {
        REFCKON_W::new(self, 4)
    }
    ///Bit 5 - BYPSHAD
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BYPSHAD_W<CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    ///Bit 6 - FMT
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<CRrs> {
        FMT_W::new(self, 6)
    }
    ///Bit 8 - ALRAE
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> ALRAE_W<CRrs> {
        ALRAE_W::new(self, 8)
    }
    ///Bit 9 - ALRBE
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> ALRBE_W<CRrs> {
        ALRBE_W::new(self, 9)
    }
    ///Bit 10 - WUTE
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<CRrs> {
        WUTE_W::new(self, 10)
    }
    ///Bit 11 - TSE
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<CRrs> {
        TSE_W::new(self, 11)
    }
    ///Bit 12 - ALRAIE
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> ALRAIE_W<CRrs> {
        ALRAIE_W::new(self, 12)
    }
    ///Bit 13 - ALRBIE
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> ALRBIE_W<CRrs> {
        ALRBIE_W::new(self, 13)
    }
    ///Bit 14 - WUTIE
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WUTIE_W<CRrs> {
        WUTIE_W::new(self, 14)
    }
    ///Bit 15 - TSIE
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<CRrs> {
        TSIE_W::new(self, 15)
    }
    ///Bit 16 - ADD1H
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<CRrs> {
        ADD1H_W::new(self, 16)
    }
    ///Bit 17 - SUB1H
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<CRrs> {
        SUB1H_W::new(self, 17)
    }
    ///Bit 18 - BKP
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<CRrs> {
        BKP_W::new(self, 18)
    }
    ///Bit 19 - COSEL
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<CRrs> {
        COSEL_W::new(self, 19)
    }
    ///Bit 20 - POL
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CRrs> {
        POL_W::new(self, 20)
    }
    ///Bits 21:22 - OSEL
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<CRrs> {
        OSEL_W::new(self, 21)
    }
    ///Bit 23 - COE
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<CRrs> {
        COE_W::new(self, 23)
    }
    ///Bit 24 - ITSE
    #[inline(always)]
    #[must_use]
    pub fn itse(&mut self) -> ITSE_W<CRrs> {
        ITSE_W::new(self, 24)
    }
    ///Bit 25 - TAMPTS
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TAMPTS_W<CRrs> {
        TAMPTS_W::new(self, 25)
    }
    ///Bit 26 - TAMPOE
    #[inline(always)]
    #[must_use]
    pub fn tampoe(&mut self) -> TAMPOE_W<CRrs> {
        TAMPOE_W::new(self, 26)
    }
    ///Bit 29 - TAMPALRM_PU
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<CRrs> {
        TAMPALRM_PU_W::new(self, 29)
    }
    ///Bit 30 - TAMPALRM_TYPE
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<CRrs> {
        TAMPALRM_TYPE_W::new(self, 30)
    }
    ///Bit 31 - OUT2EN
    #[inline(always)]
    #[must_use]
    pub fn out2en(&mut self) -> OUT2EN_W<CRrs> {
        OUT2EN_W::new(self, 31)
    }
}
/**RTC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#RTC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
