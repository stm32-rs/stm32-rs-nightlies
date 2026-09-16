///Register `PIOCFGRH` reader
pub type R = crate::R<PIOCFGRHrs>;
///Register `PIOCFGRH` writer
pub type W = crate::W<PIOCFGRHrs>;
///Field `PIOCFG(8-15)` reader - Port x I/O pin y configuration
pub type PIOCFG_R = crate::FieldReader;
///Field `PIOCFG(8-15)` writer - Port x I/O pin y configuration
pub type PIOCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Port x I/O pin y configuration
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PIOCFG8` field.</div>
    #[inline(always)]
    pub fn piocfg(&self, n: u8) -> PIOCFG_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PIOCFG_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg_iter(&self) -> impl Iterator<Item = PIOCFG_R> + '_ {
        (0..8).map(move |n| PIOCFG_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    ///Bits 0:3 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg8(&self) -> PIOCFG_R {
        PIOCFG_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg9(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg10(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg11(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg12(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg13(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg14(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg15(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIOCFGRH")
            .field("piocfg8", &self.piocfg8())
            .field("piocfg9", &self.piocfg9())
            .field("piocfg10", &self.piocfg10())
            .field("piocfg11", &self.piocfg11())
            .field("piocfg12", &self.piocfg12())
            .field("piocfg13", &self.piocfg13())
            .field("piocfg14", &self.piocfg14())
            .field("piocfg15", &self.piocfg15())
            .finish()
    }
}
impl W {
    ///Port x I/O pin y configuration
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PIOCFG8` field.</div>
    #[inline(always)]
    pub fn piocfg(&mut self, n: u8) -> PIOCFG_W<'_, PIOCFGRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PIOCFG_W::new(self, n * 4)
    }
    ///Bits 0:3 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg8(&mut self) -> PIOCFG_W<'_, PIOCFGRHrs> {
        PIOCFG_W::new(self, 0)
    }
    ///Bits 4:7 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg9(&mut self) -> PIOCFG_W<'_, PIOCFGRHrs> {
        PIOCFG_W::new(self, 4)
    }
    ///Bits 8:11 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg10(&mut self) -> PIOCFG_W<'_, PIOCFGRHrs> {
        PIOCFG_W::new(self, 8)
    }
    ///Bits 12:15 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg11(&mut self) -> PIOCFG_W<'_, PIOCFGRHrs> {
        PIOCFG_W::new(self, 12)
    }
    ///Bits 16:19 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg12(&mut self) -> PIOCFG_W<'_, PIOCFGRHrs> {
        PIOCFG_W::new(self, 16)
    }
    ///Bits 20:23 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg13(&mut self) -> PIOCFG_W<'_, PIOCFGRHrs> {
        PIOCFG_W::new(self, 20)
    }
    ///Bits 24:27 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg14(&mut self) -> PIOCFG_W<'_, PIOCFGRHrs> {
        PIOCFG_W::new(self, 24)
    }
    ///Bits 28:31 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg15(&mut self) -> PIOCFG_W<'_, PIOCFGRHrs> {
        PIOCFG_W::new(self, 28)
    }
}
/**GPIO port F PIO control high register

You can [`read`](crate::Reg::read) this register and get [`piocfgrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`piocfgrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GPIOF:PIOCFGRH)*/
pub struct PIOCFGRHrs;
impl crate::RegisterSpec for PIOCFGRHrs {
    type Ux = u32;
}
///`read()` method returns [`piocfgrh::R`](R) reader structure
impl crate::Readable for PIOCFGRHrs {}
///`write(|w| ..)` method takes [`piocfgrh::W`](W) writer structure
impl crate::Writable for PIOCFGRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIOCFGRH to value 0
impl crate::Resettable for PIOCFGRHrs {}
