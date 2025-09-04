///Register `WKUPCR` reader
pub type R = crate::R<WKUPCRrs>;
///Register `WKUPCR` writer
pub type W = crate::W<WKUPCRrs>;
///Field `WKUPC1` reader - Clear Wakeup pin flag for WKUP1 These bits are always read as 0.
pub type WKUPC1_R = crate::BitReader;
///Field `WKUPC1` writer - Clear Wakeup pin flag for WKUP1 These bits are always read as 0.
pub type WKUPC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC2` reader - Clear Wakeup pin flag for WKUP2 These bits are always read as 0.
pub type WKUPC2_R = crate::BitReader;
///Field `WKUPC2` writer - Clear Wakeup pin flag for WKUP2 These bits are always read as 0.
pub type WKUPC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC3` reader - Clear Wakeup pin flag for WKUP3 These bits are always read as 0.
pub type WKUPC3_R = crate::BitReader;
///Field `WKUPC3` writer - Clear Wakeup pin flag for WKUP3 These bits are always read as 0.
pub type WKUPC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC4` reader - Clear Wakeup pin flag for WKUP4 These bits are always read as 0.
pub type WKUPC4_R = crate::BitReader;
///Field `WKUPC4` writer - Clear Wakeup pin flag for WKUP4 These bits are always read as 0.
pub type WKUPC4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear Wakeup pin flag for WKUP1 These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc1(&self) -> WKUPC1_R {
        WKUPC1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear Wakeup pin flag for WKUP2 These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc2(&self) -> WKUPC2_R {
        WKUPC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear Wakeup pin flag for WKUP3 These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc3(&self) -> WKUPC3_R {
        WKUPC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear Wakeup pin flag for WKUP4 These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc4(&self) -> WKUPC4_R {
        WKUPC4_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUPCR")
            .field("wkupc1", &self.wkupc1())
            .field("wkupc2", &self.wkupc2())
            .field("wkupc3", &self.wkupc3())
            .field("wkupc4", &self.wkupc4())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Wakeup pin flag for WKUP1 These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc1(&mut self) -> WKUPC1_W<WKUPCRrs> {
        WKUPC1_W::new(self, 0)
    }
    ///Bit 1 - Clear Wakeup pin flag for WKUP2 These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc2(&mut self) -> WKUPC2_W<WKUPCRrs> {
        WKUPC2_W::new(self, 1)
    }
    ///Bit 2 - Clear Wakeup pin flag for WKUP3 These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc3(&mut self) -> WKUPC3_W<WKUPCRrs> {
        WKUPC3_W::new(self, 2)
    }
    ///Bit 3 - Clear Wakeup pin flag for WKUP4 These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc4(&mut self) -> WKUPC4_W<WKUPCRrs> {
        WKUPC4_W::new(self, 3)
    }
}
/**PWR wakeup clear register

You can [`read`](crate::Reg::read) this register and get [`wkupcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:WKUPCR)*/
pub struct WKUPCRrs;
impl crate::RegisterSpec for WKUPCRrs {
    type Ux = u32;
}
///`read()` method returns [`wkupcr::R`](R) reader structure
impl crate::Readable for WKUPCRrs {}
///`write(|w| ..)` method takes [`wkupcr::W`](W) writer structure
impl crate::Writable for WKUPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WKUPCR to value 0
impl crate::Resettable for WKUPCRrs {}
