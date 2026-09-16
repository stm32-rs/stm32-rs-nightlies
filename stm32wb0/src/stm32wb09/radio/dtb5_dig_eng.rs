///Register `DTB5_DIG_ENG` reader
pub type R = crate::R<DTB5_DIG_ENGrs>;
///Register `DTB5_DIG_ENG` writer
pub type W = crate::W<DTB5_DIG_ENGrs>;
///Field `RXTX_START_SEL` reader - enable the possibility to control some signals by the other register bits instead of system design:
pub type RXTX_START_SEL_R = crate::BitReader;
///Field `RXTX_START_SEL` writer - enable the possibility to control some signals by the other register bits instead of system design:
pub type RXTX_START_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_ACTIVE` reader - Force TX_ACTIVE signal
pub type TX_ACTIVE_R = crate::BitReader;
///Field `TX_ACTIVE` writer - Force TX_ACTIVE signal
pub type TX_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_ACTIVE` reader - Force RX_ACTIVE signal
pub type RX_ACTIVE_R = crate::BitReader;
///Field `RX_ACTIVE` writer - Force RX_ACTIVE signal
pub type RX_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITIALIZE` reader - Force INITIALIZE signal (emulate a token request of the IP_BLE)
pub type INITIALIZE_R = crate::BitReader;
///Field `INITIALIZE` writer - Force INITIALIZE signal (emulate a token request of the IP_BLE)
pub type INITIALIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_SELECTED_EN` reader - enable port selection
pub type PORT_SELECTED_EN_R = crate::BitReader;
///Field `PORT_SELECTED_EN` writer - enable port selection
pub type PORT_SELECTED_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_SELECTED_0` reader - force port_selected\[0\] signal
pub type PORT_SELECTED_0_R = crate::BitReader;
///Field `PORT_SELECTED_0` writer - force port_selected\[0\] signal
pub type PORT_SELECTED_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable the possibility to control some signals by the other register bits instead of system design:
    #[inline(always)]
    pub fn rxtx_start_sel(&self) -> RXTX_START_SEL_R {
        RXTX_START_SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Force TX_ACTIVE signal
    #[inline(always)]
    pub fn tx_active(&self) -> TX_ACTIVE_R {
        TX_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Force RX_ACTIVE signal
    #[inline(always)]
    pub fn rx_active(&self) -> RX_ACTIVE_R {
        RX_ACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Force INITIALIZE signal (emulate a token request of the IP_BLE)
    #[inline(always)]
    pub fn initialize(&self) -> INITIALIZE_R {
        INITIALIZE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - enable port selection
    #[inline(always)]
    pub fn port_selected_en(&self) -> PORT_SELECTED_EN_R {
        PORT_SELECTED_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - force port_selected\[0\] signal
    #[inline(always)]
    pub fn port_selected_0(&self) -> PORT_SELECTED_0_R {
        PORT_SELECTED_0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTB5_DIG_ENG")
            .field("rxtx_start_sel", &self.rxtx_start_sel())
            .field("tx_active", &self.tx_active())
            .field("rx_active", &self.rx_active())
            .field("initialize", &self.initialize())
            .field("port_selected_en", &self.port_selected_en())
            .field("port_selected_0", &self.port_selected_0())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable the possibility to control some signals by the other register bits instead of system design:
    #[inline(always)]
    pub fn rxtx_start_sel(&mut self) -> RXTX_START_SEL_W<'_, DTB5_DIG_ENGrs> {
        RXTX_START_SEL_W::new(self, 0)
    }
    ///Bit 1 - Force TX_ACTIVE signal
    #[inline(always)]
    pub fn tx_active(&mut self) -> TX_ACTIVE_W<'_, DTB5_DIG_ENGrs> {
        TX_ACTIVE_W::new(self, 1)
    }
    ///Bit 2 - Force RX_ACTIVE signal
    #[inline(always)]
    pub fn rx_active(&mut self) -> RX_ACTIVE_W<'_, DTB5_DIG_ENGrs> {
        RX_ACTIVE_W::new(self, 2)
    }
    ///Bit 3 - Force INITIALIZE signal (emulate a token request of the IP_BLE)
    #[inline(always)]
    pub fn initialize(&mut self) -> INITIALIZE_W<'_, DTB5_DIG_ENGrs> {
        INITIALIZE_W::new(self, 3)
    }
    ///Bit 4 - enable port selection
    #[inline(always)]
    pub fn port_selected_en(&mut self) -> PORT_SELECTED_EN_W<'_, DTB5_DIG_ENGrs> {
        PORT_SELECTED_EN_W::new(self, 4)
    }
    ///Bit 5 - force port_selected\[0\] signal
    #[inline(always)]
    pub fn port_selected_0(&mut self) -> PORT_SELECTED_0_W<'_, DTB5_DIG_ENGrs> {
        PORT_SELECTED_0_W::new(self, 5)
    }
}
/**DTB5_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`dtb5_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb5_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:DTB5_DIG_ENG)*/
pub struct DTB5_DIG_ENGrs;
impl crate::RegisterSpec for DTB5_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`dtb5_dig_eng::R`](R) reader structure
impl crate::Readable for DTB5_DIG_ENGrs {}
///`write(|w| ..)` method takes [`dtb5_dig_eng::W`](W) writer structure
impl crate::Writable for DTB5_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTB5_DIG_ENG to value 0
impl crate::Resettable for DTB5_DIG_ENGrs {}
