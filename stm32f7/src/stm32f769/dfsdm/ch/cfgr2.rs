///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `DTRBS` reader - Data right bit-shift for channel 0
pub type DTRBS_R = crate::FieldReader;
///Field `DTRBS` writer - Data right bit-shift for channel 0
pub type DTRBS_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `OFFSET` reader - 24-bit calibration offset for channel 0
pub type OFFSET_R = crate::FieldReader<u32>;
///Field `OFFSET` writer - 24-bit calibration offset for channel 0
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32, crate::Safe>;
impl R {
    ///Bits 3:7 - Data right bit-shift for channel 0
    #[inline(always)]
    pub fn dtrbs(&self) -> DTRBS_R {
        DTRBS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bits 8:31 - 24-bit calibration offset for channel 0
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("dtrbs", &self.dtrbs())
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 3:7 - Data right bit-shift for channel 0
    #[inline(always)]
    pub fn dtrbs(&mut self) -> DTRBS_W<'_, CFGR2rs> {
        DTRBS_W::new(self, 3)
    }
    ///Bits 8:31 - 24-bit calibration offset for channel 0
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<'_, CFGR2rs> {
        OFFSET_W::new(self, 8)
    }
}
/**DFSDM channel configuration 0 register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
