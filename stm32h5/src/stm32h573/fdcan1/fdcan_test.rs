#[doc = "Register `FDCAN_TEST` reader"]
pub type R = crate::R<FDCAN_TESTrs>;
#[doc = "Register `FDCAN_TEST` writer"]
pub type W = crate::W<FDCAN_TESTrs>;
#[doc = "Field `LBCK` reader - Loop back mode"]
pub type LBCK_R = crate::BitReader;
#[doc = "Field `LBCK` writer - Loop back mode"]
pub type LBCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - Control of transmit pin"]
pub type TX_R = crate::FieldReader;
#[doc = "Field `TX` writer - Control of transmit pin"]
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - Receive pin Monitors the actual value of pin FDCANx_RX"]
pub type RX_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Loop back mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of transmit pin"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive pin Monitors the actual value of pin FDCANx_RX"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop back mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LBCK_W<FDCAN_TESTrs> {
        LBCK_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of transmit pin"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<FDCAN_TESTrs> {
        TX_W::new(self, 5)
    }
}
#[doc = "FDCAN test register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TESTrs;
impl crate::RegisterSpec for FDCAN_TESTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_test::R`](R) reader structure"]
impl crate::Readable for FDCAN_TESTrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_test::W`](W) writer structure"]
impl crate::Writable for FDCAN_TESTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TEST to value 0"]
impl crate::Resettable for FDCAN_TESTrs {
    const RESET_VALUE: u32 = 0;
}
