///Register `C6DAR` reader
pub type R = crate::R<C6DARrs>;
///Register `C6DAR` writer
pub type W = crate::W<C6DARrs>;
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
        f.debug_struct("C6DAR").field("da", &self.da()).finish()
    }
}
impl W {
    ///Bits 0:31 - destination address
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, C6DARrs> {
        DA_W::new(self, 0)
    }
}
/**GPDMA channel 6 destination address register

You can [`read`](crate::Reg::read) this register and get [`c6dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GPDMA:C6DAR)*/
pub struct C6DARrs;
impl crate::RegisterSpec for C6DARrs {
    type Ux = u32;
}
///`read()` method returns [`c6dar::R`](R) reader structure
impl crate::Readable for C6DARrs {}
///`write(|w| ..)` method takes [`c6dar::W`](W) writer structure
impl crate::Writable for C6DARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C6DAR to value 0
impl crate::Resettable for C6DARrs {}
