///Register `CSICFGR` reader
pub type R = crate::R<CSICFGRrs>;
///Register `CSICFGR` writer
pub type W = crate::W<CSICFGRrs>;
///Field `CSICAL` reader - CSI clock calibration
pub type CSICAL_R = crate::FieldReader<u16>;
///Field `CSICAL` writer - CSI clock calibration
pub type CSICAL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CSITRIM` reader - CSI clock trimming
pub type CSITRIM_R = crate::FieldReader;
///Field `CSITRIM` writer - CSI clock trimming
pub type CSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    ///Bits 0:9 - CSI clock calibration
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 24:29 - CSI clock trimming
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSICFGR")
            .field("csitrim", &self.csitrim())
            .field("csical", &self.csical())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - CSI clock calibration
    #[inline(always)]
    pub fn csical(&mut self) -> CSICAL_W<'_, CSICFGRrs> {
        CSICAL_W::new(self, 0)
    }
    ///Bits 24:29 - CSI clock trimming
    #[inline(always)]
    pub fn csitrim(&mut self) -> CSITRIM_W<'_, CSICFGRrs> {
        CSITRIM_W::new(self, 24)
    }
}
/**RCC CSI configuration register

You can [`read`](crate::Reg::read) this register and get [`csicfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csicfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#RCC:CSICFGR)*/
pub struct CSICFGRrs;
impl crate::RegisterSpec for CSICFGRrs {
    type Ux = u32;
}
///`read()` method returns [`csicfgr::R`](R) reader structure
impl crate::Readable for CSICFGRrs {}
///`write(|w| ..)` method takes [`csicfgr::W`](W) writer structure
impl crate::Writable for CSICFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSICFGR to value 0
impl crate::Resettable for CSICFGRrs {}
