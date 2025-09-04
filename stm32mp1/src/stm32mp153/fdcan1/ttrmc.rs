///Register `TTRMC` reader
pub type R = crate::R<TTRMCrs>;
///Register `TTRMC` writer
pub type W = crate::W<TTRMCrs>;
///Field `RID` reader - RID
pub type RID_R = crate::FieldReader<u32>;
///Field `RID` writer - RID
pub type RID_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
///Field `XTD` reader - XTD
pub type XTD_R = crate::BitReader;
///Field `XTD` writer - XTD
pub type XTD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMPS` reader - RMPS
pub type RMPS_R = crate::BitReader;
///Field `RMPS` writer - RMPS
pub type RMPS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:28 - RID
    #[inline(always)]
    pub fn rid(&self) -> RID_R {
        RID_R::new(self.bits & 0x1fff_ffff)
    }
    ///Bit 30 - XTD
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RMPS
    #[inline(always)]
    pub fn rmps(&self) -> RMPS_R {
        RMPS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTRMC")
            .field("rid", &self.rid())
            .field("xtd", &self.xtd())
            .field("rmps", &self.rmps())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - RID
    #[inline(always)]
    pub fn rid(&mut self) -> RID_W<TTRMCrs> {
        RID_W::new(self, 0)
    }
    ///Bit 30 - XTD
    #[inline(always)]
    pub fn xtd(&mut self) -> XTD_W<TTRMCrs> {
        XTD_W::new(self, 30)
    }
    ///Bit 31 - RMPS
    #[inline(always)]
    pub fn rmps(&mut self) -> RMPS_W<TTRMCrs> {
        RMPS_W::new(self, 31)
    }
}
/**FDCAN TT reference message configuration register

You can [`read`](crate::Reg::read) this register and get [`ttrmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttrmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:TTRMC)*/
pub struct TTRMCrs;
impl crate::RegisterSpec for TTRMCrs {
    type Ux = u32;
}
///`read()` method returns [`ttrmc::R`](R) reader structure
impl crate::Readable for TTRMCrs {}
///`write(|w| ..)` method takes [`ttrmc::W`](W) writer structure
impl crate::Writable for TTRMCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTRMC to value 0
impl crate::Resettable for TTRMCrs {}
