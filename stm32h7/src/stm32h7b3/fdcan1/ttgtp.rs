///Register `TTGTP` reader
pub type R = crate::R<TTGTPrs>;
///Register `TTGTP` writer
pub type W = crate::W<TTGTPrs>;
///Field `NCL` reader - Time Preset
pub type NCL_R = crate::FieldReader<u16>;
///Field `NCL` writer - Time Preset
pub type NCL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CTP` reader - Cycle Time Target Phase
pub type CTP_R = crate::FieldReader<u16>;
///Field `CTP` writer - Cycle Time Target Phase
pub type CTP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Time Preset
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Cycle Time Target Phase
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTGTP")
            .field("ncl", &self.ncl())
            .field("ctp", &self.ctp())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Time Preset
    #[inline(always)]
    pub fn ncl(&mut self) -> NCL_W<'_, TTGTPrs> {
        NCL_W::new(self, 0)
    }
    ///Bits 16:31 - Cycle Time Target Phase
    #[inline(always)]
    pub fn ctp(&mut self) -> CTP_W<'_, TTGTPrs> {
        CTP_W::new(self, 16)
    }
}
/**FDCAN TT Global Time Preset Register

You can [`read`](crate::Reg::read) this register and get [`ttgtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttgtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#FDCAN1:TTGTP)*/
pub struct TTGTPrs;
impl crate::RegisterSpec for TTGTPrs {
    type Ux = u32;
}
///`read()` method returns [`ttgtp::R`](R) reader structure
impl crate::Readable for TTGTPrs {}
///`write(|w| ..)` method takes [`ttgtp::W`](W) writer structure
impl crate::Writable for TTGTPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTGTP to value 0
impl crate::Resettable for TTGTPrs {}
