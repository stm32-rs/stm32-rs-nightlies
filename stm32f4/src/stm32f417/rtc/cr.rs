///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `WCKSEL` reader - Wakeup clock selection
pub type WCKSEL_R = crate::FieldReader;
///Field `WCKSEL` writer - Wakeup clock selection
pub type WCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TSEDGE` reader - Time-stamp event active edge
pub type TSEDGE_R = crate::BitReader;
///Field `TSEDGE` writer - Time-stamp event active edge
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REFCKON` reader - Reference clock detection enable (50 or 60 Hz)
pub type REFCKON_R = crate::BitReader;
///Field `REFCKON` writer - Reference clock detection enable (50 or 60 Hz)
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMT` reader - Hour format
pub type FMT_R = crate::BitReader;
///Field `FMT` writer - Hour format
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCE` reader - Coarse digital calibration enable
pub type DCE_R = crate::BitReader;
///Field `DCE` writer - Coarse digital calibration enable
pub type DCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAE` reader - Alarm A enable
pub type ALRAE_R = crate::BitReader;
///Field `ALRAE` writer - Alarm A enable
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBE` reader - Alarm B enable
pub type ALRBE_R = crate::BitReader;
///Field `ALRBE` writer - Alarm B enable
pub type ALRBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTE` reader - Wakeup timer enable
pub type WUTE_R = crate::BitReader;
///Field `WUTE` writer - Wakeup timer enable
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSE` reader - Time stamp enable
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - Time stamp enable
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAIE` reader - Alarm A interrupt enable
pub type ALRAIE_R = crate::BitReader;
///Field `ALRAIE` writer - Alarm A interrupt enable
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBIE` reader - Alarm B interrupt enable
pub type ALRBIE_R = crate::BitReader;
///Field `ALRBIE` writer - Alarm B interrupt enable
pub type ALRBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTIE` reader - Wakeup timer interrupt enable
pub type WUTIE_R = crate::BitReader;
///Field `WUTIE` writer - Wakeup timer interrupt enable
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSIE` reader - Time-stamp interrupt enable
pub type TSIE_R = crate::BitReader;
///Field `TSIE` writer - Time-stamp interrupt enable
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD1H` reader - Add 1 hour (summer time change)
pub type ADD1H_R = crate::BitReader;
///Field `ADD1H` writer - Add 1 hour (summer time change)
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUB1H` reader - Subtract 1 hour (winter time change)
pub type SUB1H_R = crate::BitReader;
///Field `SUB1H` writer - Subtract 1 hour (winter time change)
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKP` reader - Backup
pub type BKP_R = crate::BitReader;
///Field `BKP` writer - Backup
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POL` reader - Output polarity
pub type POL_R = crate::BitReader;
///Field `POL` writer - Output polarity
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSEL` reader - Output selection
pub type OSEL_R = crate::FieldReader;
///Field `OSEL` writer - Output selection
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COE` reader - Calibration output enable
pub type COE_R = crate::BitReader;
///Field `COE` writer - Calibration output enable
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    pub fn wcksel(&self) -> WCKSEL_R {
        WCKSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Coarse digital calibration enable
    #[inline(always)]
    pub fn dce(&self) -> DCE_R {
        DCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Time stamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Add 1 hour (summer time change)
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Subtract 1 hour (winter time change)
    #[inline(always)]
    pub fn sub1h(&self) -> SUB1H_R {
        SUB1H_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("coe", &self.coe())
            .field("osel", &self.osel())
            .field("pol", &self.pol())
            .field("bkp", &self.bkp())
            .field("sub1h", &self.sub1h())
            .field("add1h", &self.add1h())
            .field("tsie", &self.tsie())
            .field("wutie", &self.wutie())
            .field("alrbie", &self.alrbie())
            .field("alraie", &self.alraie())
            .field("tse", &self.tse())
            .field("wute", &self.wute())
            .field("alrbe", &self.alrbe())
            .field("alrae", &self.alrae())
            .field("dce", &self.dce())
            .field("fmt", &self.fmt())
            .field("refckon", &self.refckon())
            .field("tsedge", &self.tsedge())
            .field("wcksel", &self.wcksel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    pub fn wcksel(&mut self) -> WCKSEL_W<'_, CRrs> {
        WCKSEL_W::new(self, 0)
    }
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W<'_, CRrs> {
        TSEDGE_W::new(self, 3)
    }
    ///Bit 4 - Reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W<'_, CRrs> {
        REFCKON_W::new(self, 4)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W<'_, CRrs> {
        FMT_W::new(self, 6)
    }
    ///Bit 7 - Coarse digital calibration enable
    #[inline(always)]
    pub fn dce(&mut self) -> DCE_W<'_, CRrs> {
        DCE_W::new(self, 7)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W<'_, CRrs> {
        ALRAE_W::new(self, 8)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRBE_W<'_, CRrs> {
        ALRBE_W::new(self, 9)
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W<'_, CRrs> {
        WUTE_W::new(self, 10)
    }
    ///Bit 11 - Time stamp enable
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, CRrs> {
        TSE_W::new(self, 11)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W<'_, CRrs> {
        ALRAIE_W::new(self, 12)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRBIE_W<'_, CRrs> {
        ALRBIE_W::new(self, 13)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W<'_, CRrs> {
        WUTIE_W::new(self, 14)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<'_, CRrs> {
        TSIE_W::new(self, 15)
    }
    ///Bit 16 - Add 1 hour (summer time change)
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W<'_, CRrs> {
        ADD1H_W::new(self, 16)
    }
    ///Bit 17 - Subtract 1 hour (winter time change)
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W<'_, CRrs> {
        SUB1H_W::new(self, 17)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, CRrs> {
        BKP_W::new(self, 18)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<'_, CRrs> {
        POL_W::new(self, 20)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W<'_, CRrs> {
        OSEL_W::new(self, 21)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W<'_, CRrs> {
        COE_W::new(self, 23)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
