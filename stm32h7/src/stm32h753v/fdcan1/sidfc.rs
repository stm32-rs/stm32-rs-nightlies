///Register `SIDFC` reader
pub type R = crate::R<SIDFCrs>;
///Register `SIDFC` writer
pub type W = crate::W<SIDFCrs>;
///Field `FLSSA` reader - Filter List Standard Start Address
pub type FLSSA_R = crate::FieldReader<u16>;
///Field `FLSSA` writer - Filter List Standard Start Address
pub type FLSSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `LSS` reader - List Size Standard
pub type LSS_R = crate::FieldReader;
///Field `LSS` writer - List Size Standard
pub type LSS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 2:15 - Filter List Standard Start Address
    #[inline(always)]
    pub fn flssa(&self) -> FLSSA_R {
        FLSSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:23 - List Size Standard
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIDFC")
            .field("flssa", &self.flssa())
            .field("lss", &self.lss())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Filter List Standard Start Address
    #[inline(always)]
    pub fn flssa(&mut self) -> FLSSA_W<'_, SIDFCrs> {
        FLSSA_W::new(self, 2)
    }
    ///Bits 16:23 - List Size Standard
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W<'_, SIDFCrs> {
        LSS_W::new(self, 16)
    }
}
/**FDCAN Standard ID Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`sidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#FDCAN1:SIDFC)*/
pub struct SIDFCrs;
impl crate::RegisterSpec for SIDFCrs {
    type Ux = u32;
}
///`read()` method returns [`sidfc::R`](R) reader structure
impl crate::Readable for SIDFCrs {}
///`write(|w| ..)` method takes [`sidfc::W`](W) writer structure
impl crate::Writable for SIDFCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SIDFC to value 0
impl crate::Resettable for SIDFCrs {}
