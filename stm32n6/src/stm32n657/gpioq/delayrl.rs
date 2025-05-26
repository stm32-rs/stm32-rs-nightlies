///Register `DELAYRL` reader
pub type R = crate::R<DELAYRLrs>;
///Register `DELAYRL` writer
pub type W = crate::W<DELAYRLrs>;
///Field `DELAY0` reader - Port x IO pin y delay setup
pub type DELAY0_R = crate::FieldReader;
///Field `DELAY0` writer - Port x IO pin y delay setup
pub type DELAY0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY1` reader - Port x IO pin y delay setup
pub type DELAY1_R = crate::FieldReader;
///Field `DELAY1` writer - Port x IO pin y delay setup
pub type DELAY1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY2` reader - Port x IO pin y delay setup
pub type DELAY2_R = crate::FieldReader;
///Field `DELAY2` writer - Port x IO pin y delay setup
pub type DELAY2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY3` reader - Port x IO pin y delay setup
pub type DELAY3_R = crate::FieldReader;
///Field `DELAY3` writer - Port x IO pin y delay setup
pub type DELAY3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY4` reader - Port x IO pin y delay setup
pub type DELAY4_R = crate::FieldReader;
///Field `DELAY4` writer - Port x IO pin y delay setup
pub type DELAY4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY5` reader - Port x IO pin y delay setup
pub type DELAY5_R = crate::FieldReader;
///Field `DELAY5` writer - Port x IO pin y delay setup
pub type DELAY5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY6` reader - Port x IO pin y delay setup
pub type DELAY6_R = crate::FieldReader;
///Field `DELAY6` writer - Port x IO pin y delay setup
pub type DELAY6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY7` reader - Port x IO pin y delay setup
pub type DELAY7_R = crate::FieldReader;
///Field `DELAY7` writer - Port x IO pin y delay setup
pub type DELAY7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay0(&self) -> DELAY0_R {
        DELAY0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay1(&self) -> DELAY1_R {
        DELAY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay2(&self) -> DELAY2_R {
        DELAY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay3(&self) -> DELAY3_R {
        DELAY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay4(&self) -> DELAY4_R {
        DELAY4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay5(&self) -> DELAY5_R {
        DELAY5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay6(&self) -> DELAY6_R {
        DELAY6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay7(&self) -> DELAY7_R {
        DELAY7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DELAYRL")
            .field("delay0", &self.delay0())
            .field("delay1", &self.delay1())
            .field("delay2", &self.delay2())
            .field("delay3", &self.delay3())
            .field("delay4", &self.delay4())
            .field("delay5", &self.delay5())
            .field("delay6", &self.delay6())
            .field("delay7", &self.delay7())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay0(&mut self) -> DELAY0_W<DELAYRLrs> {
        DELAY0_W::new(self, 0)
    }
    ///Bits 4:7 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay1(&mut self) -> DELAY1_W<DELAYRLrs> {
        DELAY1_W::new(self, 4)
    }
    ///Bits 8:11 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay2(&mut self) -> DELAY2_W<DELAYRLrs> {
        DELAY2_W::new(self, 8)
    }
    ///Bits 12:15 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay3(&mut self) -> DELAY3_W<DELAYRLrs> {
        DELAY3_W::new(self, 12)
    }
    ///Bits 16:19 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay4(&mut self) -> DELAY4_W<DELAYRLrs> {
        DELAY4_W::new(self, 16)
    }
    ///Bits 20:23 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay5(&mut self) -> DELAY5_W<DELAYRLrs> {
        DELAY5_W::new(self, 20)
    }
    ///Bits 24:27 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay6(&mut self) -> DELAY6_W<DELAYRLrs> {
        DELAY6_W::new(self, 24)
    }
    ///Bits 28:31 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay7(&mut self) -> DELAY7_W<DELAYRLrs> {
        DELAY7_W::new(self, 28)
    }
}
/**GPIO port Q delay low register

You can [`read`](crate::Reg::read) this register and get [`delayrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delayrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOQ:DELAYRL)*/
pub struct DELAYRLrs;
impl crate::RegisterSpec for DELAYRLrs {
    type Ux = u32;
}
///`read()` method returns [`delayrl::R`](R) reader structure
impl crate::Readable for DELAYRLrs {}
///`write(|w| ..)` method takes [`delayrl::W`](W) writer structure
impl crate::Writable for DELAYRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DELAYRL to value 0
impl crate::Resettable for DELAYRLrs {}
