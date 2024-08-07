///Register `DDRCTRL_ODTCFG` reader
pub type R = crate::R<DDRCTRL_ODTCFGrs>;
///Register `DDRCTRL_ODTCFG` writer
pub type W = crate::W<DDRCTRL_ODTCFGrs>;
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
        f.debug_struct("DDRCTRL_ODTCFG")
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
    #[must_use]
    pub fn rd_odt_delay(&mut self) -> RD_ODT_DELAY_W<DDRCTRL_ODTCFGrs> {
        RD_ODT_DELAY_W::new(self, 2)
    }
    ///Bits 8:11 - RD_ODT_HOLD
    #[inline(always)]
    #[must_use]
    pub fn rd_odt_hold(&mut self) -> RD_ODT_HOLD_W<DDRCTRL_ODTCFGrs> {
        RD_ODT_HOLD_W::new(self, 8)
    }
    ///Bits 16:20 - WR_ODT_DELAY
    #[inline(always)]
    #[must_use]
    pub fn wr_odt_delay(&mut self) -> WR_ODT_DELAY_W<DDRCTRL_ODTCFGrs> {
        WR_ODT_DELAY_W::new(self, 16)
    }
    ///Bits 24:27 - WR_ODT_HOLD
    #[inline(always)]
    #[must_use]
    pub fn wr_odt_hold(&mut self) -> WR_ODT_HOLD_W<DDRCTRL_ODTCFGrs> {
        WR_ODT_HOLD_W::new(self, 24)
    }
}
/**DDRCTRL ODT configuration register

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_odtcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_odtcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_ODTCFG)*/
pub struct DDRCTRL_ODTCFGrs;
impl crate::RegisterSpec for DDRCTRL_ODTCFGrs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_odtcfg::R`](R) reader structure
impl crate::Readable for DDRCTRL_ODTCFGrs {}
///`write(|w| ..)` method takes [`ddrctrl_odtcfg::W`](W) writer structure
impl crate::Writable for DDRCTRL_ODTCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_ODTCFG to value 0x0400_0400
impl crate::Resettable for DDRCTRL_ODTCFGrs {
    const RESET_VALUE: u32 = 0x0400_0400;
}
