///Register `UNMAPR` reader
pub type R = crate::R<UNMAPRrs>;
///Register `UNMAPR` writer
pub type W = crate::W<UNMAPRrs>;
///Field `UNMAP` reader - Unmap key
pub type UNMAP_R = crate::FieldReader<u32>;
///Field `UNMAP` writer - Unmap key
pub type UNMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Unmap key
    #[inline(always)]
    pub fn unmap(&self) -> UNMAP_R {
        UNMAP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNMAPR")
            .field("unmap", &self.unmap())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Unmap key
    #[inline(always)]
    pub fn unmap(&mut self) -> UNMAP_W<'_, UNMAPRrs> {
        UNMAP_W::new(self, 0)
    }
}
/**BSEC unmap register

You can [`read`](crate::Reg::read) this register and get [`unmapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unmapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:UNMAPR)*/
pub struct UNMAPRrs;
impl crate::RegisterSpec for UNMAPRrs {
    type Ux = u32;
}
///`read()` method returns [`unmapr::R`](R) reader structure
impl crate::Readable for UNMAPRrs {}
///`write(|w| ..)` method takes [`unmapr::W`](W) writer structure
impl crate::Writable for UNMAPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UNMAPR to value 0
impl crate::Resettable for UNMAPRrs {}
