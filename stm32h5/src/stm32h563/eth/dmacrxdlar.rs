#[doc = "Register `DMACRXDLAR` reader"]
pub type R = crate::R<DMACRXDLARrs>;
#[doc = "Register `DMACRXDLAR` writer"]
pub type W = crate::W<DMACRXDLARrs>;
#[doc = "Field `RDESLA` reader - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
pub type RDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `RDESLA` writer - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
pub type RDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    #[must_use]
    pub fn rdesla(&mut self) -> RDESLA_W<DMACRXDLARrs> {
        RDESLA_W::new(self, 0)
    }
}
#[doc = "Channel Rx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrxdlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrxdlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRXDLARrs;
impl crate::RegisterSpec for DMACRXDLARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrxdlar::R`](R) reader structure"]
impl crate::Readable for DMACRXDLARrs {}
#[doc = "`write(|w| ..)` method takes [`dmacrxdlar::W`](W) writer structure"]
impl crate::Writable for DMACRXDLARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACRXDLAR to value 0"]
impl crate::Resettable for DMACRXDLARrs {
    const RESET_VALUE: u32 = 0;
}
