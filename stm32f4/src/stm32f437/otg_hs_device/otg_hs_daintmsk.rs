///Register `OTG_HS_DAINTMSK` reader
pub type R = crate::R<OTG_HS_DAINTMSKrs>;
///Register `OTG_HS_DAINTMSK` writer
pub type W = crate::W<OTG_HS_DAINTMSKrs>;
///Field `IEPM` reader - IN EP interrupt mask bits
pub type IEPM_R = crate::FieldReader<u16>;
///Field `IEPM` writer - IN EP interrupt mask bits
pub type IEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `OEPM` reader - OUT EP interrupt mask bits
pub type OEPM_R = crate::FieldReader<u16>;
///Field `OEPM` writer - OUT EP interrupt mask bits
pub type OEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IN EP interrupt mask bits
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OUT EP interrupt mask bits
    #[inline(always)]
    pub fn oepm(&self) -> OEPM_R {
        OEPM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DAINTMSK")
            .field("iepm", &self.iepm())
            .field("oepm", &self.oepm())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IN EP interrupt mask bits
    #[inline(always)]
    pub fn iepm(&mut self) -> IEPM_W<'_, OTG_HS_DAINTMSKrs> {
        IEPM_W::new(self, 0)
    }
    ///Bits 16:31 - OUT EP interrupt mask bits
    #[inline(always)]
    pub fn oepm(&mut self) -> OEPM_W<'_, OTG_HS_DAINTMSKrs> {
        OEPM_W::new(self, 16)
    }
}
/**OTG_HS all endpoints interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_daintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_daintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_DEVICE:OTG_HS_DAINTMSK)*/
pub struct OTG_HS_DAINTMSKrs;
impl crate::RegisterSpec for OTG_HS_DAINTMSKrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_daintmsk::R`](R) reader structure
impl crate::Readable for OTG_HS_DAINTMSKrs {}
///`write(|w| ..)` method takes [`otg_hs_daintmsk::W`](W) writer structure
impl crate::Writable for OTG_HS_DAINTMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DAINTMSK to value 0
impl crate::Resettable for OTG_HS_DAINTMSKrs {}
