///Register `AFRH` reader
pub type R = crate::R<AFRHrs>;
///Register `AFRH` writer
pub type W = crate::W<AFRHrs>;
///Field `AFR(8-15)` reader - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
pub use super::afrl::AFR_R;
///Field `AFR(8-15)` writer - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
pub use super::afrl::AFR_W;
///3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
pub use super::afrl::ALTERNATE_FUNCTION;
impl R {
    ///3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFR8` field.</div>
    #[inline(always)]
    pub fn afr(&self, n: u8) -> AFR_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr_iter(&self) -> impl Iterator<Item = AFR_R> + '_ {
        (0..8).map(move |n| AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    ///Bits 0:3 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr8(&self) -> AFR_R {
        AFR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr9(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr10(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr11(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr12(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr13(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr14(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr15(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRH")
            .field("afr8", &self.afr8())
            .field("afr9", &self.afr9())
            .field("afr10", &self.afr10())
            .field("afr11", &self.afr11())
            .field("afr12", &self.afr12())
            .field("afr13", &self.afr13())
            .field("afr14", &self.afr14())
            .field("afr15", &self.afr15())
            .finish()
    }
}
impl W {
    ///3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFR8` field.</div>
    #[inline(always)]
    pub fn afr(&mut self, n: u8) -> AFR_W<'_, AFRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_W::new(self, n * 4)
    }
    ///Bits 0:3 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr8(&mut self) -> AFR_W<'_, AFRHrs> {
        AFR_W::new(self, 0)
    }
    ///Bits 4:7 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr9(&mut self) -> AFR_W<'_, AFRHrs> {
        AFR_W::new(self, 4)
    }
    ///Bits 8:11 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr10(&mut self) -> AFR_W<'_, AFRHrs> {
        AFR_W::new(self, 8)
    }
    ///Bits 12:15 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr11(&mut self) -> AFR_W<'_, AFRHrs> {
        AFR_W::new(self, 12)
    }
    ///Bits 16:19 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr12(&mut self) -> AFR_W<'_, AFRHrs> {
        AFR_W::new(self, 16)
    }
    ///Bits 20:23 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr13(&mut self) -> AFR_W<'_, AFRHrs> {
        AFR_W::new(self, 20)
    }
    ///Bits 24:27 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr14(&mut self) -> AFR_W<'_, AFRHrs> {
        AFR_W::new(self, 24)
    }
    ///Bits 28:31 - 3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afr15(&mut self) -> AFR_W<'_, AFRHrs> {
        AFR_W::new(self, 28)
    }
}
/**GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#GPIOA:AFRH)*/
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
