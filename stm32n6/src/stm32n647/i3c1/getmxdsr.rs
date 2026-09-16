///Register `GETMXDSR` reader
pub type R = crate::R<GETMXDSRrs>;
///Register `GETMXDSR` writer
pub type W = crate::W<GETMXDSRrs>;
///Field `HOFFAS` reader - Controller hand-off activity state
pub type HOFFAS_R = crate::FieldReader;
///Field `HOFFAS` writer - Controller hand-off activity state
pub type HOFFAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FMT` reader - GETMXDS CCC format
pub type FMT_R = crate::FieldReader;
///Field `FMT` writer - GETMXDS CCC format
pub type FMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RDTURN` reader - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte)
pub type RDTURN_R = crate::FieldReader;
///Field `RDTURN` writer - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte)
pub type RDTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TSCO` reader - clock-to-data turnaround time (tless thansub>SCOless than/sub>)
pub type TSCO_R = crate::BitReader;
///Field `TSCO` writer - clock-to-data turnaround time (tless thansub>SCOless than/sub>)
pub type TSCO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Controller hand-off activity state
    #[inline(always)]
    pub fn hoffas(&self) -> HOFFAS_R {
        HOFFAS_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - GETMXDS CCC format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:23 - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte)
    #[inline(always)]
    pub fn rdturn(&self) -> RDTURN_R {
        RDTURN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - clock-to-data turnaround time (tless thansub>SCOless than/sub>)
    #[inline(always)]
    pub fn tsco(&self) -> TSCO_R {
        TSCO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GETMXDSR")
            .field("hoffas", &self.hoffas())
            .field("fmt", &self.fmt())
            .field("rdturn", &self.rdturn())
            .field("tsco", &self.tsco())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Controller hand-off activity state
    #[inline(always)]
    pub fn hoffas(&mut self) -> HOFFAS_W<'_, GETMXDSRrs> {
        HOFFAS_W::new(self, 0)
    }
    ///Bits 8:9 - GETMXDS CCC format
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W<'_, GETMXDSRrs> {
        FMT_W::new(self, 8)
    }
    ///Bits 16:23 - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte)
    #[inline(always)]
    pub fn rdturn(&mut self) -> RDTURN_W<'_, GETMXDSRrs> {
        RDTURN_W::new(self, 16)
    }
    ///Bit 24 - clock-to-data turnaround time (tless thansub>SCOless than/sub>)
    #[inline(always)]
    pub fn tsco(&mut self) -> TSCO_W<'_, GETMXDSRrs> {
        TSCO_W::new(self, 24)
    }
}
/**I3C get capability register

You can [`read`](crate::Reg::read) this register and get [`getmxdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getmxdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#I3C1:GETMXDSR)*/
pub struct GETMXDSRrs;
impl crate::RegisterSpec for GETMXDSRrs {
    type Ux = u32;
}
///`read()` method returns [`getmxdsr::R`](R) reader structure
impl crate::Readable for GETMXDSRrs {}
///`write(|w| ..)` method takes [`getmxdsr::W`](W) writer structure
impl crate::Writable for GETMXDSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GETMXDSR to value 0
impl crate::Resettable for GETMXDSRrs {}
