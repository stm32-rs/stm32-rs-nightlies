#[doc = "Register `DMACTxDLAR` reader"]
pub type R = crate::R<DMACTX_DLARrs>;
#[doc = "Register `DMACTxDLAR` writer"]
pub type W = crate::W<DMACTX_DLARrs>;
#[doc = "Field `TDESLA` reader - Start of Transmit List"]
pub type TDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `TDESLA` writer - Start of Transmit List"]
pub type TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    #[must_use]
    pub fn tdesla(&mut self) -> TDESLA_W<DMACTX_DLARrs> {
        TDESLA_W::new(self, 2)
    }
}
#[doc = "Channel Tx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_dlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_dlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTX_DLARrs;
impl crate::RegisterSpec for DMACTX_DLARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_dlar::R`](R) reader structure"]
impl crate::Readable for DMACTX_DLARrs {}
#[doc = "`write(|w| ..)` method takes [`dmactx_dlar::W`](W) writer structure"]
impl crate::Writable for DMACTX_DLARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTxDLAR to value 0"]
impl crate::Resettable for DMACTX_DLARrs {
    const RESET_VALUE: u32 = 0;
}
