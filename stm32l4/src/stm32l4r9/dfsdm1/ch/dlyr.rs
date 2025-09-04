///Register `DLYR` reader
pub type R = crate::R<DLYRrs>;
///Register `DLYR` writer
pub type W = crate::W<DLYRrs>;
///Field `PLSSKP` reader - PLSSKP
pub type PLSSKP_R = crate::FieldReader;
///Field `PLSSKP` writer - PLSSKP
pub type PLSSKP_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLYR")
            .field("plsskp", &self.plsskp())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&mut self) -> PLSSKP_W<DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
/**channel y delay register

You can [`read`](crate::Reg::read) this register and get [`dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DLYRrs;
impl crate::RegisterSpec for DLYRrs {
    type Ux = u32;
}
///`read()` method returns [`dlyr::R`](R) reader structure
impl crate::Readable for DLYRrs {}
///`write(|w| ..)` method takes [`dlyr::W`](W) writer structure
impl crate::Writable for DLYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLYR to value 0
impl crate::Resettable for DLYRrs {}
