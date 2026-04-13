///Register `NVDSSA` reader
pub type R = crate::R<NVDSSArs>;
///Register `NVDSSA` writer
pub type W = crate::W<NVDSSArs>;
///Field `ADD` reader - Non-volatile data segment start address
pub type ADD_R = crate::FieldReader<u16>;
///Field `ADD` writer - Non-volatile data segment start address
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 8:23 - Non-volatile data segment start address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVDSSA").field("add", &self.add()).finish()
    }
}
impl W {
    ///Bits 8:23 - Non-volatile data segment start address
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, NVDSSArs> {
        ADD_W::new(self, 8)
    }
}
/**Non-volatile data segment start address

You can [`read`](crate::Reg::read) this register and get [`nvdssa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvdssa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#FIREWALL:NVDSSA)*/
pub struct NVDSSArs;
impl crate::RegisterSpec for NVDSSArs {
    type Ux = u32;
}
///`read()` method returns [`nvdssa::R`](R) reader structure
impl crate::Readable for NVDSSArs {}
///`write(|w| ..)` method takes [`nvdssa::W`](W) writer structure
impl crate::Writable for NVDSSArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NVDSSA to value 0
impl crate::Resettable for NVDSSArs {}
