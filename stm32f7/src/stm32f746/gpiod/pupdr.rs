///Register `PUPDR` reader
pub type R = crate::R<PUPDRrs>;
///Register `PUPDR` writer
pub type W = crate::W<PUPDRrs>;
///Port x configuration pin %s
pub use crate::stm32f746::gpioa::pupdr::PULL;
///Field `PUPDR(0-15)` reader - Port x configuration pin %s
pub use crate::stm32f746::gpioa::pupdr::PUPDR_R;
///Field `PUPDR(0-15)` writer - Port x configuration pin %s
pub use crate::stm32f746::gpioa::pupdr::PUPDR_W;
impl R {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PUPDR0` field.</div>
    #[inline(always)]
    pub fn pupdr(&self, n: u8) -> PUPDR_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PUPDR_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-15)
    #[inline(always)]
    pub fn pupdr_iter(&self) -> impl Iterator<Item = PUPDR_R> + '_ {
        (0..16).map(move |n| PUPDR_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR_R {
        PUPDR_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn pupdr8(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn pupdr9(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn pupdr10(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn pupdr11(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn pupdr12(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPDR")
            .field("pupdr0", &self.pupdr0())
            .field("pupdr1", &self.pupdr1())
            .field("pupdr2", &self.pupdr2())
            .field("pupdr3", &self.pupdr3())
            .field("pupdr4", &self.pupdr4())
            .field("pupdr5", &self.pupdr5())
            .field("pupdr6", &self.pupdr6())
            .field("pupdr7", &self.pupdr7())
            .field("pupdr8", &self.pupdr8())
            .field("pupdr9", &self.pupdr9())
            .field("pupdr10", &self.pupdr10())
            .field("pupdr11", &self.pupdr11())
            .field("pupdr12", &self.pupdr12())
            .field("pupdr13", &self.pupdr13())
            .field("pupdr14", &self.pupdr14())
            .field("pupdr15", &self.pupdr15())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PUPDR0` field.</div>
    #[inline(always)]
    pub fn pupdr(&mut self, n: u8) -> PUPDR_W<'_, PUPDRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PUPDR_W::new(self, n * 2)
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn pupdr0(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn pupdr1(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn pupdr2(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn pupdr3(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn pupdr4(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn pupdr5(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn pupdr6(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn pupdr7(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn pupdr8(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn pupdr9(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn pupdr10(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn pupdr11(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn pupdr12(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn pupdr13(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn pupdr14(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn pupdr15(&mut self) -> PUPDR_W<'_, PUPDRrs> {
        PUPDR_W::new(self, 30)
    }
}
/**GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#GPIOD:PUPDR)*/
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
///`read()` method returns [`pupdr::R`](R) reader structure
impl crate::Readable for PUPDRrs {}
///`write(|w| ..)` method takes [`pupdr::W`](W) writer structure
impl crate::Writable for PUPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUPDR to value 0
impl crate::Resettable for PUPDRrs {}
