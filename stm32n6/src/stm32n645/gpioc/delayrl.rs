///Register `DELAYRL` reader
pub type R = crate::R<DELAYRLrs>;
///Register `DELAYRL` writer
pub type W = crate::W<DELAYRLrs>;
///Field `DELAY(0-7)` reader - Port x IO pin y delay setup
pub type DELAY_R = crate::FieldReader;
///Field `DELAY(0-7)` writer - Port x IO pin y delay setup
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Port x IO pin y delay setup
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DELAY0` field.</div>
    #[inline(always)]
    pub fn delay(&self, n: u8) -> DELAY_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DELAY_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay_iter(&self) -> impl Iterator<Item = DELAY_R> + '_ {
        (0..8).map(move |n| DELAY_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    ///Bits 0:3 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay0(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay1(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay2(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay3(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay4(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay5(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay6(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay7(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 28) & 0x0f) as u8)
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
    ///Port x IO pin y delay setup
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DELAY0` field.</div>
    #[inline(always)]
    pub fn delay(&mut self, n: u8) -> DELAY_W<'_, DELAYRLrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DELAY_W::new(self, n * 4)
    }
    ///Bits 0:3 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay0(&mut self) -> DELAY_W<'_, DELAYRLrs> {
        DELAY_W::new(self, 0)
    }
    ///Bits 4:7 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay1(&mut self) -> DELAY_W<'_, DELAYRLrs> {
        DELAY_W::new(self, 4)
    }
    ///Bits 8:11 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay2(&mut self) -> DELAY_W<'_, DELAYRLrs> {
        DELAY_W::new(self, 8)
    }
    ///Bits 12:15 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay3(&mut self) -> DELAY_W<'_, DELAYRLrs> {
        DELAY_W::new(self, 12)
    }
    ///Bits 16:19 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay4(&mut self) -> DELAY_W<'_, DELAYRLrs> {
        DELAY_W::new(self, 16)
    }
    ///Bits 20:23 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay5(&mut self) -> DELAY_W<'_, DELAYRLrs> {
        DELAY_W::new(self, 20)
    }
    ///Bits 24:27 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay6(&mut self) -> DELAY_W<'_, DELAYRLrs> {
        DELAY_W::new(self, 24)
    }
    ///Bits 28:31 - Port x IO pin y delay setup
    #[inline(always)]
    pub fn delay7(&mut self) -> DELAY_W<'_, DELAYRLrs> {
        DELAY_W::new(self, 28)
    }
}
/**GPIO port C delay low register

You can [`read`](crate::Reg::read) this register and get [`delayrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delayrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOC:DELAYRL)*/
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
