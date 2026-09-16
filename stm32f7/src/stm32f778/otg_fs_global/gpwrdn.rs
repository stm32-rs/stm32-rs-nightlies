///Register `GPWRDN` reader
pub type R = crate::R<GPWRDNrs>;
///Register `GPWRDN` writer
pub type W = crate::W<GPWRDNrs>;
///Field `ADPMEN` reader - ADP module enable
pub type ADPMEN_R = crate::BitReader;
///Field `ADPMEN` writer - ADP module enable
pub type ADPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADPIF` reader - ADP interrupt flag
pub type ADPIF_R = crate::BitReader;
///Field `ADPIF` writer - ADP interrupt flag
pub type ADPIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADP module enable
    #[inline(always)]
    pub fn adpmen(&self) -> ADPMEN_R {
        ADPMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 23 - ADP interrupt flag
    #[inline(always)]
    pub fn adpif(&self) -> ADPIF_R {
        ADPIF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPWRDN")
            .field("adpmen", &self.adpmen())
            .field("adpif", &self.adpif())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADP module enable
    #[inline(always)]
    pub fn adpmen(&mut self) -> ADPMEN_W<'_, GPWRDNrs> {
        ADPMEN_W::new(self, 0)
    }
    ///Bit 23 - ADP interrupt flag
    #[inline(always)]
    pub fn adpif(&mut self) -> ADPIF_W<'_, GPWRDNrs> {
        ADPIF_W::new(self, 23)
    }
}
/**OTG power down register

You can [`read`](crate::Reg::read) this register and get [`gpwrdn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpwrdn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#OTG_FS_GLOBAL:GPWRDN)*/
pub struct GPWRDNrs;
impl crate::RegisterSpec for GPWRDNrs {
    type Ux = u32;
}
///`read()` method returns [`gpwrdn::R`](R) reader structure
impl crate::Readable for GPWRDNrs {}
///`write(|w| ..)` method takes [`gpwrdn::W`](W) writer structure
impl crate::Writable for GPWRDNrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPWRDN to value 0x0200_0400
impl crate::Resettable for GPWRDNrs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
