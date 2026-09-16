///Register `DELAYRH` reader
pub type R = crate::R<DELAYRHrs>;
///Register `DELAYRH` writer
pub type W = crate::W<DELAYRHrs>;
///Field `DELAY(8-15)` reader - Port x I/O pin y delay setup
pub type DELAY_R = crate::FieldReader;
///Field `DELAY(8-15)` writer - Port x I/O pin y delay setup
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Port x I/O pin y delay setup
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DELAY8` field.</div>
    #[inline(always)]
    pub fn delay(&self, n: u8) -> DELAY_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DELAY_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay_iter(&self) -> impl Iterator<Item = DELAY_R> + '_ {
        (0..8).map(move |n| DELAY_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    ///Bits 0:3 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay8(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay9(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay10(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay11(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay12(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay13(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay14(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay15(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 28) & 0x0f) as u8)
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
    ///Port x I/O pin y delay setup
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DELAY8` field.</div>
    #[inline(always)]
    pub fn delay(&mut self, n: u8) -> DELAY_W<'_, DELAYRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DELAY_W::new(self, n * 4)
    }
    ///Bits 0:3 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay8(&mut self) -> DELAY_W<'_, DELAYRHrs> {
        DELAY_W::new(self, 0)
    }
    ///Bits 4:7 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay9(&mut self) -> DELAY_W<'_, DELAYRHrs> {
        DELAY_W::new(self, 4)
    }
    ///Bits 8:11 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay10(&mut self) -> DELAY_W<'_, DELAYRHrs> {
        DELAY_W::new(self, 8)
    }
    ///Bits 12:15 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay11(&mut self) -> DELAY_W<'_, DELAYRHrs> {
        DELAY_W::new(self, 12)
    }
    ///Bits 16:19 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay12(&mut self) -> DELAY_W<'_, DELAYRHrs> {
        DELAY_W::new(self, 16)
    }
    ///Bits 20:23 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay13(&mut self) -> DELAY_W<'_, DELAYRHrs> {
        DELAY_W::new(self, 20)
    }
    ///Bits 24:27 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay14(&mut self) -> DELAY_W<'_, DELAYRHrs> {
        DELAY_W::new(self, 24)
    }
    ///Bits 28:31 - Port x I/O pin y delay setup
    #[inline(always)]
    pub fn delay15(&mut self) -> DELAY_W<'_, DELAYRHrs> {
        DELAY_W::new(self, 28)
    }
}
/**GPIO port P delay high register

You can [`read`](crate::Reg::read) this register and get [`delayrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delayrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPIOP:DELAYRH)*/
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
