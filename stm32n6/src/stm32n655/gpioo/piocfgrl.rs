///Register `PIOCFGRL` reader
pub type R = crate::R<PIOCFGRLrs>;
///Register `PIOCFGRL` writer
pub type W = crate::W<PIOCFGRLrs>;
///Field `PIOCFG0` reader - Port x I/O pin y configuration
pub type PIOCFG0_R = crate::FieldReader;
///Field `PIOCFG0` writer - Port x I/O pin y configuration
pub type PIOCFG0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG1` reader - Port x I/O pin y configuration
pub type PIOCFG1_R = crate::FieldReader;
///Field `PIOCFG1` writer - Port x I/O pin y configuration
pub type PIOCFG1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG2` reader - Port x I/O pin y configuration
pub type PIOCFG2_R = crate::FieldReader;
///Field `PIOCFG2` writer - Port x I/O pin y configuration
pub type PIOCFG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG3` reader - Port x I/O pin y configuration
pub type PIOCFG3_R = crate::FieldReader;
///Field `PIOCFG3` writer - Port x I/O pin y configuration
pub type PIOCFG3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG4` reader - Port x I/O pin y configuration
pub type PIOCFG4_R = crate::FieldReader;
///Field `PIOCFG4` writer - Port x I/O pin y configuration
pub type PIOCFG4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG5` reader - Port x I/O pin y configuration
pub type PIOCFG5_R = crate::FieldReader;
///Field `PIOCFG5` writer - Port x I/O pin y configuration
pub type PIOCFG5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG6` reader - Port x I/O pin y configuration
pub type PIOCFG6_R = crate::FieldReader;
///Field `PIOCFG6` writer - Port x I/O pin y configuration
pub type PIOCFG6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG7` reader - Port x I/O pin y configuration
pub type PIOCFG7_R = crate::FieldReader;
///Field `PIOCFG7` writer - Port x I/O pin y configuration
pub type PIOCFG7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg0(&self) -> PIOCFG0_R {
        PIOCFG0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg1(&self) -> PIOCFG1_R {
        PIOCFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg2(&self) -> PIOCFG2_R {
        PIOCFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg3(&self) -> PIOCFG3_R {
        PIOCFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg4(&self) -> PIOCFG4_R {
        PIOCFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg5(&self) -> PIOCFG5_R {
        PIOCFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg6(&self) -> PIOCFG6_R {
        PIOCFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg7(&self) -> PIOCFG7_R {
        PIOCFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIOCFGRL")
            .field("piocfg0", &self.piocfg0())
            .field("piocfg1", &self.piocfg1())
            .field("piocfg2", &self.piocfg2())
            .field("piocfg3", &self.piocfg3())
            .field("piocfg4", &self.piocfg4())
            .field("piocfg5", &self.piocfg5())
            .field("piocfg6", &self.piocfg6())
            .field("piocfg7", &self.piocfg7())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg0(&mut self) -> PIOCFG0_W<PIOCFGRLrs> {
        PIOCFG0_W::new(self, 0)
    }
    ///Bits 4:7 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg1(&mut self) -> PIOCFG1_W<PIOCFGRLrs> {
        PIOCFG1_W::new(self, 4)
    }
    ///Bits 8:11 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg2(&mut self) -> PIOCFG2_W<PIOCFGRLrs> {
        PIOCFG2_W::new(self, 8)
    }
    ///Bits 12:15 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg3(&mut self) -> PIOCFG3_W<PIOCFGRLrs> {
        PIOCFG3_W::new(self, 12)
    }
    ///Bits 16:19 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg4(&mut self) -> PIOCFG4_W<PIOCFGRLrs> {
        PIOCFG4_W::new(self, 16)
    }
    ///Bits 20:23 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg5(&mut self) -> PIOCFG5_W<PIOCFGRLrs> {
        PIOCFG5_W::new(self, 20)
    }
    ///Bits 24:27 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg6(&mut self) -> PIOCFG6_W<PIOCFGRLrs> {
        PIOCFG6_W::new(self, 24)
    }
    ///Bits 28:31 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg7(&mut self) -> PIOCFG7_W<PIOCFGRLrs> {
        PIOCFG7_W::new(self, 28)
    }
}
/**GPIO port O PIO control low register

You can [`read`](crate::Reg::read) this register and get [`piocfgrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`piocfgrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GPIOO:PIOCFGRL)*/
pub struct PIOCFGRLrs;
impl crate::RegisterSpec for PIOCFGRLrs {
    type Ux = u32;
}
///`read()` method returns [`piocfgrl::R`](R) reader structure
impl crate::Readable for PIOCFGRLrs {}
///`write(|w| ..)` method takes [`piocfgrl::W`](W) writer structure
impl crate::Writable for PIOCFGRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIOCFGRL to value 0
impl crate::Resettable for PIOCFGRLrs {}
