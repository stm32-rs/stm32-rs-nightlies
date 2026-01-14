///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `ENABLE` reader - Counter enable
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - Counter enable
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TICKINT` reader - SysTick exception request enable
pub type TICKINT_R = crate::BitReader;
///Field `TICKINT` writer - SysTick exception request enable
pub type TICKINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKSOURCE` reader - Clock source selection
pub type CLKSOURCE_R = crate::BitReader;
///Field `CLKSOURCE` writer - Clock source selection
pub type CLKSOURCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COUNTFLAG` reader - COUNTFLAG
pub type COUNTFLAG_R = crate::BitReader;
///Field `COUNTFLAG` writer - COUNTFLAG
pub type COUNTFLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SysTick exception request enable
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clock source selection
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - COUNTFLAG
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("enable", &self.enable())
            .field("tickint", &self.tickint())
            .field("clksource", &self.clksource())
            .field("countflag", &self.countflag())
            .finish()
    }
}
impl W {
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, CTRLrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bit 1 - SysTick exception request enable
    #[inline(always)]
    pub fn tickint(&mut self) -> TICKINT_W<'_, CTRLrs> {
        TICKINT_W::new(self, 1)
    }
    ///Bit 2 - Clock source selection
    #[inline(always)]
    pub fn clksource(&mut self) -> CLKSOURCE_W<'_, CTRLrs> {
        CLKSOURCE_W::new(self, 2)
    }
    ///Bit 16 - COUNTFLAG
    #[inline(always)]
    pub fn countflag(&mut self) -> COUNTFLAG_W<'_, CTRLrs> {
        COUNTFLAG_W::new(self, 16)
    }
}
/**SysTick control and status register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#STK:CTRL)*/
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {}
