///Register `PUPDR` reader
pub type R = crate::R<PUPDRrs>;
///Register `PUPDR` writer
pub type W = crate::W<PUPDRrs>;
///Port x configuration pin %s
pub use crate::stm32wb55::gpioa::pupdr::PULL;
///Field `PUPDR(0-4)` reader - Port x configuration pin %s
pub use crate::stm32wb55::gpioa::pupdr::PUPDR_R;
///Field `PUPDR(0-4)` writer - Port x configuration pin %s
pub use crate::stm32wb55::gpioa::pupdr::PUPDR_W;
impl R {
    ///Port x configuration pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PUPDR0` field.</div>
    #[inline(always)]
    pub fn pupdr(&self, n: u8) -> PUPDR_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PUPDR_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-4)
    #[inline(always)]
    pub fn pupdr_iter(&self) -> impl Iterator<Item = PUPDR_R> + '_ {
        (0..5).map(move |n| PUPDR_R::new(((self.bits >> (n * 2)) & 3) as u8))
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPDR")
            .field("pupdr0", &self.pupdr0())
            .field("pupdr1", &self.pupdr1())
            .field("pupdr2", &self.pupdr2())
            .field("pupdr3", &self.pupdr3())
            .field("pupdr4", &self.pupdr4())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PUPDR0` field.</div>
    #[inline(always)]
    pub fn pupdr(&mut self, n: u8) -> PUPDR_W<'_, PUPDRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
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
}
/**GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:PUPDR)*/
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
