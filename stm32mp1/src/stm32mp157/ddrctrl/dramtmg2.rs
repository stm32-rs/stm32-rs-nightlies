///Register `DRAMTMG2` reader
pub type R = crate::R<DRAMTMG2rs>;
///Register `DRAMTMG2` writer
pub type W = crate::W<DRAMTMG2rs>;
///Field `WR2RD` reader - WR2RD
pub type WR2RD_R = crate::FieldReader;
///Field `WR2RD` writer - WR2RD
pub type WR2RD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RD2WR` reader - RD2WR
pub type RD2WR_R = crate::FieldReader;
///Field `RD2WR` writer - RD2WR
pub type RD2WR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `READ_LATENCY` reader - READ_LATENCY
pub type READ_LATENCY_R = crate::FieldReader;
///Field `READ_LATENCY` writer - READ_LATENCY
pub type READ_LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `WRITE_LATENCY` reader - WRITE_LATENCY
pub type WRITE_LATENCY_R = crate::FieldReader;
///Field `WRITE_LATENCY` writer - WRITE_LATENCY
pub type WRITE_LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - WR2RD
    #[inline(always)]
    pub fn wr2rd(&self) -> WR2RD_R {
        WR2RD_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - RD2WR
    #[inline(always)]
    pub fn rd2wr(&self) -> RD2WR_R {
        RD2WR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - READ_LATENCY
    #[inline(always)]
    pub fn read_latency(&self) -> READ_LATENCY_R {
        READ_LATENCY_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - WRITE_LATENCY
    #[inline(always)]
    pub fn write_latency(&self) -> WRITE_LATENCY_R {
        WRITE_LATENCY_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAMTMG2")
            .field("wr2rd", &self.wr2rd())
            .field("rd2wr", &self.rd2wr())
            .field("read_latency", &self.read_latency())
            .field("write_latency", &self.write_latency())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - WR2RD
    #[inline(always)]
    pub fn wr2rd(&mut self) -> WR2RD_W<'_, DRAMTMG2rs> {
        WR2RD_W::new(self, 0)
    }
    ///Bits 8:13 - RD2WR
    #[inline(always)]
    pub fn rd2wr(&mut self) -> RD2WR_W<'_, DRAMTMG2rs> {
        RD2WR_W::new(self, 8)
    }
    ///Bits 16:21 - READ_LATENCY
    #[inline(always)]
    pub fn read_latency(&mut self) -> READ_LATENCY_W<'_, DRAMTMG2rs> {
        READ_LATENCY_W::new(self, 16)
    }
    ///Bits 24:29 - WRITE_LATENCY
    #[inline(always)]
    pub fn write_latency(&mut self) -> WRITE_LATENCY_W<'_, DRAMTMG2rs> {
        WRITE_LATENCY_W::new(self, 24)
    }
}
/**DDRCTRL SDRAM timing register 2

You can [`read`](crate::Reg::read) this register and get [`dramtmg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG2)*/
pub struct DRAMTMG2rs;
impl crate::RegisterSpec for DRAMTMG2rs {
    type Ux = u32;
}
///`read()` method returns [`dramtmg2::R`](R) reader structure
impl crate::Readable for DRAMTMG2rs {}
///`write(|w| ..)` method takes [`dramtmg2::W`](W) writer structure
impl crate::Writable for DRAMTMG2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DRAMTMG2 to value 0x0305_060d
impl crate::Resettable for DRAMTMG2rs {
    const RESET_VALUE: u32 = 0x0305_060d;
}
