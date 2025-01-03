///Register `CMAR2` reader
pub type R = crate::R<CMAR2rs>;
///Register `CMAR2` writer
pub type W = crate::W<CMAR2rs>;
///Field `MA` reader - peripheral address
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - peripheral address
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMAR2").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<CMAR2rs> {
        MA_W::new(self, 0)
    }
}
/**channel x memory address register

You can [`read`](crate::Reg::read) this register and get [`cmar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#DMA1:CMAR2)*/
pub struct CMAR2rs;
impl crate::RegisterSpec for CMAR2rs {
    type Ux = u32;
}
///`read()` method returns [`cmar2::R`](R) reader structure
impl crate::Readable for CMAR2rs {}
///`write(|w| ..)` method takes [`cmar2::W`](W) writer structure
impl crate::Writable for CMAR2rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMAR2 to value 0
impl crate::Resettable for CMAR2rs {
    const RESET_VALUE: u32 = 0;
}
