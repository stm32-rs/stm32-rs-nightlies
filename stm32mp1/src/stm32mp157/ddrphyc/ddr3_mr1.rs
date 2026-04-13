///Register `DDR3_MR1` reader
pub type R = crate::R<DDR3_MR1rs>;
///Register `DDR3_MR1` writer
pub type W = crate::W<DDR3_MR1rs>;
///Field `DE` reader - DE
pub type DE_R = crate::BitReader;
///Field `DE` writer - DE
pub type DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIC0` reader - DIC0
pub type DIC0_R = crate::BitReader;
///Field `DIC0` writer - DIC0
pub type DIC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTT0` reader - RTT0
pub type RTT0_R = crate::BitReader;
///Field `RTT0` writer - RTT0
pub type RTT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AL` reader - AL
pub type AL_R = crate::FieldReader;
///Field `AL` writer - AL
pub type AL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIC1` reader - DIC1
pub type DIC1_R = crate::BitReader;
///Field `DIC1` writer - DIC1
pub type DIC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTT1` reader - RTT1
pub type RTT1_R = crate::BitReader;
///Field `RTT1` writer - RTT1
pub type RTT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LEVEL` reader - LEVEL
pub type LEVEL_R = crate::BitReader;
///Field `LEVEL` writer - LEVEL
pub type LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTT2` reader - RTT2
pub type RTT2_R = crate::BitReader;
///Field `RTT2` writer - RTT2
pub type RTT2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDQS` reader - TDQS
pub type TDQS_R = crate::BitReader;
///Field `TDQS` writer - TDQS
pub type TDQS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QOFF` reader - QOFF
pub type QOFF_R = crate::BitReader;
///Field `QOFF` writer - QOFF
pub type QOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DE
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DIC0
    #[inline(always)]
    pub fn dic0(&self) -> DIC0_R {
        DIC0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RTT0
    #[inline(always)]
    pub fn rtt0(&self) -> RTT0_R {
        RTT0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - AL
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - DIC1
    #[inline(always)]
    pub fn dic1(&self) -> DIC1_R {
        DIC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RTT1
    #[inline(always)]
    pub fn rtt1(&self) -> RTT1_R {
        RTT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LEVEL
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - RTT2
    #[inline(always)]
    pub fn rtt2(&self) -> RTT2_R {
        RTT2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - TDQS
    #[inline(always)]
    pub fn tdqs(&self) -> TDQS_R {
        TDQS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - QOFF
    #[inline(always)]
    pub fn qoff(&self) -> QOFF_R {
        QOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR3_MR1")
            .field("de", &self.de())
            .field("dic0", &self.dic0())
            .field("rtt0", &self.rtt0())
            .field("al", &self.al())
            .field("dic1", &self.dic1())
            .field("rtt1", &self.rtt1())
            .field("level", &self.level())
            .field("rtt2", &self.rtt2())
            .field("tdqs", &self.tdqs())
            .field("qoff", &self.qoff())
            .finish()
    }
}
impl W {
    ///Bit 0 - DE
    #[inline(always)]
    pub fn de(&mut self) -> DE_W<'_, DDR3_MR1rs> {
        DE_W::new(self, 0)
    }
    ///Bit 1 - DIC0
    #[inline(always)]
    pub fn dic0(&mut self) -> DIC0_W<'_, DDR3_MR1rs> {
        DIC0_W::new(self, 1)
    }
    ///Bit 2 - RTT0
    #[inline(always)]
    pub fn rtt0(&mut self) -> RTT0_W<'_, DDR3_MR1rs> {
        RTT0_W::new(self, 2)
    }
    ///Bits 3:4 - AL
    #[inline(always)]
    pub fn al(&mut self) -> AL_W<'_, DDR3_MR1rs> {
        AL_W::new(self, 3)
    }
    ///Bit 5 - DIC1
    #[inline(always)]
    pub fn dic1(&mut self) -> DIC1_W<'_, DDR3_MR1rs> {
        DIC1_W::new(self, 5)
    }
    ///Bit 6 - RTT1
    #[inline(always)]
    pub fn rtt1(&mut self) -> RTT1_W<'_, DDR3_MR1rs> {
        RTT1_W::new(self, 6)
    }
    ///Bit 7 - LEVEL
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W<'_, DDR3_MR1rs> {
        LEVEL_W::new(self, 7)
    }
    ///Bit 9 - RTT2
    #[inline(always)]
    pub fn rtt2(&mut self) -> RTT2_W<'_, DDR3_MR1rs> {
        RTT2_W::new(self, 9)
    }
    ///Bit 11 - TDQS
    #[inline(always)]
    pub fn tdqs(&mut self) -> TDQS_W<'_, DDR3_MR1rs> {
        TDQS_W::new(self, 11)
    }
    ///Bit 12 - QOFF
    #[inline(always)]
    pub fn qoff(&mut self) -> QOFF_W<'_, DDR3_MR1rs> {
        QOFF_W::new(self, 12)
    }
}
/**DDRPHYC MR1 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddr3_mr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr3_mr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDR3_MR1)*/
pub struct DDR3_MR1rs;
impl crate::RegisterSpec for DDR3_MR1rs {
    type Ux = u16;
}
///`read()` method returns [`ddr3_mr1::R`](R) reader structure
impl crate::Readable for DDR3_MR1rs {}
///`write(|w| ..)` method takes [`ddr3_mr1::W`](W) writer structure
impl crate::Writable for DDR3_MR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DDR3_MR1 to value 0
impl crate::Resettable for DDR3_MR1rs {}
