///Register `CH1CFGR2` reader
pub type R = crate::R<CH1CFGR2rs>;
///Register `CH1CFGR2` writer
pub type W = crate::W<CH1CFGR2rs>;
///Field `DTRBS` reader - DTRBS
pub type DTRBS_R = crate::FieldReader;
///Field `DTRBS` writer - DTRBS
pub type DTRBS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `OFFSET` reader - OFFSET
pub type OFFSET_R = crate::FieldReader<u32>;
///Field `OFFSET` writer - OFFSET
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 3:7 - DTRBS
    #[inline(always)]
    pub fn dtrbs(&self) -> DTRBS_R {
        DTRBS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bits 8:31 - OFFSET
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1CFGR2")
            .field("dtrbs", &self.dtrbs())
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 3:7 - DTRBS
    #[inline(always)]
    pub fn dtrbs(&mut self) -> DTRBS_W<'_, CH1CFGR2rs> {
        DTRBS_W::new(self, 3)
    }
    ///Bits 8:31 - OFFSET
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<'_, CH1CFGR2rs> {
        OFFSET_W::new(self, 8)
    }
}
/**This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch1cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:CH1CFGR2)*/
pub struct CH1CFGR2rs;
impl crate::RegisterSpec for CH1CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`ch1cfgr2::R`](R) reader structure
impl crate::Readable for CH1CFGR2rs {}
///`write(|w| ..)` method takes [`ch1cfgr2::W`](W) writer structure
impl crate::Writable for CH1CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH1CFGR2 to value 0
impl crate::Resettable for CH1CFGR2rs {}
