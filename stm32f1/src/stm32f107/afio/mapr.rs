///Register `MAPR` reader
pub type R = crate::R<MAPRrs>;
///Register `MAPR` writer
pub type W = crate::W<MAPRrs>;
///Field `SPI1_REMAP` reader - SPI1 remapping
pub type SPI1_REMAP_R = crate::BitReader;
///Field `SPI1_REMAP` writer - SPI1 remapping
pub type SPI1_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1_REMAP` reader - I2C1 remapping
pub type I2C1_REMAP_R = crate::BitReader;
///Field `I2C1_REMAP` writer - I2C1 remapping
pub type I2C1_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1_REMAP` reader - USART1 remapping
pub type USART1_REMAP_R = crate::BitReader;
///Field `USART1_REMAP` writer - USART1 remapping
pub type USART1_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2_REMAP` reader - USART2 remapping
pub type USART2_REMAP_R = crate::BitReader;
///Field `USART2_REMAP` writer - USART2 remapping
pub type USART2_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3_REMAP` reader - USART3 remapping
pub type USART3_REMAP_R = crate::FieldReader;
///Field `USART3_REMAP` writer - USART3 remapping
pub type USART3_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM1_REMAP` reader - TIM1 remapping
pub type TIM1_REMAP_R = crate::FieldReader;
///Field `TIM1_REMAP` writer - TIM1 remapping
pub type TIM1_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM2_REMAP` reader - TIM2 remapping
pub type TIM2_REMAP_R = crate::FieldReader;
///Field `TIM2_REMAP` writer - TIM2 remapping
pub type TIM2_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM3_REMAP` reader - TIM3 remapping
pub type TIM3_REMAP_R = crate::FieldReader;
///Field `TIM3_REMAP` writer - TIM3 remapping
pub type TIM3_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM4_REMAP` reader - TIM4 remapping
pub type TIM4_REMAP_R = crate::BitReader;
///Field `TIM4_REMAP` writer - TIM4 remapping
pub type TIM4_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAN1_REMAP` reader - CAN1 remapping
pub type CAN1_REMAP_R = crate::FieldReader;
///Field `CAN1_REMAP` writer - CAN1 remapping
pub type CAN1_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PD01_REMAP` reader - Port D0/Port D1 mapping on OSCIN/OSCOUT
pub type PD01_REMAP_R = crate::BitReader;
///Field `PD01_REMAP` writer - Port D0/Port D1 mapping on OSCIN/OSCOUT
pub type PD01_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5CH4_IREMAP` reader - Set and cleared by software
pub type TIM5CH4_IREMAP_R = crate::BitReader;
///Field `TIM5CH4_IREMAP` writer - Set and cleared by software
pub type TIM5CH4_IREMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH_REMAP` reader - Ethernet MAC I/O remapping
pub type ETH_REMAP_R = crate::BitReader;
///Field `ETH_REMAP` writer - Ethernet MAC I/O remapping
pub type ETH_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAN2_REMAP` reader - CAN2 I/O remapping
pub type CAN2_REMAP_R = crate::BitReader;
///Field `CAN2_REMAP` writer - CAN2 I/O remapping
pub type CAN2_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MII_RMII_SEL` reader - MII or RMII selection
pub type MII_RMII_SEL_R = crate::BitReader;
///Field `MII_RMII_SEL` writer - MII or RMII selection
pub type MII_RMII_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWJ_CFG` writer - Serial wire JTAG configuration
pub type SWJ_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI3_REMAP` reader - SPI3/I2S3 remapping
pub type SPI3_REMAP_R = crate::BitReader;
///Field `SPI3_REMAP` writer - SPI3/I2S3 remapping
pub type SPI3_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM2ITR1_IREMAP` reader - TIM2 internal trigger 1 remapping
pub type TIM2ITR1_IREMAP_R = crate::BitReader;
///Field `TIM2ITR1_IREMAP` writer - TIM2 internal trigger 1 remapping
pub type TIM2ITR1_IREMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTP_PPS_REMAP` reader - Ethernet PTP PPS remapping
pub type PTP_PPS_REMAP_R = crate::BitReader;
///Field `PTP_PPS_REMAP` writer - Ethernet PTP PPS remapping
pub type PTP_PPS_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPI1 remapping
    #[inline(always)]
    pub fn spi1_remap(&self) -> SPI1_REMAP_R {
        SPI1_REMAP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C1 remapping
    #[inline(always)]
    pub fn i2c1_remap(&self) -> I2C1_REMAP_R {
        I2C1_REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - USART1 remapping
    #[inline(always)]
    pub fn usart1_remap(&self) -> USART1_REMAP_R {
        USART1_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - USART2 remapping
    #[inline(always)]
    pub fn usart2_remap(&self) -> USART2_REMAP_R {
        USART2_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - USART3 remapping
    #[inline(always)]
    pub fn usart3_remap(&self) -> USART3_REMAP_R {
        USART3_REMAP_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - TIM1 remapping
    #[inline(always)]
    pub fn tim1_remap(&self) -> TIM1_REMAP_R {
        TIM1_REMAP_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - TIM2 remapping
    #[inline(always)]
    pub fn tim2_remap(&self) -> TIM2_REMAP_R {
        TIM2_REMAP_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - TIM3 remapping
    #[inline(always)]
    pub fn tim3_remap(&self) -> TIM3_REMAP_R {
        TIM3_REMAP_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - TIM4 remapping
    #[inline(always)]
    pub fn tim4_remap(&self) -> TIM4_REMAP_R {
        TIM4_REMAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - CAN1 remapping
    #[inline(always)]
    pub fn can1_remap(&self) -> CAN1_REMAP_R {
        CAN1_REMAP_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT
    #[inline(always)]
    pub fn pd01_remap(&self) -> PD01_REMAP_R {
        PD01_REMAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Set and cleared by software
    #[inline(always)]
    pub fn tim5ch4_iremap(&self) -> TIM5CH4_IREMAP_R {
        TIM5CH4_IREMAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - Ethernet MAC I/O remapping
    #[inline(always)]
    pub fn eth_remap(&self) -> ETH_REMAP_R {
        ETH_REMAP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CAN2 I/O remapping
    #[inline(always)]
    pub fn can2_remap(&self) -> CAN2_REMAP_R {
        CAN2_REMAP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - MII or RMII selection
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 28 - SPI3/I2S3 remapping
    #[inline(always)]
    pub fn spi3_remap(&self) -> SPI3_REMAP_R {
        SPI3_REMAP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - TIM2 internal trigger 1 remapping
    #[inline(always)]
    pub fn tim2itr1_iremap(&self) -> TIM2ITR1_IREMAP_R {
        TIM2ITR1_IREMAP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Ethernet PTP PPS remapping
    #[inline(always)]
    pub fn ptp_pps_remap(&self) -> PTP_PPS_REMAP_R {
        PTP_PPS_REMAP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAPR")
            .field("spi1_remap", &self.spi1_remap())
            .field("i2c1_remap", &self.i2c1_remap())
            .field("usart1_remap", &self.usart1_remap())
            .field("usart2_remap", &self.usart2_remap())
            .field("usart3_remap", &self.usart3_remap())
            .field("tim1_remap", &self.tim1_remap())
            .field("tim2_remap", &self.tim2_remap())
            .field("tim3_remap", &self.tim3_remap())
            .field("tim4_remap", &self.tim4_remap())
            .field("can1_remap", &self.can1_remap())
            .field("pd01_remap", &self.pd01_remap())
            .field("tim5ch4_iremap", &self.tim5ch4_iremap())
            .field("eth_remap", &self.eth_remap())
            .field("can2_remap", &self.can2_remap())
            .field("mii_rmii_sel", &self.mii_rmii_sel())
            .field("spi3_remap", &self.spi3_remap())
            .field("tim2itr1_iremap", &self.tim2itr1_iremap())
            .field("ptp_pps_remap", &self.ptp_pps_remap())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI1 remapping
    #[inline(always)]
    pub fn spi1_remap(&mut self) -> SPI1_REMAP_W<MAPRrs> {
        SPI1_REMAP_W::new(self, 0)
    }
    ///Bit 1 - I2C1 remapping
    #[inline(always)]
    pub fn i2c1_remap(&mut self) -> I2C1_REMAP_W<MAPRrs> {
        I2C1_REMAP_W::new(self, 1)
    }
    ///Bit 2 - USART1 remapping
    #[inline(always)]
    pub fn usart1_remap(&mut self) -> USART1_REMAP_W<MAPRrs> {
        USART1_REMAP_W::new(self, 2)
    }
    ///Bit 3 - USART2 remapping
    #[inline(always)]
    pub fn usart2_remap(&mut self) -> USART2_REMAP_W<MAPRrs> {
        USART2_REMAP_W::new(self, 3)
    }
    ///Bits 4:5 - USART3 remapping
    #[inline(always)]
    pub fn usart3_remap(&mut self) -> USART3_REMAP_W<MAPRrs> {
        USART3_REMAP_W::new(self, 4)
    }
    ///Bits 6:7 - TIM1 remapping
    #[inline(always)]
    pub fn tim1_remap(&mut self) -> TIM1_REMAP_W<MAPRrs> {
        TIM1_REMAP_W::new(self, 6)
    }
    ///Bits 8:9 - TIM2 remapping
    #[inline(always)]
    pub fn tim2_remap(&mut self) -> TIM2_REMAP_W<MAPRrs> {
        TIM2_REMAP_W::new(self, 8)
    }
    ///Bits 10:11 - TIM3 remapping
    #[inline(always)]
    pub fn tim3_remap(&mut self) -> TIM3_REMAP_W<MAPRrs> {
        TIM3_REMAP_W::new(self, 10)
    }
    ///Bit 12 - TIM4 remapping
    #[inline(always)]
    pub fn tim4_remap(&mut self) -> TIM4_REMAP_W<MAPRrs> {
        TIM4_REMAP_W::new(self, 12)
    }
    ///Bits 13:14 - CAN1 remapping
    #[inline(always)]
    pub fn can1_remap(&mut self) -> CAN1_REMAP_W<MAPRrs> {
        CAN1_REMAP_W::new(self, 13)
    }
    ///Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT
    #[inline(always)]
    pub fn pd01_remap(&mut self) -> PD01_REMAP_W<MAPRrs> {
        PD01_REMAP_W::new(self, 15)
    }
    ///Bit 16 - Set and cleared by software
    #[inline(always)]
    pub fn tim5ch4_iremap(&mut self) -> TIM5CH4_IREMAP_W<MAPRrs> {
        TIM5CH4_IREMAP_W::new(self, 16)
    }
    ///Bit 21 - Ethernet MAC I/O remapping
    #[inline(always)]
    pub fn eth_remap(&mut self) -> ETH_REMAP_W<MAPRrs> {
        ETH_REMAP_W::new(self, 21)
    }
    ///Bit 22 - CAN2 I/O remapping
    #[inline(always)]
    pub fn can2_remap(&mut self) -> CAN2_REMAP_W<MAPRrs> {
        CAN2_REMAP_W::new(self, 22)
    }
    ///Bit 23 - MII or RMII selection
    #[inline(always)]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<MAPRrs> {
        MII_RMII_SEL_W::new(self, 23)
    }
    ///Bits 24:26 - Serial wire JTAG configuration
    #[inline(always)]
    pub fn swj_cfg(&mut self) -> SWJ_CFG_W<MAPRrs> {
        SWJ_CFG_W::new(self, 24)
    }
    ///Bit 28 - SPI3/I2S3 remapping
    #[inline(always)]
    pub fn spi3_remap(&mut self) -> SPI3_REMAP_W<MAPRrs> {
        SPI3_REMAP_W::new(self, 28)
    }
    ///Bit 29 - TIM2 internal trigger 1 remapping
    #[inline(always)]
    pub fn tim2itr1_iremap(&mut self) -> TIM2ITR1_IREMAP_W<MAPRrs> {
        TIM2ITR1_IREMAP_W::new(self, 29)
    }
    ///Bit 30 - Ethernet PTP PPS remapping
    #[inline(always)]
    pub fn ptp_pps_remap(&mut self) -> PTP_PPS_REMAP_W<MAPRrs> {
        PTP_PPS_REMAP_W::new(self, 30)
    }
}
/**AF remap and debug I/O configuration register (AFIO_MAPR)

You can [`read`](crate::Reg::read) this register and get [`mapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#AFIO:MAPR)*/
pub struct MAPRrs;
impl crate::RegisterSpec for MAPRrs {
    type Ux = u32;
}
///`read()` method returns [`mapr::R`](R) reader structure
impl crate::Readable for MAPRrs {}
///`write(|w| ..)` method takes [`mapr::W`](W) writer structure
impl crate::Writable for MAPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAPR to value 0
impl crate::Resettable for MAPRrs {}
