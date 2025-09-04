///Register `SDTR%s` reader
pub type R = crate::R<SDTRrs>;
///Register `SDTR%s` writer
pub type W = crate::W<SDTRrs>;
///Field `TMRD` reader - Load Mode Register to Active
pub type TMRD_R = crate::FieldReader;
///Field `TMRD` writer - Load Mode Register to Active
pub type TMRD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TXSR` reader - Exit self-refresh delay
pub type TXSR_R = crate::FieldReader;
///Field `TXSR` writer - Exit self-refresh delay
pub type TXSR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TRAS` reader - Self refresh time
pub type TRAS_R = crate::FieldReader;
///Field `TRAS` writer - Self refresh time
pub type TRAS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TRC` reader - Row cycle delay
pub type TRC_R = crate::FieldReader;
///Field `TRC` writer - Row cycle delay
pub type TRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TWR` reader - Recovery delay
pub type TWR_R = crate::FieldReader;
///Field `TWR` writer - Recovery delay
pub type TWR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TRP` reader - Row precharge delay
pub type TRP_R = crate::FieldReader;
///Field `TRP` writer - Row precharge delay
pub type TRP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TRCD` reader - Row to column delay
pub type TRCD_R = crate::FieldReader;
///Field `TRCD` writer - Row to column delay
pub type TRCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bits 0:3 - Load Mode Register to Active
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Exit self-refresh delay
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Self refresh time
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Row cycle delay
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Recovery delay
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Row precharge delay
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Row to column delay
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDTR")
            .field("tmrd", &self.tmrd())
            .field("txsr", &self.txsr())
            .field("tras", &self.tras())
            .field("trc", &self.trc())
            .field("twr", &self.twr())
            .field("trp", &self.trp())
            .field("trcd", &self.trcd())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Load Mode Register to Active
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W<SDTRrs> {
        TMRD_W::new(self, 0)
    }
    ///Bits 4:7 - Exit self-refresh delay
    #[inline(always)]
    pub fn txsr(&mut self) -> TXSR_W<SDTRrs> {
        TXSR_W::new(self, 4)
    }
    ///Bits 8:11 - Self refresh time
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W<SDTRrs> {
        TRAS_W::new(self, 8)
    }
    ///Bits 12:15 - Row cycle delay
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W<SDTRrs> {
        TRC_W::new(self, 12)
    }
    ///Bits 16:19 - Recovery delay
    #[inline(always)]
    pub fn twr(&mut self) -> TWR_W<SDTRrs> {
        TWR_W::new(self, 16)
    }
    ///Bits 20:23 - Row precharge delay
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W<SDTRrs> {
        TRP_W::new(self, 20)
    }
    ///Bits 24:27 - Row to column delay
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W<SDTRrs> {
        TRCD_W::new(self, 24)
    }
}
/**SDRAM Timing register %s

You can [`read`](crate::Reg::read) this register and get [`sdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#FMC:SDTR[1])*/
pub struct SDTRrs;
impl crate::RegisterSpec for SDTRrs {
    type Ux = u32;
}
///`read()` method returns [`sdtr::R`](R) reader structure
impl crate::Readable for SDTRrs {}
///`write(|w| ..)` method takes [`sdtr::W`](W) writer structure
impl crate::Writable for SDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDTR%s to value 0x0fff_ffff
impl crate::Resettable for SDTRrs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
