///Register `DAR` reader
pub type R = crate::R<DARrs>;
///Register `DAR` writer
pub type W = crate::W<DARrs>;
///Field `DAR` reader - Destination adr base
pub type DAR_R = crate::FieldReader<u32>;
///Field `DAR` writer - Destination adr base
pub type DAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Destination adr base
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAR").field("dar", &self.dar()).finish()
    }
}
impl W {
    ///Bits 0:31 - Destination adr base
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W<'_, DARrs> {
        DAR_W::new(self, 0)
    }
}
/**MDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DARrs;
impl crate::RegisterSpec for DARrs {
    type Ux = u32;
}
///`read()` method returns [`dar::R`](R) reader structure
impl crate::Readable for DARrs {}
///`write(|w| ..)` method takes [`dar::W`](W) writer structure
impl crate::Writable for DARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAR to value 0
impl crate::Resettable for DARrs {}
