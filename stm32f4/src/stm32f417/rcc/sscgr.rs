///Register `SSCGR` reader
pub type R = crate::R<SSCGRrs>;
///Register `SSCGR` writer
pub type W = crate::W<SSCGRrs>;
///Field `MODPER` reader - Modulation period
pub type MODPER_R = crate::FieldReader<u16>;
///Field `MODPER` writer - Modulation period
pub type MODPER_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `INCSTEP` reader - Incrementation step
pub type INCSTEP_R = crate::FieldReader<u16>;
///Field `INCSTEP` writer - Incrementation step
pub type INCSTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `SPREADSEL` reader - Spread Select
pub type SPREADSEL_R = crate::BitReader;
///Field `SPREADSEL` writer - Spread Select
pub type SPREADSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSCGEN` reader - Spread spectrum modulation enable
pub type SSCGEN_R = crate::BitReader;
///Field `SSCGEN` writer - Spread spectrum modulation enable
pub type SSCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:12 - Modulation period
    #[inline(always)]
    pub fn modper(&self) -> MODPER_R {
        MODPER_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:27 - Incrementation step
    #[inline(always)]
    pub fn incstep(&self) -> INCSTEP_R {
        INCSTEP_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    ///Bit 30 - Spread Select
    #[inline(always)]
    pub fn spreadsel(&self) -> SPREADSEL_R {
        SPREADSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Spread spectrum modulation enable
    #[inline(always)]
    pub fn sscgen(&self) -> SSCGEN_R {
        SSCGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSCGR")
            .field("sscgen", &self.sscgen())
            .field("spreadsel", &self.spreadsel())
            .field("incstep", &self.incstep())
            .field("modper", &self.modper())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Modulation period
    #[inline(always)]
    pub fn modper(&mut self) -> MODPER_W<'_, SSCGRrs> {
        MODPER_W::new(self, 0)
    }
    ///Bits 13:27 - Incrementation step
    #[inline(always)]
    pub fn incstep(&mut self) -> INCSTEP_W<'_, SSCGRrs> {
        INCSTEP_W::new(self, 13)
    }
    ///Bit 30 - Spread Select
    #[inline(always)]
    pub fn spreadsel(&mut self) -> SPREADSEL_W<'_, SSCGRrs> {
        SPREADSEL_W::new(self, 30)
    }
    ///Bit 31 - Spread spectrum modulation enable
    #[inline(always)]
    pub fn sscgen(&mut self) -> SSCGEN_W<'_, SSCGRrs> {
        SSCGEN_W::new(self, 31)
    }
}
/**spread spectrum clock generation register

You can [`read`](crate::Reg::read) this register and get [`sscgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sscgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RCC:SSCGR)*/
pub struct SSCGRrs;
impl crate::RegisterSpec for SSCGRrs {
    type Ux = u32;
}
///`read()` method returns [`sscgr::R`](R) reader structure
impl crate::Readable for SSCGRrs {}
///`write(|w| ..)` method takes [`sscgr::W`](W) writer structure
impl crate::Writable for SSCGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSCGR to value 0
impl crate::Resettable for SSCGRrs {}
