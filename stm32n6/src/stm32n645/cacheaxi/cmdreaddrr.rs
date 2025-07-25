///Register `CMDREADDRR` reader
pub type R = crate::R<CMDREADDRRrs>;
///Register `CMDREADDRR` writer
pub type W = crate::W<CMDREADDRRrs>;
///Field `CMDENDADDR` reader - end address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies
pub type CMDENDADDR_R = crate::FieldReader<u32>;
///Field `CMDENDADDR` writer - end address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies
pub type CMDENDADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 6:31 - end address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies
    #[inline(always)]
    pub fn cmdendaddr(&self) -> CMDENDADDR_R {
        CMDENDADDR_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDREADDRR")
            .field("cmdendaddr", &self.cmdendaddr())
            .finish()
    }
}
impl W {
    ///Bits 6:31 - end address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies
    #[inline(always)]
    pub fn cmdendaddr(&mut self) -> CMDENDADDR_W<CMDREADDRRrs> {
        CMDENDADDR_W::new(self, 6)
    }
}
/**CACHEAXI command range end address register

You can [`read`](crate::Reg::read) this register and get [`cmdreaddrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdreaddrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CACHEAXI:CMDREADDRR)*/
pub struct CMDREADDRRrs;
impl crate::RegisterSpec for CMDREADDRRrs {
    type Ux = u32;
}
///`read()` method returns [`cmdreaddrr::R`](R) reader structure
impl crate::Readable for CMDREADDRRrs {}
///`write(|w| ..)` method takes [`cmdreaddrr::W`](W) writer structure
impl crate::Writable for CMDREADDRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMDREADDRR to value 0
impl crate::Resettable for CMDREADDRRrs {}
