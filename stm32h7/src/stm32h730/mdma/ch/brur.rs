///Register `BRUR` reader
pub type R = crate::R<BRURrs>;
///Register `BRUR` writer
pub type W = crate::W<BRURrs>;
///Field `SUV` reader - source adresse update value
pub type SUV_R = crate::FieldReader<u16>;
///Field `SUV` writer - source adresse update value
pub type SUV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DUV` reader - destination address update
pub type DUV_R = crate::FieldReader<u16>;
///Field `DUV` writer - destination address update
pub type DUV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - source adresse update value
    #[inline(always)]
    pub fn suv(&self) -> SUV_R {
        SUV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - destination address update
    #[inline(always)]
    pub fn duv(&self) -> DUV_R {
        DUV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRUR")
            .field("suv", &self.suv())
            .field("duv", &self.duv())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - source adresse update value
    #[inline(always)]
    pub fn suv(&mut self) -> SUV_W<'_, BRURrs> {
        SUV_W::new(self, 0)
    }
    ///Bits 16:31 - destination address update
    #[inline(always)]
    pub fn duv(&mut self) -> DUV_W<'_, BRURrs> {
        DUV_W::new(self, 16)
    }
}
/**MDMA channel x Block Repeat address Update register

You can [`read`](crate::Reg::read) this register and get [`brur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BRURrs;
impl crate::RegisterSpec for BRURrs {
    type Ux = u32;
}
///`read()` method returns [`brur::R`](R) reader structure
impl crate::Readable for BRURrs {}
///`write(|w| ..)` method takes [`brur::W`](W) writer structure
impl crate::Writable for BRURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRUR to value 0
impl crate::Resettable for BRURrs {}
