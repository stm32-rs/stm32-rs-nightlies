///Register `PRIER` reader
pub type R = crate::R<PRIERrs>;
///Register `PRIER` writer
pub type W = crate::W<PRIERrs>;
///Field `ERRIE` reader - Synchronization error interrupt enable
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Synchronization error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - Synchronization error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIER")
            .field("errie", &self.errie())
            .finish()
    }
}
impl W {
    ///Bit 6 - Synchronization error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, PRIERrs> {
        ERRIE_W::new(self, 6)
    }
}
/**DCMIPP parallel interface interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`prier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:PRIER)*/
pub struct PRIERrs;
impl crate::RegisterSpec for PRIERrs {
    type Ux = u32;
}
///`read()` method returns [`prier::R`](R) reader structure
impl crate::Readable for PRIERrs {}
///`write(|w| ..)` method takes [`prier::W`](W) writer structure
impl crate::Writable for PRIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIER to value 0
impl crate::Resettable for PRIERrs {}
