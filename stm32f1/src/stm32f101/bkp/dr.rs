///Register `DR%s` reader
pub type R = crate::R<DRrs>;
///Register `DR%s` writer
pub type W = crate::W<DRrs>;
///Field `D` reader - Backup data
pub type D_R = crate::FieldReader<u16>;
///Field `D` writer - Backup data
pub type D_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Backup data
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR").field("d", &self.d()).finish()
    }
}
impl W {
    ///Bits 0:15 - Backup data
    #[inline(always)]
    pub fn d(&mut self) -> D_W<'_, DRrs> {
        D_W::new(self, 0)
    }
}
/**Backup data register (BKP_DR)

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#BKP:DR[1])*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`write(|w| ..)` method takes [`dr::W`](W) writer structure
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DR%s to value 0
impl crate::Resettable for DRrs {}
