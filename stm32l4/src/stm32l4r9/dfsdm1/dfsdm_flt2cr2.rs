///Register `DFSDM_FLT2CR2` reader
pub type R = crate::R<DFSDM_FLT2CR2rs>;
///Register `DFSDM_FLT2CR2` writer
pub type W = crate::W<DFSDM_FLT2CR2rs>;
///Field `JEOCIE` reader - Injected end of conversion interrupt enable
pub type JEOCIE_R = crate::BitReader;
///Field `JEOCIE` writer - Injected end of conversion interrupt enable
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REOCIE` reader - Regular end of conversion interrupt enable
pub type REOCIE_R = crate::BitReader;
///Field `REOCIE` writer - Regular end of conversion interrupt enable
pub type REOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JOVRIE` reader - Injected data overrun interrupt enable
pub type JOVRIE_R = crate::BitReader;
///Field `JOVRIE` writer - Injected data overrun interrupt enable
pub type JOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROVRIE` reader - Regular data overrun interrupt enable
pub type ROVRIE_R = crate::BitReader;
///Field `ROVRIE` writer - Regular data overrun interrupt enable
pub type ROVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWDIE` reader - Analog watchdog interrupt enable
pub type AWDIE_R = crate::BitReader;
///Field `AWDIE` writer - Analog watchdog interrupt enable
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCDIE` reader - Short-circuit detector interrupt enable
pub type SCDIE_R = crate::BitReader;
///Field `SCDIE` writer - Short-circuit detector interrupt enable
pub type SCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABIE` reader - Clock absence interrupt enable
pub type CKABIE_R = crate::BitReader;
///Field `CKABIE` writer - Clock absence interrupt enable
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXCH` reader - Extremes detector channel selection
pub type EXCH_R = crate::FieldReader;
///Field `EXCH` writer - Extremes detector channel selection
pub type EXCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `AWDCH` reader - Analog watchdog channel selection
pub type AWDCH_R = crate::FieldReader;
///Field `AWDCH` writer - Analog watchdog channel selection
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Injected end of conversion interrupt enable
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Regular end of conversion interrupt enable
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected data overrun interrupt enable
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Regular data overrun interrupt enable
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Short-circuit detector interrupt enable
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clock absence interrupt enable
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:15 - Extremes detector channel selection
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Analog watchdog channel selection
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM_FLT2CR2")
            .field("awdch", &self.awdch())
            .field("exch", &self.exch())
            .field("ckabie", &self.ckabie())
            .field("scdie", &self.scdie())
            .field("awdie", &self.awdie())
            .field("rovrie", &self.rovrie())
            .field("jovrie", &self.jovrie())
            .field("reocie", &self.reocie())
            .field("jeocie", &self.jeocie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Injected end of conversion interrupt enable
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<DFSDM_FLT2CR2rs> {
        JEOCIE_W::new(self, 0)
    }
    ///Bit 1 - Regular end of conversion interrupt enable
    #[inline(always)]
    pub fn reocie(&mut self) -> REOCIE_W<DFSDM_FLT2CR2rs> {
        REOCIE_W::new(self, 1)
    }
    ///Bit 2 - Injected data overrun interrupt enable
    #[inline(always)]
    pub fn jovrie(&mut self) -> JOVRIE_W<DFSDM_FLT2CR2rs> {
        JOVRIE_W::new(self, 2)
    }
    ///Bit 3 - Regular data overrun interrupt enable
    #[inline(always)]
    pub fn rovrie(&mut self) -> ROVRIE_W<DFSDM_FLT2CR2rs> {
        ROVRIE_W::new(self, 3)
    }
    ///Bit 4 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W<DFSDM_FLT2CR2rs> {
        AWDIE_W::new(self, 4)
    }
    ///Bit 5 - Short-circuit detector interrupt enable
    #[inline(always)]
    pub fn scdie(&mut self) -> SCDIE_W<DFSDM_FLT2CR2rs> {
        SCDIE_W::new(self, 5)
    }
    ///Bit 6 - Clock absence interrupt enable
    #[inline(always)]
    pub fn ckabie(&mut self) -> CKABIE_W<DFSDM_FLT2CR2rs> {
        CKABIE_W::new(self, 6)
    }
    ///Bits 8:15 - Extremes detector channel selection
    #[inline(always)]
    pub fn exch(&mut self) -> EXCH_W<DFSDM_FLT2CR2rs> {
        EXCH_W::new(self, 8)
    }
    ///Bits 16:23 - Analog watchdog channel selection
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W<DFSDM_FLT2CR2rs> {
        AWDCH_W::new(self, 16)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2CR2)*/
pub struct DFSDM_FLT2CR2rs;
impl crate::RegisterSpec for DFSDM_FLT2CR2rs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_flt2cr2::R`](R) reader structure
impl crate::Readable for DFSDM_FLT2CR2rs {}
///`write(|w| ..)` method takes [`dfsdm_flt2cr2::W`](W) writer structure
impl crate::Writable for DFSDM_FLT2CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM_FLT2CR2 to value 0
impl crate::Resettable for DFSDM_FLT2CR2rs {
    const RESET_VALUE: u32 = 0;
}
