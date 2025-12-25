///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
///Field `OSPEEDR(0-15)` reader - Port x configuration pin %s
pub use crate::stm32f429::gpioa::ospeedr::OSPEEDR_R;
///Field `OSPEEDR(0-15)` writer - Port x configuration pin %s
pub use crate::stm32f429::gpioa::ospeedr::OSPEEDR_W;
///Port x configuration pin %s
pub use crate::stm32f429::gpioa::ospeedr::OUTPUT_SPEED;
impl R {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OSPEEDR0` field.</div>
    #[inline(always)]
    pub fn ospeedr(&self, n: u8) -> OSPEEDR_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OSPEEDR_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-15)
    #[inline(always)]
    pub fn ospeedr_iter(&self) -> impl Iterator<Item = OSPEEDR_R> + '_ {
        (0..16).map(move |n| OSPEEDR_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR_R {
        OSPEEDR_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn ospeedr7(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn ospeedr8(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn ospeedr9(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn ospeedr10(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn ospeedr11(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn ospeedr12(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeedr0", &self.ospeedr0())
            .field("ospeedr1", &self.ospeedr1())
            .field("ospeedr2", &self.ospeedr2())
            .field("ospeedr3", &self.ospeedr3())
            .field("ospeedr4", &self.ospeedr4())
            .field("ospeedr5", &self.ospeedr5())
            .field("ospeedr6", &self.ospeedr6())
            .field("ospeedr7", &self.ospeedr7())
            .field("ospeedr8", &self.ospeedr8())
            .field("ospeedr9", &self.ospeedr9())
            .field("ospeedr10", &self.ospeedr10())
            .field("ospeedr11", &self.ospeedr11())
            .field("ospeedr12", &self.ospeedr12())
            .field("ospeedr13", &self.ospeedr13())
            .field("ospeedr14", &self.ospeedr14())
            .field("ospeedr15", &self.ospeedr15())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OSPEEDR0` field.</div>
    #[inline(always)]
    pub fn ospeedr(&mut self, n: u8) -> OSPEEDR_W<'_, OSPEEDRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OSPEEDR_W::new(self, n * 2)
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn ospeedr5(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn ospeedr6(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn ospeedr7(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn ospeedr8(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn ospeedr9(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn ospeedr10(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn ospeedr11(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn ospeedr12(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn ospeedr13(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn ospeedr14(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn ospeedr15(&mut self) -> OSPEEDR_W<'_, OSPEEDRrs> {
        OSPEEDR_W::new(self, 30)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#GPIOK:OSPEEDR)*/
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`ospeedr::R`](R) reader structure
impl crate::Readable for OSPEEDRrs {}
///`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OSPEEDR to value 0
impl crate::Resettable for OSPEEDRrs {}
