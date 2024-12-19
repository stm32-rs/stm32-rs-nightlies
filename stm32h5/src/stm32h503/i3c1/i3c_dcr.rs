///Register `I3C_DCR` reader
pub type R = crate::R<I3C_DCRrs>;
///Register `I3C_DCR` writer
pub type W = crate::W<I3C_DCRrs>;
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
        f.debug_struct("I3C_DCR").field("dcr", &self.dcr()).finish()
    }
}
impl W {
    ///Bits 0:7 - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register
    #[inline(always)]
    pub fn dcr(&mut self) -> DCR_W<I3C_DCRrs> {
        DCR_W::new(self, 0)
    }
}
/**I3C device characteristics register

You can [`read`](crate::Reg::read) this register and get [`i3c_dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_DCR)*/
pub struct I3C_DCRrs;
impl crate::RegisterSpec for I3C_DCRrs {
    type Ux = u32;
}
///`read()` method returns [`i3c_dcr::R`](R) reader structure
impl crate::Readable for I3C_DCRrs {}
///`write(|w| ..)` method takes [`i3c_dcr::W`](W) writer structure
impl crate::Writable for I3C_DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I3C_DCR to value 0
impl crate::Resettable for I3C_DCRrs {
    const RESET_VALUE: u32 = 0;
}
