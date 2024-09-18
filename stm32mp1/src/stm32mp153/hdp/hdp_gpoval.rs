///Register `HDP_GPOVAL` reader
pub type R = crate::R<HDP_GPOVALrs>;
///Register `HDP_GPOVAL` writer
pub type W = crate::W<HDP_GPOVALrs>;
///Field `HDPGPOVAL` reader - HDPGPOVAL
pub type HDPGPOVAL_R = crate::FieldReader;
///Field `HDPGPOVAL` writer - HDPGPOVAL
pub type HDPGPOVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - HDPGPOVAL
    #[inline(always)]
    pub fn hdpgpoval(&self) -> HDPGPOVAL_R {
        HDPGPOVAL_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDP_GPOVAL")
            .field("hdpgpoval", &self.hdpgpoval())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - HDPGPOVAL
    #[inline(always)]
    #[must_use]
    pub fn hdpgpoval(&mut self) -> HDPGPOVAL_W<HDP_GPOVALrs> {
        HDPGPOVAL_W::new(self, 0)
    }
}
/**HDP GPO value

You can [`read`](crate::Reg::read) this register and get [`hdp_gpoval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp_gpoval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_GPOVAL)*/
pub struct HDP_GPOVALrs;
impl crate::RegisterSpec for HDP_GPOVALrs {
    type Ux = u32;
}
///`read()` method returns [`hdp_gpoval::R`](R) reader structure
impl crate::Readable for HDP_GPOVALrs {}
///`write(|w| ..)` method takes [`hdp_gpoval::W`](W) writer structure
impl crate::Writable for HDP_GPOVALrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HDP_GPOVAL to value 0
impl crate::Resettable for HDP_GPOVALrs {
    const RESET_VALUE: u32 = 0;
}
