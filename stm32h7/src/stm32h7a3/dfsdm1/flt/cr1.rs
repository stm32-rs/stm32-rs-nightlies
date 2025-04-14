///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `DFEN` reader - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state
pub type DFEN_R = crate::BitReader;
///Field `DFEN` writer - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state
pub type DFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JSWSTART` reader - Start a conversion of the injected group of channels This bit is always read as '0â.
pub type JSWSTART_R = crate::BitReader;
///Field `JSWSTART` writer - Start a conversion of the injected group of channels This bit is always read as '0â.
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JSYNC` reader - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JSYNC_R = crate::BitReader;
///Field `JSYNC` writer - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JSCAN` reader - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.
pub type JSCAN_R = crate::BitReader;
///Field `JSCAN` writer - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.
pub type JSCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JDMAEN_R = crate::BitReader;
///Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).
pub type JEXTSEL_R = crate::FieldReader;
///Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JEXTEN_R = crate::FieldReader;
///Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSWSTART` reader - Software start of a conversion on the regular channel This bit is always read as '0â.
pub type RSWSTART_R = crate::BitReader;
///Field `RSWSTART` writer - Software start of a conversion on the regular channel This bit is always read as '0â.
pub type RSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCONT` reader - Continuous mode selection for regular conversions Writing '0â to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.
pub type RCONT_R = crate::BitReader;
///Field `RCONT` writer - Continuous mode selection for regular conversions Writing '0â to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.
pub type RCONT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSYNC` reader - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type RSYNC_R = crate::BitReader;
///Field `RSYNC` writer - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type RSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDMAEN` reader - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type RDMAEN_R = crate::BitReader;
///Field `RDMAEN` writer - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type RDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCH` reader - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).
pub type RCH_R = crate::FieldReader;
///Field `RCH` writer - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).
pub type RCH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FAST` reader - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.
pub type FAST_R = crate::BitReader;
///Field `FAST` writer - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.
pub type FAST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWFSEL` reader - Analog watchdog fast mode select
pub type AWFSEL_R = crate::BitReader;
///Field `AWFSEL` writer - Analog watchdog fast mode select
pub type AWFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start a conversion of the injected group of channels This bit is always read as '0â.
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:12 - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 13:14 - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 17 - Software start of a conversion on the regular channel This bit is always read as '0â.
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Continuous mode selection for regular conversions Writing '0â to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:26 - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 29 - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Analog watchdog fast mode select
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("dfen", &self.dfen())
            .field("jswstart", &self.jswstart())
            .field("jsync", &self.jsync())
            .field("jscan", &self.jscan())
            .field("jdmaen", &self.jdmaen())
            .field("jextsel", &self.jextsel())
            .field("jexten", &self.jexten())
            .field("rswstart", &self.rswstart())
            .field("rcont", &self.rcont())
            .field("rsync", &self.rsync())
            .field("rdmaen", &self.rdmaen())
            .field("rch", &self.rch())
            .field("fast", &self.fast())
            .field("awfsel", &self.awfsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state
    #[inline(always)]
    pub fn dfen(&mut self) -> DFEN_W<CR1rs> {
        DFEN_W::new(self, 0)
    }
    ///Bit 1 - Start a conversion of the injected group of channels This bit is always read as '0â.
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W<CR1rs> {
        JSWSTART_W::new(self, 1)
    }
    ///Bit 3 - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jsync(&mut self) -> JSYNC_W<CR1rs> {
        JSYNC_W::new(self, 3)
    }
    ///Bit 4 - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.
    #[inline(always)]
    pub fn jscan(&mut self) -> JSCAN_W<CR1rs> {
        JSCAN_W::new(self, 4)
    }
    ///Bit 5 - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jdmaen(&mut self) -> JDMAEN_W<CR1rs> {
        JDMAEN_W::new(self, 5)
    }
    ///Bits 8:12 - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<CR1rs> {
        JEXTSEL_W::new(self, 8)
    }
    ///Bits 13:14 - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<CR1rs> {
        JEXTEN_W::new(self, 13)
    }
    ///Bit 17 - Software start of a conversion on the regular channel This bit is always read as '0â.
    #[inline(always)]
    pub fn rswstart(&mut self) -> RSWSTART_W<CR1rs> {
        RSWSTART_W::new(self, 17)
    }
    ///Bit 18 - Continuous mode selection for regular conversions Writing '0â to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.
    #[inline(always)]
    pub fn rcont(&mut self) -> RCONT_W<CR1rs> {
        RCONT_W::new(self, 18)
    }
    ///Bit 19 - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn rsync(&mut self) -> RSYNC_W<CR1rs> {
        RSYNC_W::new(self, 19)
    }
    ///Bit 21 - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RDMAEN_W<CR1rs> {
        RDMAEN_W::new(self, 21)
    }
    ///Bits 24:26 - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).
    #[inline(always)]
    pub fn rch(&mut self) -> RCH_W<CR1rs> {
        RCH_W::new(self, 24)
    }
    ///Bit 29 - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W<CR1rs> {
        FAST_W::new(self, 29)
    }
    ///Bit 30 - Analog watchdog fast mode select
    #[inline(always)]
    pub fn awfsel(&mut self) -> AWFSEL_W<CR1rs> {
        AWFSEL_W::new(self, 30)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
