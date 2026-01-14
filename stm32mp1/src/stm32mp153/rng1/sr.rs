///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `DRDY` reader - DRDY
pub type DRDY_R = crate::BitReader;
///Field `CECS` reader - CECS
pub type CECS_R = crate::BitReader;
///Field `SECS` reader - SECS
pub type SECS_R = crate::BitReader;
///Field `CEIS` reader - CEIS
pub type CEIS_R = crate::BitReader;
///Field `CEIS` writer - CEIS
pub type CEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEIS` reader - SEIS
pub type SEIS_R = crate::BitReader;
///Field `SEIS` writer - SEIS
pub type SEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DRDY
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CECS
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SECS
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - CEIS
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SEIS
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("drdy", &self.drdy())
            .field("cecs", &self.cecs())
            .field("secs", &self.secs())
            .field("ceis", &self.ceis())
            .field("seis", &self.seis())
            .finish()
    }
}
impl W {
    ///Bit 5 - CEIS
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W<'_, SRrs> {
        CEIS_W::new(self, 5)
    }
    ///Bit 6 - SEIS
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W<'_, SRrs> {
        SEIS_W::new(self, 6)
    }
}
/**RNG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RNG1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
