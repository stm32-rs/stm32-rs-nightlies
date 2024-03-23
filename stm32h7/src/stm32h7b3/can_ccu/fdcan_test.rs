#[doc = "Register `FDCAN_TEST` reader"]
pub type R = crate::R<FDCAN_TESTrs>;
#[doc = "Field `LBCK` reader - Loop Back mode"]
pub type LBCK_R = crate::BitReader;
#[doc = "Field `TX` reader - Loop Back mode"]
pub type TX_R = crate::FieldReader;
#[doc = "Field `RX` reader - Control of Transmit Pin"]
pub type RX_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Loop Back mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Loop Back mode"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "FDCAN Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_test::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TESTrs;
impl crate::RegisterSpec for FDCAN_TESTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_test::R`](R) reader structure"]
impl crate::Readable for FDCAN_TESTrs {}
#[doc = "`reset()` method sets FDCAN_TEST to value 0"]
impl crate::Resettable for FDCAN_TESTrs {
    const RESET_VALUE: u32 = 0;
}
