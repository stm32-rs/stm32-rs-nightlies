///Register `DBGSMPS` reader
pub type R = crate::R<DBGSMPSrs>;
///Register `DBGSMPS` writer
pub type W = crate::W<DBGSMPSrs>;
///Field `TESTDIG` reader - TESTDIG: SMPS TEST_DIG_3V3\[3:0\] SMPS control signal
pub type TESTDIG_R = crate::FieldReader;
///Field `TESTDIG` writer - TESTDIG: SMPS TEST_DIG_3V3\[3:0\] SMPS control signal
pub type TESTDIG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TESTKEL` reader - TESTKEL: SMPS TEST_KEL_3V3\[1:0\] SMPS control signal
pub type TESTKEL_R = crate::FieldReader;
///Field `TESTKEL` writer - TESTKEL: SMPS TEST_KEL_3V3\[1:0\] SMPS control signal
pub type TESTKEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HOT_STUP` reader - HOT_STUP_3V3 SMPS control signal
pub type HOT_STUP_R = crate::BitReader;
///Field `HOT_STUP` writer - HOT_STUP_3V3 SMPS control signal
pub type HOT_STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NO_STUP` reader - NO_STUP_3V3 SMPS control signal
pub type NO_STUP_R = crate::BitReader;
///Field `NO_STUP` writer - NO_STUP_3V3 SMPS control signal
pub type NO_STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TESTILIM` reader - TESTILIM: SMPS TEST_ILIM_3V3 SMPS control signal
pub type TESTILIM_R = crate::BitReader;
///Field `TESTILIM` writer - TESTILIM: SMPS TEST_ILIM_3V3 SMPS control signal
pub type TESTILIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTLRES_RAMP` reader - CTLRES_RAM_3V3 SMPS control signal
pub type CTLRES_RAMP_R = crate::BitReader;
///Field `CTLRES_RAMP` writer - CTLRES_RAM_3V3 SMPS control signal
pub type CTLRES_RAMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_BIG_MOS` reader - DIS_BIG_MOS_3V3 SMPS control signal
pub type DIS_BIG_MOS_R = crate::BitReader;
///Field `DIS_BIG_MOS` writer - DIS_BIG_MOS_3V3 SMPS control signal
pub type DIS_BIG_MOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEST_OL` reader - TEST_OL_3V3 SMPS control signal
pub type TEST_OL_R = crate::BitReader;
///Field `TEST_OL` writer - TEST_OL_3V3 SMPS control signal
pub type TEST_OL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_ILIM` reader - DIS_ILIM_3V3 SMPS control signal
pub type DIS_ILIM_R = crate::BitReader;
///Field `DIS_ILIM` writer - DIS_ILIM_3V3 SMPS control signal
pub type DIS_ILIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILIM_BOOST` reader - ILIM_BOOST_3V3 SMPS current limitation Boost - 0: Max current = 110mA (Default) - 1: Max current = 130mA
pub type ILIM_BOOST_R = crate::BitReader;
///Field `ILIM_BOOST` writer - ILIM_BOOST_3V3 SMPS current limitation Boost - 0: Max current = 110mA (Default) - 1: Max current = 130mA
pub type ILIM_BOOST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOF_CUR_SEL` reader - BOF_CUR_SEL Bypass On the Fly current limitation - 00 : 20mA - 01 : 40mA - 10 : 60mA (default) - 11 : no limit
pub type BOF_CUR_SEL_R = crate::FieldReader;
///Field `BOF_CUR_SEL` writer - BOF_CUR_SEL Bypass On the Fly current limitation - 00 : 20mA - 01 : 40mA - 10 : 60mA (default) - 11 : no limit
pub type BOF_CUR_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - TESTDIG: SMPS TEST_DIG_3V3\[3:0\] SMPS control signal
    #[inline(always)]
    pub fn testdig(&self) -> TESTDIG_R {
        TESTDIG_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - TESTKEL: SMPS TEST_KEL_3V3\[1:0\] SMPS control signal
    #[inline(always)]
    pub fn testkel(&self) -> TESTKEL_R {
        TESTKEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - HOT_STUP_3V3 SMPS control signal
    #[inline(always)]
    pub fn hot_stup(&self) -> HOT_STUP_R {
        HOT_STUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NO_STUP_3V3 SMPS control signal
    #[inline(always)]
    pub fn no_stup(&self) -> NO_STUP_R {
        NO_STUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TESTILIM: SMPS TEST_ILIM_3V3 SMPS control signal
    #[inline(always)]
    pub fn testilim(&self) -> TESTILIM_R {
        TESTILIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTLRES_RAM_3V3 SMPS control signal
    #[inline(always)]
    pub fn ctlres_ramp(&self) -> CTLRES_RAMP_R {
        CTLRES_RAMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DIS_BIG_MOS_3V3 SMPS control signal
    #[inline(always)]
    pub fn dis_big_mos(&self) -> DIS_BIG_MOS_R {
        DIS_BIG_MOS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TEST_OL_3V3 SMPS control signal
    #[inline(always)]
    pub fn test_ol(&self) -> TEST_OL_R {
        TEST_OL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DIS_ILIM_3V3 SMPS control signal
    #[inline(always)]
    pub fn dis_ilim(&self) -> DIS_ILIM_R {
        DIS_ILIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ILIM_BOOST_3V3 SMPS current limitation Boost - 0: Max current = 110mA (Default) - 1: Max current = 130mA
    #[inline(always)]
    pub fn ilim_boost(&self) -> ILIM_BOOST_R {
        ILIM_BOOST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - BOF_CUR_SEL Bypass On the Fly current limitation - 00 : 20mA - 01 : 40mA - 10 : 60mA (default) - 11 : no limit
    #[inline(always)]
    pub fn bof_cur_sel(&self) -> BOF_CUR_SEL_R {
        BOF_CUR_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGSMPS")
            .field("testdig", &self.testdig())
            .field("testkel", &self.testkel())
            .field("hot_stup", &self.hot_stup())
            .field("no_stup", &self.no_stup())
            .field("testilim", &self.testilim())
            .field("ctlres_ramp", &self.ctlres_ramp())
            .field("dis_big_mos", &self.dis_big_mos())
            .field("test_ol", &self.test_ol())
            .field("dis_ilim", &self.dis_ilim())
            .field("ilim_boost", &self.ilim_boost())
            .field("bof_cur_sel", &self.bof_cur_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - TESTDIG: SMPS TEST_DIG_3V3\[3:0\] SMPS control signal
    #[inline(always)]
    pub fn testdig(&mut self) -> TESTDIG_W<'_, DBGSMPSrs> {
        TESTDIG_W::new(self, 0)
    }
    ///Bits 4:5 - TESTKEL: SMPS TEST_KEL_3V3\[1:0\] SMPS control signal
    #[inline(always)]
    pub fn testkel(&mut self) -> TESTKEL_W<'_, DBGSMPSrs> {
        TESTKEL_W::new(self, 4)
    }
    ///Bit 6 - HOT_STUP_3V3 SMPS control signal
    #[inline(always)]
    pub fn hot_stup(&mut self) -> HOT_STUP_W<'_, DBGSMPSrs> {
        HOT_STUP_W::new(self, 6)
    }
    ///Bit 7 - NO_STUP_3V3 SMPS control signal
    #[inline(always)]
    pub fn no_stup(&mut self) -> NO_STUP_W<'_, DBGSMPSrs> {
        NO_STUP_W::new(self, 7)
    }
    ///Bit 8 - TESTILIM: SMPS TEST_ILIM_3V3 SMPS control signal
    #[inline(always)]
    pub fn testilim(&mut self) -> TESTILIM_W<'_, DBGSMPSrs> {
        TESTILIM_W::new(self, 8)
    }
    ///Bit 9 - CTLRES_RAM_3V3 SMPS control signal
    #[inline(always)]
    pub fn ctlres_ramp(&mut self) -> CTLRES_RAMP_W<'_, DBGSMPSrs> {
        CTLRES_RAMP_W::new(self, 9)
    }
    ///Bit 10 - DIS_BIG_MOS_3V3 SMPS control signal
    #[inline(always)]
    pub fn dis_big_mos(&mut self) -> DIS_BIG_MOS_W<'_, DBGSMPSrs> {
        DIS_BIG_MOS_W::new(self, 10)
    }
    ///Bit 11 - TEST_OL_3V3 SMPS control signal
    #[inline(always)]
    pub fn test_ol(&mut self) -> TEST_OL_W<'_, DBGSMPSrs> {
        TEST_OL_W::new(self, 11)
    }
    ///Bit 12 - DIS_ILIM_3V3 SMPS control signal
    #[inline(always)]
    pub fn dis_ilim(&mut self) -> DIS_ILIM_W<'_, DBGSMPSrs> {
        DIS_ILIM_W::new(self, 12)
    }
    ///Bit 13 - ILIM_BOOST_3V3 SMPS current limitation Boost - 0: Max current = 110mA (Default) - 1: Max current = 130mA
    #[inline(always)]
    pub fn ilim_boost(&mut self) -> ILIM_BOOST_W<'_, DBGSMPSrs> {
        ILIM_BOOST_W::new(self, 13)
    }
    ///Bits 14:15 - BOF_CUR_SEL Bypass On the Fly current limitation - 00 : 20mA - 01 : 40mA - 10 : 60mA (default) - 11 : no limit
    #[inline(always)]
    pub fn bof_cur_sel(&mut self) -> BOF_CUR_SEL_W<'_, DBGSMPSrs> {
        BOF_CUR_SEL_W::new(self, 14)
    }
}
/**DBGSMPS register

You can [`read`](crate::Reg::read) this register and get [`dbgsmps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsmps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:DBGSMPS)*/
pub struct DBGSMPSrs;
impl crate::RegisterSpec for DBGSMPSrs {
    type Ux = u32;
}
///`read()` method returns [`dbgsmps::R`](R) reader structure
impl crate::Readable for DBGSMPSrs {}
///`write(|w| ..)` method takes [`dbgsmps::W`](W) writer structure
impl crate::Writable for DBGSMPSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGSMPS to value 0x8000
impl crate::Resettable for DBGSMPSrs {
    const RESET_VALUE: u32 = 0x8000;
}
