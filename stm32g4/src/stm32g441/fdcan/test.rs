#[doc = "Register `TEST` reader"]
pub type R = crate::R<TESTrs>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TESTrs>;
#[doc = "Field `LBCK` reader - LBCK"]
pub type LBCK_R = crate::BitReader;
#[doc = "Field `LBCK` writer - LBCK"]
pub type LBCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - TX"]
pub type TX_R = crate::FieldReader;
#[doc = "Field `TX` writer - TX"]
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - RX"]
pub type RX_R = crate::BitReader;
#[doc = "Field `RX` writer - RX"]
pub type RX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - LBCK"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - TX"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - RX"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - LBCK"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LBCK_W<TESTrs> {
        LBCK_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - TX"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<TESTrs> {
        TX_W::new(self, 5)
    }
    #[doc = "Bit 7 - RX"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<TESTrs> {
        RX_W::new(self, 7)
    }
}
#[doc = "Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TESTrs;
impl crate::RegisterSpec for TESTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TESTrs {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TESTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TESTrs {
    const RESET_VALUE: u32 = 0;
}
