///Register `TEST` reader
pub type R = crate::R<TESTrs>;
///Register `TEST` writer
pub type W = crate::W<TESTrs>;
///Field `LBCK` reader - LBCK
pub type LBCK_R = crate::BitReader;
///Field `LBCK` writer - LBCK
pub type LBCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX` reader - TX
pub type TX_R = crate::FieldReader;
///Field `TX` writer - TX
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RX` reader - RX
pub type RX_R = crate::BitReader;
impl R {
    ///Bit 4 - LBCK
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - TX
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - RX
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("lbck", &self.lbck())
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
impl W {
    ///Bit 4 - LBCK
    #[inline(always)]
    pub fn lbck(&mut self) -> LBCK_W<'_, TESTrs> {
        LBCK_W::new(self, 4)
    }
    ///Bits 5:6 - TX
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W<'_, TESTrs> {
        TX_W::new(self, 5)
    }
}
/**Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.

You can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:TEST)*/
pub struct TESTrs;
impl crate::RegisterSpec for TESTrs {
    type Ux = u32;
}
///`read()` method returns [`test::R`](R) reader structure
impl crate::Readable for TESTrs {}
///`write(|w| ..)` method takes [`test::W`](W) writer structure
impl crate::Writable for TESTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TEST to value 0
impl crate::Resettable for TESTrs {}
