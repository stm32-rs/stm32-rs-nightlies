///Register `IVRR` reader
pub type R = crate::R<IVRRrs>;
///Register `IVRR` writer
pub type W = crate::W<IVRRrs>;
///Field `IVI` reader - Initialization vector bit x This bitfield stores the initialization vector bits \[95:64\] for AES chaining modes other than ECB. The value stored in CRYP_IVxR/LR registers is updated by hardware after each computation round (when applicable). Write to this register is ignored when CRYP is busy (BUSY bit set).
pub type IVI_R = crate::FieldReader<u32>;
///Field `IVI` writer - Initialization vector bit x This bitfield stores the initialization vector bits \[95:64\] for AES chaining modes other than ECB. The value stored in CRYP_IVxR/LR registers is updated by hardware after each computation round (when applicable). Write to this register is ignored when CRYP is busy (BUSY bit set).
pub type IVI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Initialization vector bit x This bitfield stores the initialization vector bits \[95:64\] for AES chaining modes other than ECB. The value stored in CRYP_IVxR/LR registers is updated by hardware after each computation round (when applicable). Write to this register is ignored when CRYP is busy (BUSY bit set).
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVRR").field("ivi", &self.ivi()).finish()
    }
}
impl W {
    ///Bits 0:31 - Initialization vector bit x This bitfield stores the initialization vector bits \[95:64\] for AES chaining modes other than ECB. The value stored in CRYP_IVxR/LR registers is updated by hardware after each computation round (when applicable). Write to this register is ignored when CRYP is busy (BUSY bit set).
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W<IVRRrs> {
        IVI_W::new(self, 0)
    }
}
/**CRYP initialization vector register 0R

You can [`read`](crate::Reg::read) this register and get [`ivrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IVRRrs;
impl crate::RegisterSpec for IVRRrs {
    type Ux = u32;
}
///`read()` method returns [`ivrr::R`](R) reader structure
impl crate::Readable for IVRRrs {}
///`write(|w| ..)` method takes [`ivrr::W`](W) writer structure
impl crate::Writable for IVRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IVRR to value 0
impl crate::Resettable for IVRRrs {}
