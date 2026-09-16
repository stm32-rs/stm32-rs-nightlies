///Register `GPOVAL` reader
pub type R = crate::R<GPOVALrs>;
///Register `GPOVAL` writer
pub type W = crate::W<GPOVALrs>;
///Field `HDPGPOVAL` reader - When written, define the value of the HDP GPO.
pub type HDPGPOVAL_R = crate::FieldReader;
///Field `HDPGPOVAL` writer - When written, define the value of the HDP GPO.
pub type HDPGPOVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - When written, define the value of the HDP GPO.
    #[inline(always)]
    pub fn hdpgpoval(&self) -> HDPGPOVAL_R {
        HDPGPOVAL_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPOVAL")
            .field("hdpgpoval", &self.hdpgpoval())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - When written, define the value of the HDP GPO.
    #[inline(always)]
    pub fn hdpgpoval(&mut self) -> HDPGPOVAL_W<'_, GPOVALrs> {
        HDPGPOVAL_W::new(self, 0)
    }
}
/**HDP general purpose output value register

You can [`read`](crate::Reg::read) this register and get [`gpoval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HDP:GPOVAL)*/
pub struct GPOVALrs;
impl crate::RegisterSpec for GPOVALrs {
    type Ux = u32;
}
///`read()` method returns [`gpoval::R`](R) reader structure
impl crate::Readable for GPOVALrs {}
///`write(|w| ..)` method takes [`gpoval::W`](W) writer structure
impl crate::Writable for GPOVALrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPOVAL to value 0
impl crate::Resettable for GPOVALrs {}
