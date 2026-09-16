///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
///Port x configuration pin %s
pub use crate::stm32wb55::gpioa::moder::MODE;
///Field `MODER(0-4)` reader - Port x configuration pin %s
pub use crate::stm32wb55::gpioa::moder::MODER_R;
///Field `MODER(0-4)` writer - Port x configuration pin %s
pub use crate::stm32wb55::gpioa::moder::MODER_W;
impl R {
    ///Port x configuration pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODER0` field.</div>
    #[inline(always)]
    pub fn moder(&self, n: u8) -> MODER_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        MODER_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-4)
    #[inline(always)]
    pub fn moder_iter(&self) -> impl Iterator<Item = MODER_R> + '_ {
        (0..5).map(move |n| MODER_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn moder0(&self) -> MODER_R {
        MODER_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn moder1(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn moder2(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn moder3(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn moder4(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("moder0", &self.moder0())
            .field("moder1", &self.moder1())
            .field("moder2", &self.moder2())
            .field("moder3", &self.moder3())
            .field("moder4", &self.moder4())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODER0` field.</div>
    #[inline(always)]
    pub fn moder(&mut self, n: u8) -> MODER_W<'_, MODERrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        MODER_W::new(self, n * 2)
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn moder0(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn moder1(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn moder2(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn moder3(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn moder4(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 8)
    }
}
/**GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:MODER)*/
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
///`read()` method returns [`moder::R`](R) reader structure
impl crate::Readable for MODERrs {}
///`write(|w| ..)` method takes [`moder::W`](W) writer structure
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MODER to value 0x03ff
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0x03ff;
}
