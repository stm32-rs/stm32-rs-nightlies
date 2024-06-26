///Register `OUTER` reader
pub type R = crate::R<OUTERrs>;
///Register `OUTER` writer
pub type W = crate::W<OUTERrs>;
///Field `POL1` reader - Output 1 polarity
pub type POL1_R = crate::BitReader;
///Field `POL1` writer - Output 1 polarity
pub type POL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLEM1` reader - Output 1 Idle mode
pub type IDLEM1_R = crate::BitReader;
///Field `IDLEM1` writer - Output 1 Idle mode
pub type IDLEM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLES1` reader - Output 1 Idle State
pub type IDLES1_R = crate::BitReader;
///Field `IDLES1` writer - Output 1 Idle State
pub type IDLES1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAULT1` reader - Output 1 Fault state
pub type FAULT1_R = crate::FieldReader;
///Field `FAULT1` writer - Output 1 Fault state
pub type FAULT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHP1` reader - Output 1 Chopper enable
pub type CHP1_R = crate::BitReader;
///Field `CHP1` writer - Output 1 Chopper enable
pub type CHP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIDL1` reader - Output 1 Deadtime upon burst mode Idle entry
pub type DIDL1_R = crate::BitReader;
///Field `DIDL1` writer - Output 1 Deadtime upon burst mode Idle entry
pub type DIDL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTEN` reader - Deadtime enable
pub type DTEN_R = crate::BitReader;
///Field `DTEN` writer - Deadtime enable
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLYPRTEN` reader - Delayed Protection Enable
pub type DLYPRTEN_R = crate::BitReader;
///Field `DLYPRTEN` writer - Delayed Protection Enable
pub type DLYPRTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLYPRT` reader - Delayed Protection
pub type DLYPRT_R = crate::FieldReader;
///Field `DLYPRT` writer - Delayed Protection
pub type DLYPRT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BIAR` reader - Balanced Idle Automatic Resume
pub type BIAR_R = crate::BitReader;
///Field `BIAR` writer - Balanced Idle Automatic Resume
pub type BIAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POL2` reader - Output 2 polarity
pub type POL2_R = crate::BitReader;
///Field `POL2` writer - Output 2 polarity
pub type POL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLEM2` reader - Output 2 Idle mode
pub type IDLEM2_R = crate::BitReader;
///Field `IDLEM2` writer - Output 2 Idle mode
pub type IDLEM2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLES2` reader - Output 2 Idle State
pub type IDLES2_R = crate::BitReader;
///Field `IDLES2` writer - Output 2 Idle State
pub type IDLES2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAULT2` reader - Output 2 Fault state
pub type FAULT2_R = crate::FieldReader;
///Field `FAULT2` writer - Output 2 Fault state
pub type FAULT2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHP2` reader - Output 2 Chopper enable
pub type CHP2_R = crate::BitReader;
///Field `CHP2` writer - Output 2 Chopper enable
pub type CHP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIDL2` reader - Output 2 Deadtime upon burst mode Idle entry
pub type DIDL2_R = crate::BitReader;
///Field `DIDL2` writer - Output 2 Deadtime upon burst mode Idle entry
pub type DIDL2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Output 1 polarity
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Output 1 Idle mode
    #[inline(always)]
    pub fn idlem1(&self) -> IDLEM1_R {
        IDLEM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output 1 Idle State
    #[inline(always)]
    pub fn idles1(&self) -> IDLES1_R {
        IDLES1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Output 1 Fault state
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Output 1 Chopper enable
    #[inline(always)]
    pub fn chp1(&self) -> CHP1_R {
        CHP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Output 1 Deadtime upon burst mode Idle entry
    #[inline(always)]
    pub fn didl1(&self) -> DIDL1_R {
        DIDL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Deadtime enable
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Delayed Protection Enable
    #[inline(always)]
    pub fn dlyprten(&self) -> DLYPRTEN_R {
        DLYPRTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - Delayed Protection
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 14 - Balanced Idle Automatic Resume
    #[inline(always)]
    pub fn biar(&self) -> BIAR_R {
        BIAR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - Output 2 polarity
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Output 2 Idle mode
    #[inline(always)]
    pub fn idlem2(&self) -> IDLEM2_R {
        IDLEM2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Output 2 Idle State
    #[inline(always)]
    pub fn idles2(&self) -> IDLES2_R {
        IDLES2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Output 2 Fault state
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Output 2 Chopper enable
    #[inline(always)]
    pub fn chp2(&self) -> CHP2_R {
        CHP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Output 2 Deadtime upon burst mode Idle entry
    #[inline(always)]
    pub fn didl2(&self) -> DIDL2_R {
        DIDL2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTER")
            .field("didl2", &self.didl2())
            .field("chp2", &self.chp2())
            .field("fault2", &self.fault2())
            .field("idles2", &self.idles2())
            .field("idlem2", &self.idlem2())
            .field("pol2", &self.pol2())
            .field("biar", &self.biar())
            .field("dlyprt", &self.dlyprt())
            .field("dlyprten", &self.dlyprten())
            .field("dten", &self.dten())
            .field("didl1", &self.didl1())
            .field("chp1", &self.chp1())
            .field("fault1", &self.fault1())
            .field("idles1", &self.idles1())
            .field("idlem1", &self.idlem1())
            .field("pol1", &self.pol1())
            .finish()
    }
}
impl W {
    ///Bit 1 - Output 1 polarity
    #[inline(always)]
    #[must_use]
    pub fn pol1(&mut self) -> POL1_W<OUTERrs> {
        POL1_W::new(self, 1)
    }
    ///Bit 2 - Output 1 Idle mode
    #[inline(always)]
    #[must_use]
    pub fn idlem1(&mut self) -> IDLEM1_W<OUTERrs> {
        IDLEM1_W::new(self, 2)
    }
    ///Bit 3 - Output 1 Idle State
    #[inline(always)]
    #[must_use]
    pub fn idles1(&mut self) -> IDLES1_W<OUTERrs> {
        IDLES1_W::new(self, 3)
    }
    ///Bits 4:5 - Output 1 Fault state
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> FAULT1_W<OUTERrs> {
        FAULT1_W::new(self, 4)
    }
    ///Bit 6 - Output 1 Chopper enable
    #[inline(always)]
    #[must_use]
    pub fn chp1(&mut self) -> CHP1_W<OUTERrs> {
        CHP1_W::new(self, 6)
    }
    ///Bit 7 - Output 1 Deadtime upon burst mode Idle entry
    #[inline(always)]
    #[must_use]
    pub fn didl1(&mut self) -> DIDL1_W<OUTERrs> {
        DIDL1_W::new(self, 7)
    }
    ///Bit 8 - Deadtime enable
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<OUTERrs> {
        DTEN_W::new(self, 8)
    }
    ///Bit 9 - Delayed Protection Enable
    #[inline(always)]
    #[must_use]
    pub fn dlyprten(&mut self) -> DLYPRTEN_W<OUTERrs> {
        DLYPRTEN_W::new(self, 9)
    }
    ///Bits 10:12 - Delayed Protection
    #[inline(always)]
    #[must_use]
    pub fn dlyprt(&mut self) -> DLYPRT_W<OUTERrs> {
        DLYPRT_W::new(self, 10)
    }
    ///Bit 14 - Balanced Idle Automatic Resume
    #[inline(always)]
    #[must_use]
    pub fn biar(&mut self) -> BIAR_W<OUTERrs> {
        BIAR_W::new(self, 14)
    }
    ///Bit 17 - Output 2 polarity
    #[inline(always)]
    #[must_use]
    pub fn pol2(&mut self) -> POL2_W<OUTERrs> {
        POL2_W::new(self, 17)
    }
    ///Bit 18 - Output 2 Idle mode
    #[inline(always)]
    #[must_use]
    pub fn idlem2(&mut self) -> IDLEM2_W<OUTERrs> {
        IDLEM2_W::new(self, 18)
    }
    ///Bit 19 - Output 2 Idle State
    #[inline(always)]
    #[must_use]
    pub fn idles2(&mut self) -> IDLES2_W<OUTERrs> {
        IDLES2_W::new(self, 19)
    }
    ///Bits 20:21 - Output 2 Fault state
    #[inline(always)]
    #[must_use]
    pub fn fault2(&mut self) -> FAULT2_W<OUTERrs> {
        FAULT2_W::new(self, 20)
    }
    ///Bit 22 - Output 2 Chopper enable
    #[inline(always)]
    #[must_use]
    pub fn chp2(&mut self) -> CHP2_W<OUTERrs> {
        CHP2_W::new(self, 22)
    }
    ///Bit 23 - Output 2 Deadtime upon burst mode Idle entry
    #[inline(always)]
    #[must_use]
    pub fn didl2(&mut self) -> DIDL2_W<OUTERrs> {
        DIDL2_W::new(self, 23)
    }
}
/**Timerx Output Register

You can [`read`](crate::Reg::read) this register and get [`outer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:OUTER)*/
pub struct OUTERrs;
impl crate::RegisterSpec for OUTERrs {
    type Ux = u32;
}
///`read()` method returns [`outer::R`](R) reader structure
impl crate::Readable for OUTERrs {}
///`write(|w| ..)` method takes [`outer::W`](W) writer structure
impl crate::Writable for OUTERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OUTER to value 0
impl crate::Resettable for OUTERrs {
    const RESET_VALUE: u32 = 0;
}
