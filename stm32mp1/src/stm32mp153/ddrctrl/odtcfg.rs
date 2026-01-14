///Register `ODTCFG` reader
pub type R = crate::R<ODTCFGrs>;
///Register `ODTCFG` writer
pub type W = crate::W<ODTCFGrs>;
///Field `RD_ODT_DELAY` reader - RD_ODT_DELAY
pub type RD_ODT_DELAY_R = crate::FieldReader;
///Field `RD_ODT_DELAY` writer - RD_ODT_DELAY
pub type RD_ODT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RD_ODT_HOLD` reader - RD_ODT_HOLD
pub type RD_ODT_HOLD_R = crate::FieldReader;
///Field `RD_ODT_HOLD` writer - RD_ODT_HOLD
pub type RD_ODT_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `WR_ODT_DELAY` reader - WR_ODT_DELAY
pub type WR_ODT_DELAY_R = crate::FieldReader;
///Field `WR_ODT_DELAY` writer - WR_ODT_DELAY
pub type WR_ODT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `WR_ODT_HOLD` reader - WR_ODT_HOLD
pub type WR_ODT_HOLD_R = crate::FieldReader;
///Field `WR_ODT_HOLD` writer - WR_ODT_HOLD
pub type WR_ODT_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 2:6 - RD_ODT_DELAY
    #[inline(always)]
    pub fn rd_odt_delay(&self) -> RD_ODT_DELAY_R {
        RD_ODT_DELAY_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 8:11 - RD_ODT_HOLD
    #[inline(always)]
    pub fn rd_odt_hold(&self) -> RD_ODT_HOLD_R {
        RD_ODT_HOLD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:20 - WR_ODT_DELAY
    #[inline(always)]
    pub fn wr_odt_delay(&self) -> WR_ODT_DELAY_R {
        WR_ODT_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:27 - WR_ODT_HOLD
    #[inline(always)]
    pub fn wr_odt_hold(&self) -> WR_ODT_HOLD_R {
        WR_ODT_HOLD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODTCFG")
            .field("rd_odt_delay", &self.rd_odt_delay())
            .field("rd_odt_hold", &self.rd_odt_hold())
            .field("wr_odt_delay", &self.wr_odt_delay())
            .field("wr_odt_hold", &self.wr_odt_hold())
            .finish()
    }
}
impl W {
    ///Bits 2:6 - RD_ODT_DELAY
    #[inline(always)]
    pub fn rd_odt_delay(&mut self) -> RD_ODT_DELAY_W<'_, ODTCFGrs> {
        RD_ODT_DELAY_W::new(self, 2)
    }
    ///Bits 8:11 - RD_ODT_HOLD
    #[inline(always)]
    pub fn rd_odt_hold(&mut self) -> RD_ODT_HOLD_W<'_, ODTCFGrs> {
        RD_ODT_HOLD_W::new(self, 8)
    }
    ///Bits 16:20 - WR_ODT_DELAY
    #[inline(always)]
    pub fn wr_odt_delay(&mut self) -> WR_ODT_DELAY_W<'_, ODTCFGrs> {
        WR_ODT_DELAY_W::new(self, 16)
    }
    ///Bits 24:27 - WR_ODT_HOLD
    #[inline(always)]
    pub fn wr_odt_hold(&mut self) -> WR_ODT_HOLD_W<'_, ODTCFGrs> {
        WR_ODT_HOLD_W::new(self, 24)
    }
}
/**DDRCTRL ODT configuration register

You can [`read`](crate::Reg::read) this register and get [`odtcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odtcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:ODTCFG)*/
pub struct ODTCFGrs;
impl crate::RegisterSpec for ODTCFGrs {
    type Ux = u32;
}
///`read()` method returns [`odtcfg::R`](R) reader structure
impl crate::Readable for ODTCFGrs {}
///`write(|w| ..)` method takes [`odtcfg::W`](W) writer structure
impl crate::Writable for ODTCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ODTCFG to value 0x0400_0400
impl crate::Resettable for ODTCFGrs {
    const RESET_VALUE: u32 = 0x0400_0400;
}
