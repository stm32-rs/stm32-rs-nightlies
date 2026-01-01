///Register `AFRH` reader
pub type R = crate::R<AFRHrs>;
///Register `AFRH` writer
pub type W = crate::W<AFRHrs>;
///Field `AFSEL(8-15)` reader - Alternate function selection for port x pin y (y = 8..15)
pub use super::afrl::AFSEL_R;
///Field `AFSEL(8-15)` writer - Alternate function selection for port x pin y (y = 8..15)
pub use super::afrl::AFSEL_W;
///Alternate function selection for port x pin y (y = 8..15)
pub use super::afrl::ALTERNATE_FUNCTION;
impl R {
    ///Alternate function selection for port x pin y (y = 8..15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFSEL8` field.</div>
    #[inline(always)]
    pub fn afsel(&self, n: u8) -> AFSEL_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFSEL_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel_iter(&self) -> impl Iterator<Item = AFSEL_R> + '_ {
        (0..8).map(move |n| AFSEL_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    ///Bits 0:3 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL_R {
        AFSEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL_R {
        AFSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL_R {
        AFSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL_R {
        AFSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL_R {
        AFSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL_R {
        AFSEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL_R {
        AFSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL_R {
        AFSEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRH")
            .field("afsel8", &self.afsel8())
            .field("afsel9", &self.afsel9())
            .field("afsel10", &self.afsel10())
            .field("afsel11", &self.afsel11())
            .field("afsel12", &self.afsel12())
            .field("afsel13", &self.afsel13())
            .field("afsel14", &self.afsel14())
            .field("afsel15", &self.afsel15())
            .finish()
    }
}
impl W {
    ///Alternate function selection for port x pin y (y = 8..15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFSEL8` field.</div>
    #[inline(always)]
    pub fn afsel(&mut self, n: u8) -> AFSEL_W<'_, AFRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFSEL_W::new(self, n * 4)
    }
    ///Bits 0:3 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel8(&mut self) -> AFSEL_W<'_, AFRHrs> {
        AFSEL_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel9(&mut self) -> AFSEL_W<'_, AFRHrs> {
        AFSEL_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel10(&mut self) -> AFSEL_W<'_, AFRHrs> {
        AFSEL_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel11(&mut self) -> AFSEL_W<'_, AFRHrs> {
        AFSEL_W::new(self, 12)
    }
    ///Bits 16:19 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel12(&mut self) -> AFSEL_W<'_, AFRHrs> {
        AFSEL_W::new(self, 16)
    }
    ///Bits 20:23 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel13(&mut self) -> AFSEL_W<'_, AFRHrs> {
        AFSEL_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel14(&mut self) -> AFSEL_W<'_, AFRHrs> {
        AFSEL_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port x pin y (y = 8..15)
    #[inline(always)]
    pub fn afsel15(&mut self) -> AFSEL_W<'_, AFRHrs> {
        AFSEL_W::new(self, 28)
    }
}
/**GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#GPIOA:AFRH)*/
pub struct AFRHrs;
impl crate::RegisterSpec for AFRHrs {
    type Ux = u32;
}
///`read()` method returns [`afrh::R`](R) reader structure
impl crate::Readable for AFRHrs {}
///`write(|w| ..)` method takes [`afrh::W`](W) writer structure
impl crate::Writable for AFRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFRH to value 0
impl crate::Resettable for AFRHrs {}
