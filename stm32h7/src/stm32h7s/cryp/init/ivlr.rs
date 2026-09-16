///Register `IVLR` reader
pub type R = crate::R<IVLRrs>;
///Register `IVLR` writer
pub type W = crate::W<IVLRrs>;
///Field `IVI` reader - Initialization vector bit x This bitfield stores the initialization vector bits \[127:96\] for AES chaining modes other than ECB. The value stored in CRYP_IVxR/LR registers is updated by hardware after each computation round (when applicable). Write to this register is ignored when CRYP is busy (BUSY bit set).
pub type IVI_R = crate::FieldReader<u32>;
///Field `IVI` writer - Initialization vector bit x This bitfield stores the initialization vector bits \[127:96\] for AES chaining modes other than ECB. The value stored in CRYP_IVxR/LR registers is updated by hardware after each computation round (when applicable). Write to this register is ignored when CRYP is busy (BUSY bit set).
pub type IVI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Initialization vector bit x This bitfield stores the initialization vector bits \[127:96\] for AES chaining modes other than ECB. The value stored in CRYP_IVxR/LR registers is updated by hardware after each computation round (when applicable). Write to this register is ignored when CRYP is busy (BUSY bit set).
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVLR").field("ivi", &self.ivi()).finish()
    }
}
impl W {
    ///Bits 0:31 - Initialization vector bit x This bitfield stores the initialization vector bits \[127:96\] for AES chaining modes other than ECB. The value stored in CRYP_IVxR/LR registers is updated by hardware after each computation round (when applicable). Write to this register is ignored when CRYP is busy (BUSY bit set).
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W<'_, IVLRrs> {
        IVI_W::new(self, 0)
    }
}
/**CRYP initialization vector register 0L

You can [`read`](crate::Reg::read) this register and get [`ivlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IVLRrs;
impl crate::RegisterSpec for IVLRrs {
    type Ux = u32;
}
///`read()` method returns [`ivlr::R`](R) reader structure
impl crate::Readable for IVLRrs {}
///`write(|w| ..)` method takes [`ivlr::W`](W) writer structure
impl crate::Writable for IVLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IVLR to value 0
impl crate::Resettable for IVLRrs {}
