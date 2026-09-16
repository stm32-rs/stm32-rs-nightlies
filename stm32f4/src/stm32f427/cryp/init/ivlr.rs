///Register `IVLR` reader
pub type R = crate::R<IVLRrs>;
///Register `IVLR` writer
pub type W = crate::W<IVLRrs>;
///Field `IV` reader - IV31
pub type IV_R = crate::FieldReader<u32>;
///Field `IV` writer - IV31
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IV31
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVLR").field("iv", &self.iv()).finish()
    }
}
impl W {
    ///Bits 0:31 - IV31
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<'_, IVLRrs> {
        IV_W::new(self, 0)
    }
}
/**initialization vector registers

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
