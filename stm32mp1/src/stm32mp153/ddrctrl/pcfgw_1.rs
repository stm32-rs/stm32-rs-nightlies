///Register `PCFGW_1` reader
pub type R = crate::R<PCFGW_1rs>;
///Register `PCFGW_1` writer
pub type W = crate::W<PCFGW_1rs>;
///Field `WR_PORT_PRIORITY` reader - WR_PORT_PRIORITY
pub type WR_PORT_PRIORITY_R = crate::FieldReader<u16>;
///Field `WR_PORT_PRIORITY` writer - WR_PORT_PRIORITY
pub type WR_PORT_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `WR_PORT_AGING_EN` reader - WR_PORT_AGING_EN
pub type WR_PORT_AGING_EN_R = crate::BitReader;
///Field `WR_PORT_AGING_EN` writer - WR_PORT_AGING_EN
pub type WR_PORT_AGING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WR_PORT_URGENT_EN` reader - WR_PORT_URGENT_EN
pub type WR_PORT_URGENT_EN_R = crate::BitReader;
///Field `WR_PORT_URGENT_EN` writer - WR_PORT_URGENT_EN
pub type WR_PORT_URGENT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WR_PORT_PAGEMATCH_EN` reader - WR_PORT_PAGEMATCH_EN
pub type WR_PORT_PAGEMATCH_EN_R = crate::BitReader;
///Field `WR_PORT_PAGEMATCH_EN` writer - WR_PORT_PAGEMATCH_EN
pub type WR_PORT_PAGEMATCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - WR_PORT_PRIORITY
    #[inline(always)]
    pub fn wr_port_priority(&self) -> WR_PORT_PRIORITY_R {
        WR_PORT_PRIORITY_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 12 - WR_PORT_AGING_EN
    #[inline(always)]
    pub fn wr_port_aging_en(&self) -> WR_PORT_AGING_EN_R {
        WR_PORT_AGING_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - WR_PORT_URGENT_EN
    #[inline(always)]
    pub fn wr_port_urgent_en(&self) -> WR_PORT_URGENT_EN_R {
        WR_PORT_URGENT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - WR_PORT_PAGEMATCH_EN
    #[inline(always)]
    pub fn wr_port_pagematch_en(&self) -> WR_PORT_PAGEMATCH_EN_R {
        WR_PORT_PAGEMATCH_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCFGW_1")
            .field("wr_port_priority", &self.wr_port_priority())
            .field("wr_port_aging_en", &self.wr_port_aging_en())
            .field("wr_port_urgent_en", &self.wr_port_urgent_en())
            .field("wr_port_pagematch_en", &self.wr_port_pagematch_en())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - WR_PORT_PRIORITY
    #[inline(always)]
    pub fn wr_port_priority(&mut self) -> WR_PORT_PRIORITY_W<'_, PCFGW_1rs> {
        WR_PORT_PRIORITY_W::new(self, 0)
    }
    ///Bit 12 - WR_PORT_AGING_EN
    #[inline(always)]
    pub fn wr_port_aging_en(&mut self) -> WR_PORT_AGING_EN_W<'_, PCFGW_1rs> {
        WR_PORT_AGING_EN_W::new(self, 12)
    }
    ///Bit 13 - WR_PORT_URGENT_EN
    #[inline(always)]
    pub fn wr_port_urgent_en(&mut self) -> WR_PORT_URGENT_EN_W<'_, PCFGW_1rs> {
        WR_PORT_URGENT_EN_W::new(self, 13)
    }
    ///Bit 14 - WR_PORT_PAGEMATCH_EN
    #[inline(always)]
    pub fn wr_port_pagematch_en(&mut self) -> WR_PORT_PAGEMATCH_EN_W<'_, PCFGW_1rs> {
        WR_PORT_PAGEMATCH_EN_W::new(self, 14)
    }
}
/**DDRCTRL port 1 configuration write register

You can [`read`](crate::Reg::read) this register and get [`pcfgw_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgw_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:PCFGW_1)*/
pub struct PCFGW_1rs;
impl crate::RegisterSpec for PCFGW_1rs {
    type Ux = u32;
}
///`read()` method returns [`pcfgw_1::R`](R) reader structure
impl crate::Readable for PCFGW_1rs {}
///`write(|w| ..)` method takes [`pcfgw_1::W`](W) writer structure
impl crate::Writable for PCFGW_1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCFGW_1 to value 0x4000
impl crate::Resettable for PCFGW_1rs {
    const RESET_VALUE: u32 = 0x4000;
}
