///Register `IOxCFG` reader
pub type R = crate::R<IOX_CFGrs>;
///Register `IOxCFG` writer
pub type W = crate::W<IOX_CFGrs>;
///Field `IOCFG0` reader - Drive configuration for PA8.
pub type IOCFG0_R = crate::FieldReader;
///Field `IOCFG0` writer - Drive configuration for PA8.
pub type IOCFG0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOCFG1` reader - Drive configuration for PA9.
pub type IOCFG1_R = crate::FieldReader;
///Field `IOCFG1` writer - Drive configuration for PA9.
pub type IOCFG1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOCFG2` reader - Drive configuration for PA10.
pub type IOCFG2_R = crate::FieldReader;
///Field `IOCFG2` writer - Drive configuration for PA10.
pub type IOCFG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOCFG3` reader - Drive configuration for PA11.
pub type IOCFG3_R = crate::FieldReader;
///Field `IOCFG3` writer - Drive configuration for PA11.
pub type IOCFG3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOCFG4` reader - Drive configuration for PA4.
pub type IOCFG4_R = crate::FieldReader;
///Field `IOCFG4` writer - Drive configuration for PA4.
pub type IOCFG4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOCFG5` reader - Drive configuration for PA5.
pub type IOCFG5_R = crate::FieldReader;
///Field `IOCFG5` writer - Drive configuration for PA5.
pub type IOCFG5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOCFG6` reader - Drive configuration for PA6.
pub type IOCFG6_R = crate::FieldReader;
///Field `IOCFG6` writer - Drive configuration for PA6.
pub type IOCFG6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOCFG7` reader - Drive configuration for PA7.
pub type IOCFG7_R = crate::FieldReader;
///Field `IOCFG7` writer - Drive configuration for PA7.
pub type IOCFG7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Drive configuration for PA8.
    #[inline(always)]
    pub fn iocfg0(&self) -> IOCFG0_R {
        IOCFG0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Drive configuration for PA9.
    #[inline(always)]
    pub fn iocfg1(&self) -> IOCFG1_R {
        IOCFG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Drive configuration for PA10.
    #[inline(always)]
    pub fn iocfg2(&self) -> IOCFG2_R {
        IOCFG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Drive configuration for PA11.
    #[inline(always)]
    pub fn iocfg3(&self) -> IOCFG3_R {
        IOCFG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Drive configuration for PA4.
    #[inline(always)]
    pub fn iocfg4(&self) -> IOCFG4_R {
        IOCFG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Drive configuration for PA5.
    #[inline(always)]
    pub fn iocfg5(&self) -> IOCFG5_R {
        IOCFG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Drive configuration for PA6.
    #[inline(always)]
    pub fn iocfg6(&self) -> IOCFG6_R {
        IOCFG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Drive configuration for PA7.
    #[inline(always)]
    pub fn iocfg7(&self) -> IOCFG7_R {
        IOCFG7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOxCFG")
            .field("iocfg0", &self.iocfg0())
            .field("iocfg1", &self.iocfg1())
            .field("iocfg2", &self.iocfg2())
            .field("iocfg3", &self.iocfg3())
            .field("iocfg4", &self.iocfg4())
            .field("iocfg5", &self.iocfg5())
            .field("iocfg6", &self.iocfg6())
            .field("iocfg7", &self.iocfg7())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Drive configuration for PA8.
    #[inline(always)]
    pub fn iocfg0(&mut self) -> IOCFG0_W<'_, IOX_CFGrs> {
        IOCFG0_W::new(self, 0)
    }
    ///Bits 2:3 - Drive configuration for PA9.
    #[inline(always)]
    pub fn iocfg1(&mut self) -> IOCFG1_W<'_, IOX_CFGrs> {
        IOCFG1_W::new(self, 2)
    }
    ///Bits 4:5 - Drive configuration for PA10.
    #[inline(always)]
    pub fn iocfg2(&mut self) -> IOCFG2_W<'_, IOX_CFGrs> {
        IOCFG2_W::new(self, 4)
    }
    ///Bits 6:7 - Drive configuration for PA11.
    #[inline(always)]
    pub fn iocfg3(&mut self) -> IOCFG3_W<'_, IOX_CFGrs> {
        IOCFG3_W::new(self, 6)
    }
    ///Bits 8:9 - Drive configuration for PA4.
    #[inline(always)]
    pub fn iocfg4(&mut self) -> IOCFG4_W<'_, IOX_CFGrs> {
        IOCFG4_W::new(self, 8)
    }
    ///Bits 10:11 - Drive configuration for PA5.
    #[inline(always)]
    pub fn iocfg5(&mut self) -> IOCFG5_W<'_, IOX_CFGrs> {
        IOCFG5_W::new(self, 10)
    }
    ///Bits 12:13 - Drive configuration for PA6.
    #[inline(always)]
    pub fn iocfg6(&mut self) -> IOCFG6_W<'_, IOX_CFGrs> {
        IOCFG6_W::new(self, 12)
    }
    ///Bits 14:15 - Drive configuration for PA7.
    #[inline(always)]
    pub fn iocfg7(&mut self) -> IOCFG7_W<'_, IOX_CFGrs> {
        IOCFG7_W::new(self, 14)
    }
}
/**IOxCFG register

You can [`read`](crate::Reg::read) this register and get [`iox_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iox_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PWRC:IOxCFG)*/
pub struct IOX_CFGrs;
impl crate::RegisterSpec for IOX_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`iox_cfg::R`](R) reader structure
impl crate::Readable for IOX_CFGrs {}
///`write(|w| ..)` method takes [`iox_cfg::W`](W) writer structure
impl crate::Writable for IOX_CFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOxCFG to value 0
impl crate::Resettable for IOX_CFGrs {}
