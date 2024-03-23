#[doc = "Register `FDCAN_RXF1S` reader"]
pub type R = crate::R<FDCAN_RXF1Srs>;
#[doc = "Field `F1FL` reader - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
pub type F1FL_R = crate::FieldReader;
#[doc = "Field `F1GI` reader - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
pub type F1GI_R = crate::FieldReader;
#[doc = "Field `F1PI` reader - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
pub type F1PI_R = crate::FieldReader;
#[doc = "Field `F1F` reader - Rx FIFO 1 full"]
pub type F1F_R = crate::BitReader;
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\]
is reset, this bit is also reset."]
pub type RF1L_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\]
is reset, this bit is also reset."]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Rx FIFO 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1s::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RXF1Srs;
impl crate::RegisterSpec for FDCAN_RXF1Srs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf1s::R`](R) reader structure"]
impl crate::Readable for FDCAN_RXF1Srs {}
#[doc = "`reset()` method sets FDCAN_RXF1S to value 0"]
impl crate::Resettable for FDCAN_RXF1Srs {
    const RESET_VALUE: u32 = 0;
}
