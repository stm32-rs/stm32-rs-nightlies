#[doc = "Register `DMACTXDLAR` reader"]
pub type R = crate::R<DMACTXDLARrs>;
#[doc = "Register `DMACTXDLAR` writer"]
pub type W = crate::W<DMACTXDLARrs>;
#[doc = "Field `TDESLA` reader - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
pub type TDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `TDESLA` writer - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
pub type TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    #[must_use]
    pub fn tdesla(&mut self) -> TDESLA_W<DMACTXDLARrs> {
        TDESLA_W::new(self, 0)
    }
}
#[doc = "Channel Tx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactxdlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactxdlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTXDLARrs;
impl crate::RegisterSpec for DMACTXDLARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactxdlar::R`](R) reader structure"]
impl crate::Readable for DMACTXDLARrs {}
#[doc = "`write(|w| ..)` method takes [`dmactxdlar::W`](W) writer structure"]
impl crate::Writable for DMACTXDLARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTXDLAR to value 0"]
impl crate::Resettable for DMACTXDLARrs {
    const RESET_VALUE: u32 = 0;
}
