///Register `PCFGR_0` reader
pub type R = crate::R<PCFGR_0rs>;
///Register `PCFGR_0` writer
pub type W = crate::W<PCFGR_0rs>;
///Field `RD_PORT_PRIORITY` reader - RD_PORT_PRIORITY
pub type RD_PORT_PRIORITY_R = crate::FieldReader<u16>;
///Field `RD_PORT_PRIORITY` writer - RD_PORT_PRIORITY
pub type RD_PORT_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RD_PORT_AGING_EN` reader - RD_PORT_AGING_EN
pub type RD_PORT_AGING_EN_R = crate::BitReader;
///Field `RD_PORT_AGING_EN` writer - RD_PORT_AGING_EN
pub type RD_PORT_AGING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD_PORT_URGENT_EN` reader - RD_PORT_URGENT_EN
pub type RD_PORT_URGENT_EN_R = crate::BitReader;
///Field `RD_PORT_URGENT_EN` writer - RD_PORT_URGENT_EN
pub type RD_PORT_URGENT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD_PORT_PAGEMATCH_EN` reader - RD_PORT_PAGEMATCH_EN
pub type RD_PORT_PAGEMATCH_EN_R = crate::BitReader;
///Field `RD_PORT_PAGEMATCH_EN` writer - RD_PORT_PAGEMATCH_EN
pub type RD_PORT_PAGEMATCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDWR_ORDERED_EN` reader - RDWR_ORDERED_EN
pub type RDWR_ORDERED_EN_R = crate::BitReader;
///Field `RDWR_ORDERED_EN` writer - RDWR_ORDERED_EN
pub type RDWR_ORDERED_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - RD_PORT_PRIORITY
    #[inline(always)]
    pub fn rd_port_priority(&self) -> RD_PORT_PRIORITY_R {
        RD_PORT_PRIORITY_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 12 - RD_PORT_AGING_EN
    #[inline(always)]
    pub fn rd_port_aging_en(&self) -> RD_PORT_AGING_EN_R {
        RD_PORT_AGING_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RD_PORT_URGENT_EN
    #[inline(always)]
    pub fn rd_port_urgent_en(&self) -> RD_PORT_URGENT_EN_R {
        RD_PORT_URGENT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RD_PORT_PAGEMATCH_EN
    #[inline(always)]
    pub fn rd_port_pagematch_en(&self) -> RD_PORT_PAGEMATCH_EN_R {
        RD_PORT_PAGEMATCH_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - RDWR_ORDERED_EN
    #[inline(always)]
    pub fn rdwr_ordered_en(&self) -> RDWR_ORDERED_EN_R {
        RDWR_ORDERED_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCFGR_0")
            .field("rd_port_priority", &self.rd_port_priority())
            .field("rd_port_aging_en", &self.rd_port_aging_en())
            .field("rd_port_urgent_en", &self.rd_port_urgent_en())
            .field("rd_port_pagematch_en", &self.rd_port_pagematch_en())
            .field("rdwr_ordered_en", &self.rdwr_ordered_en())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - RD_PORT_PRIORITY
    #[inline(always)]
    pub fn rd_port_priority(&mut self) -> RD_PORT_PRIORITY_W<'_, PCFGR_0rs> {
        RD_PORT_PRIORITY_W::new(self, 0)
    }
    ///Bit 12 - RD_PORT_AGING_EN
    #[inline(always)]
    pub fn rd_port_aging_en(&mut self) -> RD_PORT_AGING_EN_W<'_, PCFGR_0rs> {
        RD_PORT_AGING_EN_W::new(self, 12)
    }
    ///Bit 13 - RD_PORT_URGENT_EN
    #[inline(always)]
    pub fn rd_port_urgent_en(&mut self) -> RD_PORT_URGENT_EN_W<'_, PCFGR_0rs> {
        RD_PORT_URGENT_EN_W::new(self, 13)
    }
    ///Bit 14 - RD_PORT_PAGEMATCH_EN
    #[inline(always)]
    pub fn rd_port_pagematch_en(&mut self) -> RD_PORT_PAGEMATCH_EN_W<'_, PCFGR_0rs> {
        RD_PORT_PAGEMATCH_EN_W::new(self, 14)
    }
    ///Bit 16 - RDWR_ORDERED_EN
    #[inline(always)]
    pub fn rdwr_ordered_en(&mut self) -> RDWR_ORDERED_EN_W<'_, PCFGR_0rs> {
        RDWR_ORDERED_EN_W::new(self, 16)
    }
}
/**DDRCTRL port 0 configuration read register

You can [`read`](crate::Reg::read) this register and get [`pcfgr_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgr_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGR_0)*/
pub struct PCFGR_0rs;
impl crate::RegisterSpec for PCFGR_0rs {
    type Ux = u32;
}
///`read()` method returns [`pcfgr_0::R`](R) reader structure
impl crate::Readable for PCFGR_0rs {}
///`write(|w| ..)` method takes [`pcfgr_0::W`](W) writer structure
impl crate::Writable for PCFGR_0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCFGR_0 to value 0x4000
impl crate::Resettable for PCFGR_0rs {
    const RESET_VALUE: u32 = 0x4000;
}
