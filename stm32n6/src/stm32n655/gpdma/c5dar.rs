///Register `C5DAR` reader
pub type R = crate::R<C5DARrs>;
///Register `C5DAR` writer
pub type W = crate::W<C5DARrs>;
///Field `DA` reader - destination address
pub type DA_R = crate::FieldReader<u32>;
///Field `DA` writer - destination address
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - destination address
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C5DAR").field("da", &self.da()).finish()
    }
}
impl W {
    ///Bits 0:31 - destination address
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, C5DARrs> {
        DA_W::new(self, 0)
    }
}
/**GPDMA channel 5 destination address register

You can [`read`](crate::Reg::read) this register and get [`c5dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GPDMA:C5DAR)*/
pub struct C5DARrs;
impl crate::RegisterSpec for C5DARrs {
    type Ux = u32;
}
///`read()` method returns [`c5dar::R`](R) reader structure
impl crate::Readable for C5DARrs {}
///`write(|w| ..)` method takes [`c5dar::W`](W) writer structure
impl crate::Writable for C5DARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C5DAR to value 0
impl crate::Resettable for C5DARrs {}
