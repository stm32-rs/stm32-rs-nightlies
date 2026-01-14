///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `DRDY` reader - Data ready
pub type DRDY_R = crate::BitReader;
///Field `CECS` reader - Clock error current status
pub type CECS_R = crate::BitReader;
///Field `SECS` reader - Seed error current status
pub type SECS_R = crate::BitReader;
///Field `CEIS` reader - Clock error interrupt status
pub type CEIS_R = crate::BitReader;
///Field `CEIS` writer - Clock error interrupt status
pub type CEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEIS` reader - Seed error interrupt status
pub type SEIS_R = crate::BitReader;
///Field `SEIS` writer - Seed error interrupt status
pub type SEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Data ready
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock error current status
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Seed error current status
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Clock error interrupt status
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Seed error interrupt status
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("seis", &self.seis())
            .field("ceis", &self.ceis())
            .field("secs", &self.secs())
            .field("cecs", &self.cecs())
            .field("drdy", &self.drdy())
            .finish()
    }
}
impl W {
    ///Bit 5 - Clock error interrupt status
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W<'_, SRrs> {
        CEIS_W::new(self, 5)
    }
    ///Bit 6 - Seed error interrupt status
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W<'_, SRrs> {
        SEIS_W::new(self, 6)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RNG:SR)*/
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
