///Register `DELAYRH` reader
pub type R = crate::R<DELAYRHrs>;
///Register `DELAYRH` writer
pub type W = crate::W<DELAYRHrs>;
///Field `DELAY8` reader - Port x I/O pin y delay setup
pub type DELAY8_R = crate::FieldReader;
///Field `DELAY8` writer - Port x I/O pin y delay setup
pub type DELAY8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY9` reader - Port x I/O pin y delay setup
pub type DELAY9_R = crate::FieldReader;
///Field `DELAY9` writer - Port x I/O pin y delay setup
pub type DELAY9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY10` reader - Port x I/O pin y delay setup
pub type DELAY10_R = crate::FieldReader;
///Field `DELAY10` writer - Port x I/O pin y delay setup
pub type DELAY10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY11` reader - Port x I/O pin y delay setup
pub type DELAY11_R = crate::FieldReader;
///Field `DELAY11` writer - Port x I/O pin y delay setup
pub type DELAY11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY12` reader - Port x I/O pin y delay setup
pub type DELAY12_R = crate::FieldReader;
///Field `DELAY12` writer - Port x I/O pin y delay setup
pub type DELAY12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY13` reader - Port x I/O pin y delay setup
pub type DELAY13_R = crate::FieldReader;
///Field `DELAY13` writer - Port x I/O pin y delay setup
pub type DELAY13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY14` reader - Port x I/O pin y delay setup
pub type DELAY14_R = crate::FieldReader;
///Field `DELAY14` writer - Port x I/O pin y delay setup
pub type DELAY14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DELAY15` reader - Port x I/O pin y delay setup
pub type DELAY15_R = crate::FieldReader;
///Field `DELAY15` writer - Port x I/O pin y delay setup
pub type DELAY15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay8(&self) -> DELAY8_R {
        DELAY8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay9(&self) -> DELAY9_R {
        DELAY9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay10(&self) -> DELAY10_R {
        DELAY10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay11(&self) -> DELAY11_R {
        DELAY11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay12(&self) -> DELAY12_R {
        DELAY12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay13(&self) -> DELAY13_R {
        DELAY13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay14(&self) -> DELAY14_R {
        DELAY14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay15(&self) -> DELAY15_R {
        DELAY15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DELAYRH")
            .field("delay8", &self.delay8())
            .field("delay9", &self.delay9())
            .field("delay10", &self.delay10())
            .field("delay11", &self.delay11())
            .field("delay12", &self.delay12())
            .field("delay13", &self.delay13())
            .field("delay14", &self.delay14())
            .field("delay15", &self.delay15())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay8(&mut self) -> DELAY8_W<DELAYRHrs> {
        DELAY8_W::new(self, 0)
    }
    ///Bits 4:7 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay9(&mut self) -> DELAY9_W<DELAYRHrs> {
        DELAY9_W::new(self, 4)
    }
    ///Bits 8:11 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay10(&mut self) -> DELAY10_W<DELAYRHrs> {
        DELAY10_W::new(self, 8)
    }
    ///Bits 12:15 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay11(&mut self) -> DELAY11_W<DELAYRHrs> {
        DELAY11_W::new(self, 12)
    }
    ///Bits 16:19 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay12(&mut self) -> DELAY12_W<DELAYRHrs> {
        DELAY12_W::new(self, 16)
    }
    ///Bits 20:23 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay13(&mut self) -> DELAY13_W<DELAYRHrs> {
        DELAY13_W::new(self, 20)
    }
    ///Bits 24:27 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay14(&mut self) -> DELAY14_W<DELAYRHrs> {
        DELAY14_W::new(self, 24)
    }
    ///Bits 28:31 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay15(&mut self) -> DELAY15_W<DELAYRHrs> {
        DELAY15_W::new(self, 28)
    }
}
/**GPIO port A delay high register

You can [`read`](crate::Reg::read) this register and get [`delayrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delayrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOA:DELAYRH)*/
pub struct DELAYRHrs;
impl crate::RegisterSpec for DELAYRHrs {
    type Ux = u32;
}
///`read()` method returns [`delayrh::R`](R) reader structure
impl crate::Readable for DELAYRHrs {}
///`write(|w| ..)` method takes [`delayrh::W`](W) writer structure
impl crate::Writable for DELAYRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DELAYRH to value 0
impl crate::Resettable for DELAYRHrs {}
