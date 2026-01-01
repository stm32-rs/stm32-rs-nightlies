///Register `C3DAR` reader
pub type R = crate::R<C3DARrs>;
///Register `C3DAR` writer
pub type W = crate::W<C3DARrs>;
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
        f.debug_struct("C3DAR").field("da", &self.da()).finish()
    }
}
impl W {
    ///Bits 0:31 - destination address
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, C3DARrs> {
        DA_W::new(self, 0)
    }
}
/**GPDMA channel 3 destination address register

You can [`read`](crate::Reg::read) this register and get [`c3dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPDMA:C3DAR)*/
pub struct C3DARrs;
impl crate::RegisterSpec for C3DARrs {
    type Ux = u32;
}
///`read()` method returns [`c3dar::R`](R) reader structure
impl crate::Readable for C3DARrs {}
///`write(|w| ..)` method takes [`c3dar::W`](W) writer structure
impl crate::Writable for C3DARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C3DAR to value 0
impl crate::Resettable for C3DARrs {}
