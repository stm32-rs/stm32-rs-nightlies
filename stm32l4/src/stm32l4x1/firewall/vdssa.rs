///Register `VDSSA` reader
pub type R = crate::R<VDSSArs>;
///Register `VDSSA` writer
pub type W = crate::W<VDSSArs>;
///Field `ADD` reader - Volatile data segment start address
pub type ADD_R = crate::FieldReader<u16>;
///Field `ADD` writer - Volatile data segment start address
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
impl R {
    ///Bits 6:15 - Volatile data segment start address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDSSA").field("add", &self.add()).finish()
    }
}
impl W {
    ///Bits 6:15 - Volatile data segment start address
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, VDSSArs> {
        ADD_W::new(self, 6)
    }
}
/**Volatile data segment start address

You can [`read`](crate::Reg::read) this register and get [`vdssa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdssa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#FIREWALL:VDSSA)*/
pub struct VDSSArs;
impl crate::RegisterSpec for VDSSArs {
    type Ux = u32;
}
///`read()` method returns [`vdssa::R`](R) reader structure
impl crate::Readable for VDSSArs {}
///`write(|w| ..)` method takes [`vdssa::W`](W) writer structure
impl crate::Writable for VDSSArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VDSSA to value 0
impl crate::Resettable for VDSSArs {}
