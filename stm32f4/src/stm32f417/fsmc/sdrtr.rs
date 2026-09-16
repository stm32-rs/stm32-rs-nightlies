///Register `SDRTR` reader
pub type R = crate::R<SDRTRrs>;
///Register `SDRTR` writer
pub type W = crate::W<SDRTRrs>;
///Field `CRE` writer - Clear Refresh error flag
pub type CRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COUNT` reader - Refresh Timer Count
pub type COUNT_R = crate::FieldReader<u16>;
///Field `COUNT` writer - Refresh Timer Count
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `REIE` reader - RES Interrupt Enable
pub type REIE_R = crate::BitReader;
///Field `REIE` writer - RES Interrupt Enable
pub type REIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 1:13 - Refresh Timer Count
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    ///Bit 14 - RES Interrupt Enable
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDRTR")
            .field("count", &self.count())
            .field("reie", &self.reie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Refresh error flag
    #[inline(always)]
    pub fn cre(&mut self) -> CRE_W<'_, SDRTRrs> {
        CRE_W::new(self, 0)
    }
    ///Bits 1:13 - Refresh Timer Count
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W<'_, SDRTRrs> {
        COUNT_W::new(self, 1)
    }
    ///Bit 14 - RES Interrupt Enable
    #[inline(always)]
    pub fn reie(&mut self) -> REIE_W<'_, SDRTRrs> {
        REIE_W::new(self, 14)
    }
}
/**SDRAM Refresh Timer register

You can [`read`](crate::Reg::read) this register and get [`sdrtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdrtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#FSMC:SDRTR)*/
pub struct SDRTRrs;
impl crate::RegisterSpec for SDRTRrs {
    type Ux = u32;
}
///`read()` method returns [`sdrtr::R`](R) reader structure
impl crate::Readable for SDRTRrs {}
///`write(|w| ..)` method takes [`sdrtr::W`](W) writer structure
impl crate::Writable for SDRTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDRTR to value 0
impl crate::Resettable for SDRTRrs {}
