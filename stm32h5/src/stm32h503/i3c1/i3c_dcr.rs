#[doc = "Register `I3C_DCR` reader"]
pub type R = crate::R<I3C_DCRrs>;
#[doc = "Register `I3C_DCR` writer"]
pub type W = crate::W<I3C_DCRrs>;
#[doc = "Field `DCR` reader - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
pub type DCR_R = crate::FieldReader;
#[doc = "Field `DCR` writer - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
pub type DCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
    #[inline(always)]
    pub fn dcr(&self) -> DCR_R {
        DCR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
    #[inline(always)]
    #[must_use]
    pub fn dcr(&mut self) -> DCR_W<I3C_DCRrs> {
        DCR_W::new(self, 0)
    }
}
#[doc = "I3C device characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_dcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_dcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_DCRrs;
impl crate::RegisterSpec for I3C_DCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c_dcr::R`](R) reader structure"]
impl crate::Readable for I3C_DCRrs {}
#[doc = "`write(|w| ..)` method takes [`i3c_dcr::W`](W) writer structure"]
impl crate::Writable for I3C_DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I3C_DCR to value 0"]
impl crate::Resettable for I3C_DCRrs {
    const RESET_VALUE: u32 = 0;
}
