///Register `MDR` reader
pub type R = crate::R<MDRrs>;
///Register `MDR` writer
pub type W = crate::W<MDRrs>;
///Field `MDR` reader - Mask data
pub type MDR_R = crate::FieldReader<u32>;
///Field `MDR` writer - Mask data
pub type MDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Mask data
    #[inline(always)]
    pub fn mdr(&self) -> MDR_R {
        MDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDR").field("mdr", &self.mdr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Mask data
    #[inline(always)]
    pub fn mdr(&mut self) -> MDR_W<'_, MDRrs> {
        MDR_W::new(self, 0)
    }
}
/**MDMA channel x Mask Data register

You can [`read`](crate::Reg::read) this register and get [`mdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MDRrs;
impl crate::RegisterSpec for MDRrs {
    type Ux = u32;
}
///`read()` method returns [`mdr::R`](R) reader structure
impl crate::Readable for MDRrs {}
///`write(|w| ..)` method takes [`mdr::W`](W) writer structure
impl crate::Writable for MDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MDR to value 0
impl crate::Resettable for MDRrs {}
