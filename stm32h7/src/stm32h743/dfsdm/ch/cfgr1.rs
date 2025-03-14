///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `SITP` reader - Serial interface type for channel 0
pub type SITP_R = crate::FieldReader;
///Field `SITP` writer - Serial interface type for channel 0
pub type SITP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SPICKSEL` reader - SPI clock select for channel 0
pub type SPICKSEL_R = crate::FieldReader;
///Field `SPICKSEL` writer - SPI clock select for channel 0
pub type SPICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SCDEN` reader - Short-circuit detector enable on channel 0
pub type SCDEN_R = crate::BitReader;
///Field `SCDEN` writer - Short-circuit detector enable on channel 0
pub type SCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABEN` reader - Clock absence detector enable on channel 0
pub type CKABEN_R = crate::BitReader;
///Field `CKABEN` writer - Clock absence detector enable on channel 0
pub type CKABEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHEN` reader - Channel 0 enable
pub type CHEN_R = crate::BitReader;
///Field `CHEN` writer - Channel 0 enable
pub type CHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHINSEL` reader - Channel inputs selection
pub type CHINSEL_R = crate::BitReader;
///Field `CHINSEL` writer - Channel inputs selection
pub type CHINSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATMPX` reader - Input data multiplexer for channel 0
pub type DATMPX_R = crate::FieldReader;
///Field `DATMPX` writer - Input data multiplexer for channel 0
pub type DATMPX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DATPACK` reader - Data packing mode in DFSDM_CHDATINyR register
pub type DATPACK_R = crate::FieldReader;
///Field `DATPACK` writer - Data packing mode in DFSDM_CHDATINyR register
pub type DATPACK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKOUTDIV` reader - Output serial clock divider
pub type CKOUTDIV_R = crate::FieldReader;
///Field `CKOUTDIV` writer - Output serial clock divider
pub type CKOUTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CKOUTSRC` reader - Output serial clock source selection
pub type CKOUTSRC_R = crate::BitReader;
///Field `CKOUTSRC` writer - Output serial clock source selection
pub type CKOUTSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDMEN` reader - Global enable for DFSDM interface
pub type DFSDMEN_R = crate::BitReader;
///Field `DFSDMEN` writer - Global enable for DFSDM interface
pub type DFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Serial interface type for channel 0
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - SPI clock select for channel 0
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 5 - Short-circuit detector enable on channel 0
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clock absence detector enable on channel 0
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel 0 enable
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel inputs selection
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:13 - Input data multiplexer for channel 0
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Data packing mode in DFSDM_CHDATINyR register
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:23 - Output serial clock divider
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 30 - Output serial clock source selection
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Global enable for DFSDM interface
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("sitp", &self.sitp())
            .field("spicksel", &self.spicksel())
            .field("scden", &self.scden())
            .field("ckaben", &self.ckaben())
            .field("chen", &self.chen())
            .field("chinsel", &self.chinsel())
            .field("datmpx", &self.datmpx())
            .field("datpack", &self.datpack())
            .field("ckoutdiv", &self.ckoutdiv())
            .field("ckoutsrc", &self.ckoutsrc())
            .field("dfsdmen", &self.dfsdmen())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Serial interface type for channel 0
    #[inline(always)]
    pub fn sitp(&mut self) -> SITP_W<CFGR1rs> {
        SITP_W::new(self, 0)
    }
    ///Bits 2:3 - SPI clock select for channel 0
    #[inline(always)]
    pub fn spicksel(&mut self) -> SPICKSEL_W<CFGR1rs> {
        SPICKSEL_W::new(self, 2)
    }
    ///Bit 5 - Short-circuit detector enable on channel 0
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W<CFGR1rs> {
        SCDEN_W::new(self, 5)
    }
    ///Bit 6 - Clock absence detector enable on channel 0
    #[inline(always)]
    pub fn ckaben(&mut self) -> CKABEN_W<CFGR1rs> {
        CKABEN_W::new(self, 6)
    }
    ///Bit 7 - Channel 0 enable
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W<CFGR1rs> {
        CHEN_W::new(self, 7)
    }
    ///Bit 8 - Channel inputs selection
    #[inline(always)]
    pub fn chinsel(&mut self) -> CHINSEL_W<CFGR1rs> {
        CHINSEL_W::new(self, 8)
    }
    ///Bits 12:13 - Input data multiplexer for channel 0
    #[inline(always)]
    pub fn datmpx(&mut self) -> DATMPX_W<CFGR1rs> {
        DATMPX_W::new(self, 12)
    }
    ///Bits 14:15 - Data packing mode in DFSDM_CHDATINyR register
    #[inline(always)]
    pub fn datpack(&mut self) -> DATPACK_W<CFGR1rs> {
        DATPACK_W::new(self, 14)
    }
    ///Bits 16:23 - Output serial clock divider
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W<CFGR1rs> {
        CKOUTDIV_W::new(self, 16)
    }
    ///Bit 30 - Output serial clock source selection
    #[inline(always)]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<CFGR1rs> {
        CKOUTSRC_W::new(self, 30)
    }
    ///Bit 31 - Global enable for DFSDM interface
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<CFGR1rs> {
        DFSDMEN_W::new(self, 31)
    }
}
/**DFSDM channel configuration 0 register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
