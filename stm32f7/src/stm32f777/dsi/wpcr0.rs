///Register `WPCR0` reader
pub type R = crate::R<WPCR0rs>;
///Register `WPCR0` writer
pub type W = crate::W<WPCR0rs>;
///Field `UIX4` reader - Unit Interval multiplied by 4
pub type UIX4_R = crate::FieldReader;
///Field `UIX4` writer - Unit Interval multiplied by 4
pub type UIX4_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SWCL` reader - Swap Clock Lane pins
pub type SWCL_R = crate::BitReader;
///Field `SWCL` writer - Swap Clock Lane pins
pub type SWCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWDL0` reader - Swap Data Lane 0 pins
pub type SWDL0_R = crate::BitReader;
///Field `SWDL0` writer - Swap Data Lane 0 pins
pub type SWDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWDL1` reader - Swap Data Lane 1 pins
pub type SWDL1_R = crate::BitReader;
///Field `SWDL1` writer - Swap Data Lane 1 pins
pub type SWDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSICL` reader - Invert Hight-Speed data signal on Clock Lane
pub type HSICL_R = crate::BitReader;
///Field `HSICL` writer - Invert Hight-Speed data signal on Clock Lane
pub type HSICL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIDL0` reader - Invert the Hight-Speed data signal on Data Lane 0
pub type HSIDL0_R = crate::BitReader;
///Field `HSIDL0` writer - Invert the Hight-Speed data signal on Data Lane 0
pub type HSIDL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIDL1` reader - Invert the High-Speed data signal on Data Lane 1
pub type HSIDL1_R = crate::BitReader;
///Field `HSIDL1` writer - Invert the High-Speed data signal on Data Lane 1
pub type HSIDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTXSMCL` reader - Force in TX Stop Mode the Clock Lane
pub type FTXSMCL_R = crate::BitReader;
///Field `FTXSMCL` writer - Force in TX Stop Mode the Clock Lane
pub type FTXSMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTXSMDL` reader - Force in TX Stop Mode the Data Lanes
pub type FTXSMDL_R = crate::BitReader;
///Field `FTXSMDL` writer - Force in TX Stop Mode the Data Lanes
pub type FTXSMDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDOFFDL` reader - Contention Detection OFF on Data Lanes
pub type CDOFFDL_R = crate::BitReader;
///Field `CDOFFDL` writer - Contention Detection OFF on Data Lanes
pub type CDOFFDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDDL` reader - Turn Disable Data Lanes
pub type TDDL_R = crate::BitReader;
///Field `TDDL` writer - Turn Disable Data Lanes
pub type TDDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDEN` reader - Pull-Down Enable
pub type PDEN_R = crate::BitReader;
///Field `PDEN` writer - Pull-Down Enable
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLKPREPEN` reader - custom time for tCLK-PREPARE Enable
pub type TCLKPREPEN_R = crate::BitReader;
///Field `TCLKPREPEN` writer - custom time for tCLK-PREPARE Enable
pub type TCLKPREPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLKZEROEN` reader - custom time for tCLK-ZERO Enable
pub type TCLKZEROEN_R = crate::BitReader;
///Field `TCLKZEROEN` writer - custom time for tCLK-ZERO Enable
pub type TCLKZEROEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THSPREPEN` reader - custom time for tHS-PREPARE Enable
pub type THSPREPEN_R = crate::BitReader;
///Field `THSPREPEN` writer - custom time for tHS-PREPARE Enable
pub type THSPREPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THSTRAILEN` reader - custom time for tHS-TRAIL Enable
pub type THSTRAILEN_R = crate::BitReader;
///Field `THSTRAILEN` writer - custom time for tHS-TRAIL Enable
pub type THSTRAILEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THSZEROEN` reader - custom time for tHS-ZERO Enable
pub type THSZEROEN_R = crate::BitReader;
///Field `THSZEROEN` writer - custom time for tHS-ZERO Enable
pub type THSZEROEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TLPXDEN` reader - custom time for tLPX for Data lanes Enable
pub type TLPXDEN_R = crate::BitReader;
///Field `TLPXDEN` writer - custom time for tLPX for Data lanes Enable
pub type TLPXDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THSEXITEN` reader - custom time for tHS-EXIT Enable
pub type THSEXITEN_R = crate::BitReader;
///Field `THSEXITEN` writer - custom time for tHS-EXIT Enable
pub type THSEXITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TLPXCEN` reader - custom time for tLPX for Clock lane Enable
pub type TLPXCEN_R = crate::BitReader;
///Field `TLPXCEN` writer - custom time for tLPX for Clock lane Enable
pub type TLPXCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLKPOSTEN` reader - custom time for tCLK-POST Enable
pub type TCLKPOSTEN_R = crate::BitReader;
///Field `TCLKPOSTEN` writer - custom time for tCLK-POST Enable
pub type TCLKPOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Unit Interval multiplied by 4
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - Swap Clock Lane pins
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Swap Data Lane 0 pins
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Swap Data Lane 1 pins
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Invert Hight-Speed data signal on Clock Lane
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Invert the Hight-Speed data signal on Data Lane 0
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Invert the High-Speed data signal on Data Lane 1
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Force in TX Stop Mode the Clock Lane
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Force in TX Stop Mode the Data Lanes
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Contention Detection OFF on Data Lanes
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Turn Disable Data Lanes
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Pull-Down Enable
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - custom time for tCLK-PREPARE Enable
    #[inline(always)]
    pub fn tclkprepen(&self) -> TCLKPREPEN_R {
        TCLKPREPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - custom time for tCLK-ZERO Enable
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TCLKZEROEN_R {
        TCLKZEROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - custom time for tHS-PREPARE Enable
    #[inline(always)]
    pub fn thsprepen(&self) -> THSPREPEN_R {
        THSPREPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - custom time for tHS-TRAIL Enable
    #[inline(always)]
    pub fn thstrailen(&self) -> THSTRAILEN_R {
        THSTRAILEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - custom time for tHS-ZERO Enable
    #[inline(always)]
    pub fn thszeroen(&self) -> THSZEROEN_R {
        THSZEROEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - custom time for tLPX for Data lanes Enable
    #[inline(always)]
    pub fn tlpxden(&self) -> TLPXDEN_R {
        TLPXDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - custom time for tHS-EXIT Enable
    #[inline(always)]
    pub fn thsexiten(&self) -> THSEXITEN_R {
        THSEXITEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - custom time for tLPX for Clock lane Enable
    #[inline(always)]
    pub fn tlpxcen(&self) -> TLPXCEN_R {
        TLPXCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - custom time for tCLK-POST Enable
    #[inline(always)]
    pub fn tclkposten(&self) -> TCLKPOSTEN_R {
        TCLKPOSTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR0")
            .field("tclkposten", &self.tclkposten())
            .field("tlpxcen", &self.tlpxcen())
            .field("thsexiten", &self.thsexiten())
            .field("tlpxden", &self.tlpxden())
            .field("thszeroen", &self.thszeroen())
            .field("thstrailen", &self.thstrailen())
            .field("thsprepen", &self.thsprepen())
            .field("tclkzeroen", &self.tclkzeroen())
            .field("tclkprepen", &self.tclkprepen())
            .field("pden", &self.pden())
            .field("tddl", &self.tddl())
            .field("cdoffdl", &self.cdoffdl())
            .field("ftxsmdl", &self.ftxsmdl())
            .field("ftxsmcl", &self.ftxsmcl())
            .field("hsidl1", &self.hsidl1())
            .field("hsidl0", &self.hsidl0())
            .field("hsicl", &self.hsicl())
            .field("swdl1", &self.swdl1())
            .field("swdl0", &self.swdl0())
            .field("swcl", &self.swcl())
            .field("uix4", &self.uix4())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Unit Interval multiplied by 4
    #[inline(always)]
    pub fn uix4(&mut self) -> UIX4_W<'_, WPCR0rs> {
        UIX4_W::new(self, 0)
    }
    ///Bit 6 - Swap Clock Lane pins
    #[inline(always)]
    pub fn swcl(&mut self) -> SWCL_W<'_, WPCR0rs> {
        SWCL_W::new(self, 6)
    }
    ///Bit 7 - Swap Data Lane 0 pins
    #[inline(always)]
    pub fn swdl0(&mut self) -> SWDL0_W<'_, WPCR0rs> {
        SWDL0_W::new(self, 7)
    }
    ///Bit 8 - Swap Data Lane 1 pins
    #[inline(always)]
    pub fn swdl1(&mut self) -> SWDL1_W<'_, WPCR0rs> {
        SWDL1_W::new(self, 8)
    }
    ///Bit 9 - Invert Hight-Speed data signal on Clock Lane
    #[inline(always)]
    pub fn hsicl(&mut self) -> HSICL_W<'_, WPCR0rs> {
        HSICL_W::new(self, 9)
    }
    ///Bit 10 - Invert the Hight-Speed data signal on Data Lane 0
    #[inline(always)]
    pub fn hsidl0(&mut self) -> HSIDL0_W<'_, WPCR0rs> {
        HSIDL0_W::new(self, 10)
    }
    ///Bit 11 - Invert the High-Speed data signal on Data Lane 1
    #[inline(always)]
    pub fn hsidl1(&mut self) -> HSIDL1_W<'_, WPCR0rs> {
        HSIDL1_W::new(self, 11)
    }
    ///Bit 12 - Force in TX Stop Mode the Clock Lane
    #[inline(always)]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<'_, WPCR0rs> {
        FTXSMCL_W::new(self, 12)
    }
    ///Bit 13 - Force in TX Stop Mode the Data Lanes
    #[inline(always)]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<'_, WPCR0rs> {
        FTXSMDL_W::new(self, 13)
    }
    ///Bit 14 - Contention Detection OFF on Data Lanes
    #[inline(always)]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W<'_, WPCR0rs> {
        CDOFFDL_W::new(self, 14)
    }
    ///Bit 16 - Turn Disable Data Lanes
    #[inline(always)]
    pub fn tddl(&mut self) -> TDDL_W<'_, WPCR0rs> {
        TDDL_W::new(self, 16)
    }
    ///Bit 18 - Pull-Down Enable
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W<'_, WPCR0rs> {
        PDEN_W::new(self, 18)
    }
    ///Bit 19 - custom time for tCLK-PREPARE Enable
    #[inline(always)]
    pub fn tclkprepen(&mut self) -> TCLKPREPEN_W<'_, WPCR0rs> {
        TCLKPREPEN_W::new(self, 19)
    }
    ///Bit 20 - custom time for tCLK-ZERO Enable
    #[inline(always)]
    pub fn tclkzeroen(&mut self) -> TCLKZEROEN_W<'_, WPCR0rs> {
        TCLKZEROEN_W::new(self, 20)
    }
    ///Bit 21 - custom time for tHS-PREPARE Enable
    #[inline(always)]
    pub fn thsprepen(&mut self) -> THSPREPEN_W<'_, WPCR0rs> {
        THSPREPEN_W::new(self, 21)
    }
    ///Bit 22 - custom time for tHS-TRAIL Enable
    #[inline(always)]
    pub fn thstrailen(&mut self) -> THSTRAILEN_W<'_, WPCR0rs> {
        THSTRAILEN_W::new(self, 22)
    }
    ///Bit 23 - custom time for tHS-ZERO Enable
    #[inline(always)]
    pub fn thszeroen(&mut self) -> THSZEROEN_W<'_, WPCR0rs> {
        THSZEROEN_W::new(self, 23)
    }
    ///Bit 24 - custom time for tLPX for Data lanes Enable
    #[inline(always)]
    pub fn tlpxden(&mut self) -> TLPXDEN_W<'_, WPCR0rs> {
        TLPXDEN_W::new(self, 24)
    }
    ///Bit 25 - custom time for tHS-EXIT Enable
    #[inline(always)]
    pub fn thsexiten(&mut self) -> THSEXITEN_W<'_, WPCR0rs> {
        THSEXITEN_W::new(self, 25)
    }
    ///Bit 26 - custom time for tLPX for Clock lane Enable
    #[inline(always)]
    pub fn tlpxcen(&mut self) -> TLPXCEN_W<'_, WPCR0rs> {
        TLPXCEN_W::new(self, 26)
    }
    ///Bit 27 - custom time for tCLK-POST Enable
    #[inline(always)]
    pub fn tclkposten(&mut self) -> TCLKPOSTEN_W<'_, WPCR0rs> {
        TCLKPOSTEN_W::new(self, 27)
    }
}
/**DSI Wrapper PHY Configuration Register 1

You can [`read`](crate::Reg::read) this register and get [`wpcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#DSI:WPCR0)*/
pub struct WPCR0rs;
impl crate::RegisterSpec for WPCR0rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr0::R`](R) reader structure
impl crate::Readable for WPCR0rs {}
///`write(|w| ..)` method takes [`wpcr0::W`](W) writer structure
impl crate::Writable for WPCR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR0 to value 0
impl crate::Resettable for WPCR0rs {}
