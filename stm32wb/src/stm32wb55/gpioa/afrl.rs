///Register `AFRL` reader
pub type R = crate::R<AFRLrs>;
///Register `AFRL` writer
pub type W = crate::W<AFRLrs>;
///Field `AFR(EL0,EL1,EL2,EL3,EL4,EL5,EL6,EL7)` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFR_R = crate::FieldReader;
///Field `AFR(EL0,EL1,EL2,EL3,EL4,EL5,EL6,EL7)` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Alternate function selection for port x bit y (y = 0..7)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFREL0` field.</div>
    #[inline(always)]
    pub fn afr(&self, n: u8) -> AFR_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afr_iter(&self) -> impl Iterator<Item = AFR_R> + '_ {
        (0..8).map(move |n| AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel0(&self) -> AFR_R {
        AFR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel1(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel2(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel3(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel4(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel5(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel6(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel7(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRL")
            .field("afrel0", &self.afrel0())
            .field("afrel1", &self.afrel1())
            .field("afrel2", &self.afrel2())
            .field("afrel3", &self.afrel3())
            .field("afrel4", &self.afrel4())
            .field("afrel5", &self.afrel5())
            .field("afrel6", &self.afrel6())
            .field("afrel7", &self.afrel7())
            .finish()
    }
}
impl W {
    ///Alternate function selection for port x bit y (y = 0..7)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFREL0` field.</div>
    #[inline(always)]
    pub fn afr(&mut self, n: u8) -> AFR_W<'_, AFRLrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_W::new(self, n * 4)
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel0(&mut self) -> AFR_W<'_, AFRLrs> {
        AFR_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel1(&mut self) -> AFR_W<'_, AFRLrs> {
        AFR_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel2(&mut self) -> AFR_W<'_, AFRLrs> {
        AFR_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel3(&mut self) -> AFR_W<'_, AFRLrs> {
        AFR_W::new(self, 12)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel4(&mut self) -> AFR_W<'_, AFRLrs> {
        AFR_W::new(self, 16)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel5(&mut self) -> AFR_W<'_, AFRLrs> {
        AFR_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel6(&mut self) -> AFR_W<'_, AFRLrs> {
        AFR_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrel7(&mut self) -> AFR_W<'_, AFRLrs> {
        AFR_W::new(self, 28)
    }
}
/**GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOA:AFRL)*/
pub struct AFRLrs;
impl crate::RegisterSpec for AFRLrs {
    type Ux = u32;
}
///`read()` method returns [`afrl::R`](R) reader structure
impl crate::Readable for AFRLrs {}
///`write(|w| ..)` method takes [`afrl::W`](W) writer structure
impl crate::Writable for AFRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFRL to value 0
impl crate::Resettable for AFRLrs {}
