#[doc = "Register `DDRCTRL_PCFGR_0` reader"]
pub type R = crate::R<DDRCTRL_PCFGR_0rs>;
#[doc = "Register `DDRCTRL_PCFGR_0` writer"]
pub type W = crate::W<DDRCTRL_PCFGR_0rs>;
#[doc = "Field `RD_PORT_PRIORITY` reader - RD_PORT_PRIORITY"]
pub type RD_PORT_PRIORITY_R = crate::FieldReader<u16>;
#[doc = "Field `RD_PORT_PRIORITY` writer - RD_PORT_PRIORITY"]
pub type RD_PORT_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RD_PORT_AGING_EN` reader - RD_PORT_AGING_EN"]
pub type RD_PORT_AGING_EN_R = crate::BitReader;
#[doc = "Field `RD_PORT_AGING_EN` writer - RD_PORT_AGING_EN"]
pub type RD_PORT_AGING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_PORT_URGENT_EN` reader - RD_PORT_URGENT_EN"]
pub type RD_PORT_URGENT_EN_R = crate::BitReader;
#[doc = "Field `RD_PORT_URGENT_EN` writer - RD_PORT_URGENT_EN"]
pub type RD_PORT_URGENT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_PORT_PAGEMATCH_EN` reader - RD_PORT_PAGEMATCH_EN"]
pub type RD_PORT_PAGEMATCH_EN_R = crate::BitReader;
#[doc = "Field `RD_PORT_PAGEMATCH_EN` writer - RD_PORT_PAGEMATCH_EN"]
pub type RD_PORT_PAGEMATCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDWR_ORDERED_EN` reader - RDWR_ORDERED_EN"]
pub type RDWR_ORDERED_EN_R = crate::BitReader;
#[doc = "Field `RDWR_ORDERED_EN` writer - RDWR_ORDERED_EN"]
pub type RDWR_ORDERED_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - RD_PORT_PRIORITY"]
    #[inline(always)]
    pub fn rd_port_priority(&self) -> RD_PORT_PRIORITY_R {
        RD_PORT_PRIORITY_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - RD_PORT_AGING_EN"]
    #[inline(always)]
    pub fn rd_port_aging_en(&self) -> RD_PORT_AGING_EN_R {
        RD_PORT_AGING_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RD_PORT_URGENT_EN"]
    #[inline(always)]
    pub fn rd_port_urgent_en(&self) -> RD_PORT_URGENT_EN_R {
        RD_PORT_URGENT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RD_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    pub fn rd_port_pagematch_en(&self) -> RD_PORT_PAGEMATCH_EN_R {
        RD_PORT_PAGEMATCH_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RDWR_ORDERED_EN"]
    #[inline(always)]
    pub fn rdwr_ordered_en(&self) -> RDWR_ORDERED_EN_R {
        RDWR_ORDERED_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - RD_PORT_PRIORITY"]
    #[inline(always)]
    #[must_use]
    pub fn rd_port_priority(&mut self) -> RD_PORT_PRIORITY_W<DDRCTRL_PCFGR_0rs> {
        RD_PORT_PRIORITY_W::new(self, 0)
    }
    #[doc = "Bit 12 - RD_PORT_AGING_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rd_port_aging_en(&mut self) -> RD_PORT_AGING_EN_W<DDRCTRL_PCFGR_0rs> {
        RD_PORT_AGING_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - RD_PORT_URGENT_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rd_port_urgent_en(&mut self) -> RD_PORT_URGENT_EN_W<DDRCTRL_PCFGR_0rs> {
        RD_PORT_URGENT_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - RD_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rd_port_pagematch_en(&mut self) -> RD_PORT_PAGEMATCH_EN_W<DDRCTRL_PCFGR_0rs> {
        RD_PORT_PAGEMATCH_EN_W::new(self, 14)
    }
    #[doc = "Bit 16 - RDWR_ORDERED_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rdwr_ordered_en(&mut self) -> RDWR_ORDERED_EN_W<DDRCTRL_PCFGR_0rs> {
        RDWR_ORDERED_EN_W::new(self, 16)
    }
}
#[doc = "DDRCTRL port 0 configuration read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgr_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgr_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PCFGR_0rs;
impl crate::RegisterSpec for DDRCTRL_PCFGR_0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_pcfgr_0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGR_0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_pcfgr_0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGR_0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGR_0 to value 0x4000"]
impl crate::Resettable for DDRCTRL_PCFGR_0rs {
    const RESET_VALUE: u32 = 0x4000;
}
