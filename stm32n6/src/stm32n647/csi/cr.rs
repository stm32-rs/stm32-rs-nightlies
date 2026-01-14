///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CSIEN` reader - CSI-2 enable
pub type CSIEN_R = crate::BitReader;
///Field `CSIEN` writer - CSI-2 enable
pub type CSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC0START` writer - Virtual channel 0 start
pub type VC0START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC0STOP` writer - Virtual channel 0 stop
pub type VC0STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC1START` writer - Virtual channel 1 start
pub type VC1START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC1STOP` writer - Virtual channel 1 stop
pub type VC1STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC2START` writer - Virtual channel 2 start
pub type VC2START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC2STOP` writer - Virtual channel 2 stop
pub type VC2STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC3START` writer - Virtual channel 3 start
pub type VC3START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC3STOP` writer - Virtual channel 3 stop
pub type VC3STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CSI-2 enable
    #[inline(always)]
    pub fn csien(&self) -> CSIEN_R {
        CSIEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR").field("csien", &self.csien()).finish()
    }
}
impl W {
    ///Bit 0 - CSI-2 enable
    #[inline(always)]
    pub fn csien(&mut self) -> CSIEN_W<'_, CRrs> {
        CSIEN_W::new(self, 0)
    }
    ///Bit 2 - Virtual channel 0 start
    #[inline(always)]
    pub fn vc0start(&mut self) -> VC0START_W<'_, CRrs> {
        VC0START_W::new(self, 2)
    }
    ///Bit 3 - Virtual channel 0 stop
    #[inline(always)]
    pub fn vc0stop(&mut self) -> VC0STOP_W<'_, CRrs> {
        VC0STOP_W::new(self, 3)
    }
    ///Bit 6 - Virtual channel 1 start
    #[inline(always)]
    pub fn vc1start(&mut self) -> VC1START_W<'_, CRrs> {
        VC1START_W::new(self, 6)
    }
    ///Bit 7 - Virtual channel 1 stop
    #[inline(always)]
    pub fn vc1stop(&mut self) -> VC1STOP_W<'_, CRrs> {
        VC1STOP_W::new(self, 7)
    }
    ///Bit 10 - Virtual channel 2 start
    #[inline(always)]
    pub fn vc2start(&mut self) -> VC2START_W<'_, CRrs> {
        VC2START_W::new(self, 10)
    }
    ///Bit 11 - Virtual channel 2 stop
    #[inline(always)]
    pub fn vc2stop(&mut self) -> VC2STOP_W<'_, CRrs> {
        VC2STOP_W::new(self, 11)
    }
    ///Bit 14 - Virtual channel 3 start
    #[inline(always)]
    pub fn vc3start(&mut self) -> VC3START_W<'_, CRrs> {
        VC3START_W::new(self, 14)
    }
    ///Bit 15 - Virtual channel 3 stop
    #[inline(always)]
    pub fn vc3stop(&mut self) -> VC3STOP_W<'_, CRrs> {
        VC3STOP_W::new(self, 15)
    }
}
/**CSI-2 Host control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CSI:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
