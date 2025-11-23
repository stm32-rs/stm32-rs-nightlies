///Register `ECRCR` reader
pub type R = crate::R<ECRCRrs>;
///Register `ECRCR` writer
pub type W = crate::W<ECRCRrs>;
///Field `ECRC` reader - expected CRC of frame
pub type ECRC_R = crate::FieldReader<u16>;
///Field `ECRC` writer - expected CRC of frame
pub type ECRC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - expected CRC of frame
    #[inline(always)]
    pub fn ecrc(&self) -> ECRC_R {
        ECRC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECRCR").field("ecrc", &self.ecrc()).finish()
    }
}
impl W {
    ///Bits 0:15 - expected CRC of frame
    #[inline(always)]
    pub fn ecrc(&mut self) -> ECRC_W<'_, ECRCRrs> {
        ECRC_W::new(self, 0)
    }
}
/**LTDC expected CRC register

You can [`read`](crate::Reg::read) this register and get [`ecrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:ECRCR)*/
pub struct ECRCRrs;
impl crate::RegisterSpec for ECRCRrs {
    type Ux = u32;
}
///`read()` method returns [`ecrcr::R`](R) reader structure
impl crate::Readable for ECRCRrs {}
///`write(|w| ..)` method takes [`ecrcr::W`](W) writer structure
impl crate::Writable for ECRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECRCR to value 0
impl crate::Resettable for ECRCRrs {}
