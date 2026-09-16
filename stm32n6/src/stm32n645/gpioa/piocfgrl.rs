///Register `PIOCFGRL` reader
pub type R = crate::R<PIOCFGRLrs>;
///Register `PIOCFGRL` writer
pub type W = crate::W<PIOCFGRLrs>;
///Field `PIOCFG(0-7)` reader - Port x I/O pin y configuration
pub type PIOCFG_R = crate::FieldReader;
///Field `PIOCFG(0-7)` writer - Port x I/O pin y configuration
pub type PIOCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Port x I/O pin y configuration
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PIOCFG0` field.</div>
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
    pub fn piocfg0(&self) -> PIOCFG_R {
        PIOCFG_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg1(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg2(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg3(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg4(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg5(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg6(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg7(&self) -> PIOCFG_R {
        PIOCFG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIOCFGRL")
            .field("piocfg0", &self.piocfg0())
            .field("piocfg1", &self.piocfg1())
            .field("piocfg2", &self.piocfg2())
            .field("piocfg3", &self.piocfg3())
            .field("piocfg4", &self.piocfg4())
            .field("piocfg5", &self.piocfg5())
            .field("piocfg6", &self.piocfg6())
            .field("piocfg7", &self.piocfg7())
            .finish()
    }
}
impl W {
    ///Port x I/O pin y configuration
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PIOCFG0` field.</div>
    #[inline(always)]
    pub fn piocfg(&mut self, n: u8) -> PIOCFG_W<'_, PIOCFGRLrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PIOCFG_W::new(self, n * 4)
    }
    ///Bits 0:3 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg0(&mut self) -> PIOCFG_W<'_, PIOCFGRLrs> {
        PIOCFG_W::new(self, 0)
    }
    ///Bits 4:7 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg1(&mut self) -> PIOCFG_W<'_, PIOCFGRLrs> {
        PIOCFG_W::new(self, 4)
    }
    ///Bits 8:11 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg2(&mut self) -> PIOCFG_W<'_, PIOCFGRLrs> {
        PIOCFG_W::new(self, 8)
    }
    ///Bits 12:15 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg3(&mut self) -> PIOCFG_W<'_, PIOCFGRLrs> {
        PIOCFG_W::new(self, 12)
    }
    ///Bits 16:19 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg4(&mut self) -> PIOCFG_W<'_, PIOCFGRLrs> {
        PIOCFG_W::new(self, 16)
    }
    ///Bits 20:23 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg5(&mut self) -> PIOCFG_W<'_, PIOCFGRLrs> {
        PIOCFG_W::new(self, 20)
    }
    ///Bits 24:27 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg6(&mut self) -> PIOCFG_W<'_, PIOCFGRLrs> {
        PIOCFG_W::new(self, 24)
    }
    ///Bits 28:31 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg7(&mut self) -> PIOCFG_W<'_, PIOCFGRLrs> {
        PIOCFG_W::new(self, 28)
    }
}
/**GPIO port A PIO control low register

You can [`read`](crate::Reg::read) this register and get [`piocfgrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`piocfgrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOA:PIOCFGRL)*/
pub struct PIOCFGRLrs;
impl crate::RegisterSpec for PIOCFGRLrs {
    type Ux = u32;
}
///`read()` method returns [`piocfgrl::R`](R) reader structure
impl crate::Readable for PIOCFGRLrs {}
///`write(|w| ..)` method takes [`piocfgrl::W`](W) writer structure
impl crate::Writable for PIOCFGRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIOCFGRL to value 0
impl crate::Resettable for PIOCFGRLrs {}
