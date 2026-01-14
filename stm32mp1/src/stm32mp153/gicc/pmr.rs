///Register `PMR` reader
pub type R = crate::R<PMRrs>;
///Register `PMR` writer
pub type W = crate::W<PMRrs>;
///Field `PRIORITY` reader - PRIORITY
pub type PRIORITY_R = crate::FieldReader;
///Field `PRIORITY` writer - PRIORITY
pub type PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 3:7 - PRIORITY
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMR")
            .field("priority", &self.priority())
            .finish()
    }
}
impl W {
    ///Bits 3:7 - PRIORITY
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W<'_, PMRrs> {
        PRIORITY_W::new(self, 3)
    }
}
/**GICC input priority mask register

You can [`read`](crate::Reg::read) this register and get [`pmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:PMR)*/
pub struct PMRrs;
impl crate::RegisterSpec for PMRrs {
    type Ux = u32;
}
///`read()` method returns [`pmr::R`](R) reader structure
impl crate::Readable for PMRrs {}
///`write(|w| ..)` method takes [`pmr::W`](W) writer structure
impl crate::Writable for PMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMR to value 0
impl crate::Resettable for PMRrs {}
