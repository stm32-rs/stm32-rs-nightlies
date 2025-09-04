///Register `IE` reader
pub type R = crate::R<IErs>;
///Register `IE` writer
pub type W = crate::W<IErs>;
///Field `CWEE` reader - CWEE
pub type CWEE_R = crate::BitReader;
///Field `CWEE` writer - CWEE
pub type CWEE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSCE` reader - CSCE
pub type CSCE_R = crate::BitReader;
///Field `CSCE` writer - CSCE
pub type CSCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CWEE
    #[inline(always)]
    pub fn cwee(&self) -> CWEE_R {
        CWEE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CSCE
    #[inline(always)]
    pub fn csce(&self) -> CSCE_R {
        CSCE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("cwee", &self.cwee())
            .field("csce", &self.csce())
            .finish()
    }
}
impl W {
    ///Bit 0 - CWEE
    #[inline(always)]
    pub fn cwee(&mut self) -> CWEE_W<IErs> {
        CWEE_W::new(self, 0)
    }
    ///Bit 1 - CSCE
    #[inline(always)]
    pub fn csce(&mut self) -> CSCE_W<IErs> {
        CSCE_W::new(self, 1)
    }
}
/**The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line.

You can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CCU:IE)*/
pub struct IErs;
impl crate::RegisterSpec for IErs {
    type Ux = u32;
}
///`read()` method returns [`ie::R`](R) reader structure
impl crate::Readable for IErs {}
///`write(|w| ..)` method takes [`ie::W`](W) writer structure
impl crate::Writable for IErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IE to value 0
impl crate::Resettable for IErs {}
