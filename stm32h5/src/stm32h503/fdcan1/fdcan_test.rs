///Register `FDCAN_TEST` reader
pub type R = crate::R<FDCAN_TESTrs>;
///Register `FDCAN_TEST` writer
pub type W = crate::W<FDCAN_TESTrs>;
///Field `LBCK` reader - Loop back mode
pub type LBCK_R = crate::BitReader;
///Field `LBCK` writer - Loop back mode
pub type LBCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX` reader - Control of transmit pin
pub type TX_R = crate::FieldReader;
///Field `TX` writer - Control of transmit pin
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RX` reader - Receive pin Monitors the actual value of pin FDCANx_RX
pub type RX_R = crate::BitReader;
impl R {
    ///Bit 4 - Loop back mode
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Control of transmit pin
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Receive pin Monitors the actual value of pin FDCANx_RX
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TEST")
            .field("lbck", &self.lbck())
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
impl W {
    ///Bit 4 - Loop back mode
    #[inline(always)]
    pub fn lbck(&mut self) -> LBCK_W<FDCAN_TESTrs> {
        LBCK_W::new(self, 4)
    }
    ///Bits 5:6 - Control of transmit pin
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W<FDCAN_TESTrs> {
        TX_W::new(self, 5)
    }
}
/**FDCAN test register

You can [`read`](crate::Reg::read) this register and get [`fdcan_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FDCAN1:FDCAN_TEST)*/
pub struct FDCAN_TESTrs;
impl crate::RegisterSpec for FDCAN_TESTrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_test::R`](R) reader structure
impl crate::Readable for FDCAN_TESTrs {}
///`write(|w| ..)` method takes [`fdcan_test::W`](W) writer structure
impl crate::Writable for FDCAN_TESTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TEST to value 0
impl crate::Resettable for FDCAN_TESTrs {
    const RESET_VALUE: u32 = 0;
}
