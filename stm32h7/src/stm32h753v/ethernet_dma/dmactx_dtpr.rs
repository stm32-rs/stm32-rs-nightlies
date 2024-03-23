#[doc = "Register `DMACTxDTPR` reader"]
pub type R = crate::R<DMACTX_DTPRrs>;
#[doc = "Register `DMACTxDTPR` writer"]
pub type W = crate::W<DMACTX_DTPRrs>;
#[doc = "Field `TDT` reader - Transmit Descriptor Tail Pointer"]
pub type TDT_R = crate::FieldReader<u32>;
#[doc = "Field `TDT` writer - Transmit Descriptor Tail Pointer"]
pub type TDT_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn tdt(&mut self) -> TDT_W<DMACTX_DTPRrs> {
        TDT_W::new(self, 2)
    }
}
#[doc = "Channel Tx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_dtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_dtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTX_DTPRrs;
impl crate::RegisterSpec for DMACTX_DTPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_dtpr::R`](R) reader structure"]
impl crate::Readable for DMACTX_DTPRrs {}
#[doc = "`write(|w| ..)` method takes [`dmactx_dtpr::W`](W) writer structure"]
impl crate::Writable for DMACTX_DTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTxDTPR to value 0"]
impl crate::Resettable for DMACTX_DTPRrs {
    const RESET_VALUE: u32 = 0;
}
