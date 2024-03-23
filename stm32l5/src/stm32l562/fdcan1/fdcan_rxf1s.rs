#[doc = "Register `FDCAN_RXF1S` reader"]
pub type R = crate::R<FDCAN_RXF1Srs>;
#[doc = "Register `FDCAN_RXF1S` writer"]
pub type W = crate::W<FDCAN_RXF1Srs>;
#[doc = "Field `F1FL` reader - Rx FIFO 1 Fill Level"]
pub type F1FL_R = crate::FieldReader;
#[doc = "Field `F1FL` writer - Rx FIFO 1 Fill Level"]
pub type F1FL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `F1GI` reader - Rx FIFO 1 Get Index"]
pub type F1GI_R = crate::FieldReader;
#[doc = "Field `F1PI` reader - Rx FIFO 1 Put Index"]
pub type F1PI_R = crate::FieldReader;
#[doc = "Field `F1F` reader - Rx FIFO 1 Full"]
pub type F1F_R = crate::BitReader;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub type RF1L_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn f1fl(&mut self) -> F1FL_W<FDCAN_RXF1Srs> {
        F1FL_W::new(self, 0)
    }
}
#[doc = "FDCAN Rx FIFO 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf1s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RXF1Srs;
impl crate::RegisterSpec for FDCAN_RXF1Srs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf1s::R`](R) reader structure"]
impl crate::Readable for FDCAN_RXF1Srs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf1s::W`](W) writer structure"]
impl crate::Writable for FDCAN_RXF1Srs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF1S to value 0"]
impl crate::Resettable for FDCAN_RXF1Srs {
    const RESET_VALUE: u32 = 0;
}
