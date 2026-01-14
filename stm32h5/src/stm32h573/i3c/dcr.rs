///Register `DCR` reader
pub type R = crate::R<DCRrs>;
///Register `DCR` writer
pub type W = crate::W<DCRrs>;
///Field `DCR` reader - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register
pub type DCR_R = crate::FieldReader;
///Field `DCR` writer - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register
pub type DCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register
    #[inline(always)]
    pub fn dcr(&self) -> DCR_R {
        DCR_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR").field("dcr", &self.dcr()).finish()
    }
}
impl W {
    ///Bits 0:7 - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register
    #[inline(always)]
    pub fn dcr(&mut self) -> DCR_W<'_, DCRrs> {
        DCR_W::new(self, 0)
    }
}
/**I3C device characteristics register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#I3C:DCR)*/
pub struct DCRrs;
impl crate::RegisterSpec for DCRrs {
    type Ux = u32;
}
///`read()` method returns [`dcr::R`](R) reader structure
impl crate::Readable for DCRrs {}
///`write(|w| ..)` method takes [`dcr::W`](W) writer structure
impl crate::Writable for DCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR to value 0
impl crate::Resettable for DCRrs {}
