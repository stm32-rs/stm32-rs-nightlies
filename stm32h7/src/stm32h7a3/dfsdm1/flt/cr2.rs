///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `JEOCIE` reader - Injected end of conversion interrupt enable Please see the explanation of JEOCF in DFSDM_FLTxISR.
pub type JEOCIE_R = crate::BitReader;
///Field `JEOCIE` writer - Injected end of conversion interrupt enable Please see the explanation of JEOCF in DFSDM_FLTxISR.
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REOCIE` reader - Regular end of conversion interrupt enable Please see the explanation of REOCF in DFSDM_FLTxISR.
pub type REOCIE_R = crate::BitReader;
///Field `REOCIE` writer - Regular end of conversion interrupt enable Please see the explanation of REOCF in DFSDM_FLTxISR.
pub type REOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JOVRIE` reader - Injected data overrun interrupt enable Please see the explanation of JOVRF in DFSDM_FLTxISR.
pub type JOVRIE_R = crate::BitReader;
///Field `JOVRIE` writer - Injected data overrun interrupt enable Please see the explanation of JOVRF in DFSDM_FLTxISR.
pub type JOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROVRIE` reader - Regular data overrun interrupt enable Please see the explanation of ROVRF in DFSDM_FLTxISR.
pub type ROVRIE_R = crate::BitReader;
///Field `ROVRIE` writer - Regular data overrun interrupt enable Please see the explanation of ROVRF in DFSDM_FLTxISR.
pub type ROVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWDIE` reader - Analog watchdog interrupt enable Please see the explanation of AWDF in DFSDM_FLTxISR.
pub type AWDIE_R = crate::BitReader;
///Field `AWDIE` writer - Analog watchdog interrupt enable Please see the explanation of AWDF in DFSDM_FLTxISR.
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCDIE` reader - Short-circuit detector interrupt enable Please see the explanation of SCDF\[7:0\] in DFSDM_FLTxISR. Note: SCDIE is present only in DFSDM_FLT0CR2 register (filter x=0)
pub type SCDIE_R = crate::BitReader;
///Field `SCDIE` writer - Short-circuit detector interrupt enable Please see the explanation of SCDF\[7:0\] in DFSDM_FLTxISR. Note: SCDIE is present only in DFSDM_FLT0CR2 register (filter x=0)
pub type SCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABIE` reader - Clock absence interrupt enable Please see the explanation of CKABF\[7:0\] in DFSDM_FLTxISR. Note: CKABIE is present only in DFSDM_FLT0CR2 register (filter x=0)
pub type CKABIE_R = crate::BitReader;
///Field `CKABIE` writer - Clock absence interrupt enable Please see the explanation of CKABF\[7:0\] in DFSDM_FLTxISR. Note: CKABIE is present only in DFSDM_FLT0CR2 register (filter x=0)
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXCH` reader - Extremes detector channel selection These bits select the input channels to be taken by the Extremes detector. EXCH\[y\] = 0: Extremes detector does not accept data from channel y EXCH\[y\] = 1: Extremes detector accepts data from channel y
pub type EXCH_R = crate::FieldReader;
///Field `EXCH` writer - Extremes detector channel selection These bits select the input channels to be taken by the Extremes detector. EXCH\[y\] = 0: Extremes detector does not accept data from channel y EXCH\[y\] = 1: Extremes detector accepts data from channel y
pub type EXCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `AWDCH` reader - Analog watchdog channel selection These bits select the input channel to be guarded continuously by the analog watchdog. AWDCH\[y\] = 0: Analog watchdog is disabled on channel y AWDCH\[y\] = 1: Analog watchdog is enabled on channel y
pub type AWDCH_R = crate::FieldReader;
///Field `AWDCH` writer - Analog watchdog channel selection These bits select the input channel to be guarded continuously by the analog watchdog. AWDCH\[y\] = 0: Analog watchdog is disabled on channel y AWDCH\[y\] = 1: Analog watchdog is enabled on channel y
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Injected end of conversion interrupt enable Please see the explanation of JEOCF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Regular end of conversion interrupt enable Please see the explanation of REOCF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected data overrun interrupt enable Please see the explanation of JOVRF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Regular data overrun interrupt enable Please see the explanation of ROVRF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog interrupt enable Please see the explanation of AWDF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Short-circuit detector interrupt enable Please see the explanation of SCDF\[7:0\] in DFSDM_FLTxISR. Note: SCDIE is present only in DFSDM_FLT0CR2 register (filter x=0)
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clock absence interrupt enable Please see the explanation of CKABF\[7:0\] in DFSDM_FLTxISR. Note: CKABIE is present only in DFSDM_FLT0CR2 register (filter x=0)
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:15 - Extremes detector channel selection These bits select the input channels to be taken by the Extremes detector. EXCH\[y\] = 0: Extremes detector does not accept data from channel y EXCH\[y\] = 1: Extremes detector accepts data from channel y
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Analog watchdog channel selection These bits select the input channel to be guarded continuously by the analog watchdog. AWDCH\[y\] = 0: Analog watchdog is disabled on channel y AWDCH\[y\] = 1: Analog watchdog is enabled on channel y
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("jeocie", &self.jeocie())
            .field("reocie", &self.reocie())
            .field("jovrie", &self.jovrie())
            .field("rovrie", &self.rovrie())
            .field("awdie", &self.awdie())
            .field("scdie", &self.scdie())
            .field("ckabie", &self.ckabie())
            .field("exch", &self.exch())
            .field("awdch", &self.awdch())
            .finish()
    }
}
impl W {
    ///Bit 0 - Injected end of conversion interrupt enable Please see the explanation of JEOCF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<CR2rs> {
        JEOCIE_W::new(self, 0)
    }
    ///Bit 1 - Regular end of conversion interrupt enable Please see the explanation of REOCF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn reocie(&mut self) -> REOCIE_W<CR2rs> {
        REOCIE_W::new(self, 1)
    }
    ///Bit 2 - Injected data overrun interrupt enable Please see the explanation of JOVRF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn jovrie(&mut self) -> JOVRIE_W<CR2rs> {
        JOVRIE_W::new(self, 2)
    }
    ///Bit 3 - Regular data overrun interrupt enable Please see the explanation of ROVRF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn rovrie(&mut self) -> ROVRIE_W<CR2rs> {
        ROVRIE_W::new(self, 3)
    }
    ///Bit 4 - Analog watchdog interrupt enable Please see the explanation of AWDF in DFSDM_FLTxISR.
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W<CR2rs> {
        AWDIE_W::new(self, 4)
    }
    ///Bit 5 - Short-circuit detector interrupt enable Please see the explanation of SCDF\[7:0\] in DFSDM_FLTxISR. Note: SCDIE is present only in DFSDM_FLT0CR2 register (filter x=0)
    #[inline(always)]
    pub fn scdie(&mut self) -> SCDIE_W<CR2rs> {
        SCDIE_W::new(self, 5)
    }
    ///Bit 6 - Clock absence interrupt enable Please see the explanation of CKABF\[7:0\] in DFSDM_FLTxISR. Note: CKABIE is present only in DFSDM_FLT0CR2 register (filter x=0)
    #[inline(always)]
    pub fn ckabie(&mut self) -> CKABIE_W<CR2rs> {
        CKABIE_W::new(self, 6)
    }
    ///Bits 8:15 - Extremes detector channel selection These bits select the input channels to be taken by the Extremes detector. EXCH\[y\] = 0: Extremes detector does not accept data from channel y EXCH\[y\] = 1: Extremes detector accepts data from channel y
    #[inline(always)]
    pub fn exch(&mut self) -> EXCH_W<CR2rs> {
        EXCH_W::new(self, 8)
    }
    ///Bits 16:23 - Analog watchdog channel selection These bits select the input channel to be guarded continuously by the analog watchdog. AWDCH\[y\] = 0: Analog watchdog is disabled on channel y AWDCH\[y\] = 1: Analog watchdog is enabled on channel y
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W<CR2rs> {
        AWDCH_W::new(self, 16)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
