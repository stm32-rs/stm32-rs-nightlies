///Register `OAR` reader
pub type R = crate::R<OARrs>;
///Register `OAR` writer
pub type W = crate::W<OARrs>;
///Field `OA` reader - Own address
pub type OA_R = crate::FieldReader;
///Field `OA` writer - Own address
pub type OA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Own address
    #[inline(always)]
    pub fn oa(&self) -> OA_R {
        OA_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR").field("oa", &self.oa()).finish()
    }
}
impl W {
    ///Bits 0:3 - Own address
    #[inline(always)]
    pub fn oa(&mut self) -> OA_W<'_, OARrs> {
        OA_W::new(self, 0)
    }
}
/**CEC own address register

You can [`read`](crate::Reg::read) this register and get [`oar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:OAR)*/
pub struct OARrs;
impl crate::RegisterSpec for OARrs {
    type Ux = u32;
}
///`read()` method returns [`oar::R`](R) reader structure
impl crate::Readable for OARrs {}
///`write(|w| ..)` method takes [`oar::W`](W) writer structure
impl crate::Writable for OARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OAR to value 0
impl crate::Resettable for OARrs {}
