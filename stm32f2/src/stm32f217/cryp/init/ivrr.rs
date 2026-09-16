///Register `IVRR` reader
pub type R = crate::R<IVRRrs>;
///Register `IVRR` writer
pub type W = crate::W<IVRRrs>;
///Field `IV` reader - IV63
pub type IV_R = crate::FieldReader<u32>;
///Field `IV` writer - IV63
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IV63
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVRR").field("iv", &self.iv()).finish()
    }
}
impl W {
    ///Bits 0:31 - IV63
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<'_, IVRRrs> {
        IV_W::new(self, 0)
    }
}
/**initialization vector registers

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
