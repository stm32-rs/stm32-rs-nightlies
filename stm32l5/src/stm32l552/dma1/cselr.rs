///Register `CSELR` reader
pub type R = crate::R<CSELRrs>;
///Register `CSELR` writer
pub type W = crate::W<CSELRrs>;
///Field `MA` reader - peripheral address
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - peripheral address
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSELR").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<CSELRrs> {
        MA_W::new(self, 0)
    }
}
/**channel selection register

You can [`read`](crate::Reg::read) this register and get [`cselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DMA1:CSELR)*/
pub struct CSELRrs;
impl crate::RegisterSpec for CSELRrs {
    type Ux = u32;
}
///`read()` method returns [`cselr::R`](R) reader structure
impl crate::Readable for CSELRrs {}
///`write(|w| ..)` method takes [`cselr::W`](W) writer structure
impl crate::Writable for CSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSELR to value 0
impl crate::Resettable for CSELRrs {
    const RESET_VALUE: u32 = 0;
}
