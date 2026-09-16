///Register `ENR` reader
pub type R = crate::R<ENRrs>;
///Register `ENR` writer
pub type W = crate::W<ENRrs>;
///Field `CLKWISE_IE` reader - Clock Wise Interrupt and Wakeup Enable
pub type CLKWISE_IE_R = crate::BitReader;
///Field `CLKWISE_IE` writer - Clock Wise Interrupt and Wakeup Enable
pub type CLKWISE_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKWISE_IE` reader - Anti Clock Wise Interrupt and Wakeup Enable
pub type ACLKWISE_IE_R = crate::BitReader;
///Field `ACLKWISE_IE` writer - Anti Clock Wise Interrupt and Wakeup Enable
pub type ACLKWISE_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP_IE` reader - Tamper Interrupt and Wakeup Enable
pub type TAMP_IE_R = crate::BitReader;
///Field `TAMP_IE` writer - Tamper Interrupt and Wakeup Enable
pub type TAMP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNT_OFB_WKP_IE` reader - LCAB Counter Out Of Bound wakeup enable
pub type CNT_OFB_WKP_IE_R = crate::BitReader;
///Field `CNT_OFB_WKP_IE` writer - LCAB Counter Out Of Bound wakeup enable
pub type CNT_OFB_WKP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCSC_EN` reader - LCSC Enable
pub type LCSC_EN_R = crate::BitReader;
///Field `LCSC_EN` writer - LCSC Enable
pub type LCSC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clock Wise Interrupt and Wakeup Enable
    #[inline(always)]
    pub fn clkwise_ie(&self) -> CLKWISE_IE_R {
        CLKWISE_IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Anti Clock Wise Interrupt and Wakeup Enable
    #[inline(always)]
    pub fn aclkwise_ie(&self) -> ACLKWISE_IE_R {
        ACLKWISE_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper Interrupt and Wakeup Enable
    #[inline(always)]
    pub fn tamp_ie(&self) -> TAMP_IE_R {
        TAMP_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LCAB Counter Out Of Bound wakeup enable
    #[inline(always)]
    pub fn cnt_ofb_wkp_ie(&self) -> CNT_OFB_WKP_IE_R {
        CNT_OFB_WKP_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 31 - LCSC Enable
    #[inline(always)]
    pub fn lcsc_en(&self) -> LCSC_EN_R {
        LCSC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENR")
            .field("clkwise_ie", &self.clkwise_ie())
            .field("aclkwise_ie", &self.aclkwise_ie())
            .field("tamp_ie", &self.tamp_ie())
            .field("cnt_ofb_wkp_ie", &self.cnt_ofb_wkp_ie())
            .field("lcsc_en", &self.lcsc_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock Wise Interrupt and Wakeup Enable
    #[inline(always)]
    pub fn clkwise_ie(&mut self) -> CLKWISE_IE_W<'_, ENRrs> {
        CLKWISE_IE_W::new(self, 0)
    }
    ///Bit 1 - Anti Clock Wise Interrupt and Wakeup Enable
    #[inline(always)]
    pub fn aclkwise_ie(&mut self) -> ACLKWISE_IE_W<'_, ENRrs> {
        ACLKWISE_IE_W::new(self, 1)
    }
    ///Bit 2 - Tamper Interrupt and Wakeup Enable
    #[inline(always)]
    pub fn tamp_ie(&mut self) -> TAMP_IE_W<'_, ENRrs> {
        TAMP_IE_W::new(self, 2)
    }
    ///Bit 3 - LCAB Counter Out Of Bound wakeup enable
    #[inline(always)]
    pub fn cnt_ofb_wkp_ie(&mut self) -> CNT_OFB_WKP_IE_W<'_, ENRrs> {
        CNT_OFB_WKP_IE_W::new(self, 3)
    }
    ///Bit 31 - LCSC Enable
    #[inline(always)]
    pub fn lcsc_en(&mut self) -> LCSC_EN_W<'_, ENRrs> {
        LCSC_EN_W::new(self, 31)
    }
}
/**LCSC_ENR register

You can [`read`](crate::Reg::read) this register and get [`enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:ENR)*/
pub struct ENRrs;
impl crate::RegisterSpec for ENRrs {
    type Ux = u32;
}
///`read()` method returns [`enr::R`](R) reader structure
impl crate::Readable for ENRrs {}
///`write(|w| ..)` method takes [`enr::W`](W) writer structure
impl crate::Writable for ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENR to value 0
impl crate::Resettable for ENRrs {}
