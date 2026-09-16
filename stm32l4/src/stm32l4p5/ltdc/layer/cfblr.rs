///Register `CFBLR` reader
pub type R = crate::R<CFBLRrs>;
///Register `CFBLR` writer
pub type W = crate::W<CFBLRrs>;
///Field `CFBLL` reader - Color Frame Buffer Line Length
pub type CFBLL_R = crate::FieldReader<u16>;
///Field `CFBLL` writer - Color Frame Buffer Line Length
pub type CFBLL_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
///Field `CFBP` reader - Color Frame Buffer Pitch in bytes
pub type CFBP_R = crate::FieldReader<u16>;
///Field `CFBP` writer - Color Frame Buffer Pitch in bytes
pub type CFBP_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
impl R {
    ///Bits 0:12 - Color Frame Buffer Line Length
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - Color Frame Buffer Pitch in bytes
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFBLR")
            .field("cfbll", &self.cfbll())
            .field("cfbp", &self.cfbp())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Color Frame Buffer Line Length
    #[inline(always)]
    pub fn cfbll(&mut self) -> CFBLL_W<'_, CFBLRrs> {
        CFBLL_W::new(self, 0)
    }
    ///Bits 16:28 - Color Frame Buffer Pitch in bytes
    #[inline(always)]
    pub fn cfbp(&mut self) -> CFBP_W<'_, CFBLRrs> {
        CFBP_W::new(self, 16)
    }
}
/**LTDC Layer Color Frame Buffer Length Register

You can [`read`](crate::Reg::read) this register and get [`cfblr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfblr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFBLRrs;
impl crate::RegisterSpec for CFBLRrs {
    type Ux = u32;
}
///`read()` method returns [`cfblr::R`](R) reader structure
impl crate::Readable for CFBLRrs {}
///`write(|w| ..)` method takes [`cfblr::W`](W) writer structure
impl crate::Writable for CFBLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFBLR to value 0
impl crate::Resettable for CFBLRrs {}
