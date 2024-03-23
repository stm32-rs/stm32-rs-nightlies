#[doc = "Register `DDRCTRL_PCFGW_1` reader"]
pub type R = crate::R<DDRCTRL_PCFGW_1rs>;
#[doc = "Register `DDRCTRL_PCFGW_1` writer"]
pub type W = crate::W<DDRCTRL_PCFGW_1rs>;
#[doc = "Field `WR_PORT_PRIORITY` reader - WR_PORT_PRIORITY"]
pub type WR_PORT_PRIORITY_R = crate::FieldReader<u16>;
#[doc = "Field `WR_PORT_PRIORITY` writer - WR_PORT_PRIORITY"]
pub type WR_PORT_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `WR_PORT_AGING_EN` reader - WR_PORT_AGING_EN"]
pub type WR_PORT_AGING_EN_R = crate::BitReader;
#[doc = "Field `WR_PORT_AGING_EN` writer - WR_PORT_AGING_EN"]
pub type WR_PORT_AGING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_PORT_URGENT_EN` reader - WR_PORT_URGENT_EN"]
pub type WR_PORT_URGENT_EN_R = crate::BitReader;
#[doc = "Field `WR_PORT_URGENT_EN` writer - WR_PORT_URGENT_EN"]
pub type WR_PORT_URGENT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_PORT_PAGEMATCH_EN` reader - WR_PORT_PAGEMATCH_EN"]
pub type WR_PORT_PAGEMATCH_EN_R = crate::BitReader;
#[doc = "Field `WR_PORT_PAGEMATCH_EN` writer - WR_PORT_PAGEMATCH_EN"]
pub type WR_PORT_PAGEMATCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - WR_PORT_PRIORITY"]
    #[inline(always)]
    pub fn wr_port_priority(&self) -> WR_PORT_PRIORITY_R {
        WR_PORT_PRIORITY_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - WR_PORT_AGING_EN"]
    #[inline(always)]
    pub fn wr_port_aging_en(&self) -> WR_PORT_AGING_EN_R {
        WR_PORT_AGING_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WR_PORT_URGENT_EN"]
    #[inline(always)]
    pub fn wr_port_urgent_en(&self) -> WR_PORT_URGENT_EN_R {
        WR_PORT_URGENT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - WR_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    pub fn wr_port_pagematch_en(&self) -> WR_PORT_PAGEMATCH_EN_R {
        WR_PORT_PAGEMATCH_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - WR_PORT_PRIORITY"]
    #[inline(always)]
    #[must_use]
    pub fn wr_port_priority(&mut self) -> WR_PORT_PRIORITY_W<DDRCTRL_PCFGW_1rs> {
        WR_PORT_PRIORITY_W::new(self, 0)
    }
    #[doc = "Bit 12 - WR_PORT_AGING_EN"]
    #[inline(always)]
    #[must_use]
    pub fn wr_port_aging_en(&mut self) -> WR_PORT_AGING_EN_W<DDRCTRL_PCFGW_1rs> {
        WR_PORT_AGING_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - WR_PORT_URGENT_EN"]
    #[inline(always)]
    #[must_use]
    pub fn wr_port_urgent_en(&mut self) -> WR_PORT_URGENT_EN_W<DDRCTRL_PCFGW_1rs> {
        WR_PORT_URGENT_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - WR_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    #[must_use]
    pub fn wr_port_pagematch_en(&mut self) -> WR_PORT_PAGEMATCH_EN_W<DDRCTRL_PCFGW_1rs> {
        WR_PORT_PAGEMATCH_EN_W::new(self, 14)
    }
}
#[doc = "DDRCTRL port 1 configuration write register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgw_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgw_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PCFGW_1rs;
impl crate::RegisterSpec for DDRCTRL_PCFGW_1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_pcfgw_1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGW_1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_pcfgw_1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGW_1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGW_1 to value 0x4000"]
impl crate::Resettable for DDRCTRL_PCFGW_1rs {
    const RESET_VALUE: u32 = 0x4000;
}
